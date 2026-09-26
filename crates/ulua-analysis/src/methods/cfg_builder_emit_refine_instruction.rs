//! Source: `Analysis/src/ControlFlowGraph.cpp:424-462` (hand-ported)
//! C++ `void CFGBuilder::emitRefineInstruction(Block* block, RefinementId refinement)`.

use crate::{
  methods::refinement_arena_type_proposition::refinement_arena_type_proposition,
  records::{
    block_registry::resolve_block_mut, cfg_builder::CfgBuilder,
    proposition_control_flow_graph::Proposition, refine::Refine, sym_def_registry::resolve_sym_def,
  },
  type_aliases::{
    block_id::BlockId, def_id_control_flow_graph::DefId,
    refinement_control_flow_graph::RefinementMember,
    refinement_id_control_flow_graph::RefinementId,
  },
};

impl CfgBuilder {
  /// # Safety
  /// 对应 cpp `CFGBuilder::emitRefineInstruction(Block*, RefinementId)`
  /// （ControlFlowGraph.cpp:424）。逐参数契约：
  /// - `block`：自 #17 续起为 `BlockId` u32 句柄，经 `block_registry` 解析写回
  ///   （见该模块契约）；调用期间该 Block 内存无其他并存可变借用。
  /// - `refinement`：类型 `RefinementId = Handle<Refinement>` 编码非空，指向
  ///   `allocator.refinement_arena`（TypedAllocator 分块分配、地址稳定）分配
  ///   的 `Refinement`；其标量字段与后代句柄（Conjunction 的 lhs/rhs、
  ///   Negation 的 refinement、Proposition.ptr 所指的 SymDef 句柄）随 arena
  ///   一并存活至本次构建结束。
  pub unsafe fn emit_refine_instruction(&mut self, block: BlockId, refinement: RefinementId) {
    // C++ `Luau::visit(overloaded{...}, *refinement)` over the 4 variant kinds.
    // Scalar fields are copied out of the borrow before mutating `self`.
    // `Handle::get` 物化共享只读借用（NonNull 契约收口在 arena_handle.rs），
    // 仅用于读判别值与拷贝字段，不再需要手写 unsafe 解引用。
    let variant_index = refinement.get().index();

    match variant_index {
      // Proposition (variant index 3)
      3 => {
        // index()==3 保证甄别恰命中 Proposition，只读借用仅用于复制 ptr
        // 字段，语句末结束。
        let prop: &Proposition = <Proposition as RefinementMember>::get_if(refinement.get())
          .expect("index()==3 甄别与 get_if 同构，恰命中 Proposition（见上 Safety 注）");
        // DefId refined = newDefinition(prop.ptr->sym);
        // prop.ptr 是 Proposition 构造时 new_definition 交回的 SymDef 句柄
        // （#17：`register_sym_def` 发放的 DefId），经注册表解析只读 clone
        // sym 字段，与 cpp `prop->ptr->sym` 同一步骤。
        let sym = resolve_sym_def(prop.ptr)
          .expect("Proposition.ptr 为构建期 register_sym_def 发放的存活句柄")
          .sym
          .clone();
        let refined: DefId = self.new_definition(sym.clone());
        // emit<Refine>(block, refined, refinement);
        self.emit::<Refine, _>(block, (refined, refinement));
        // block->setReachingDefinition(prop.ptr->sym, refined);
        // block 句柄经注册表写回（见 `block_registry` 模块契约）：refined 是刚
        // 分配的 SymDef 句柄。emit 调用返回后不存在对其余 Block 字段的存续
        // 借用，原位写 reaching_definitions 表无别名冲突。
        resolve_block_mut(block)
          .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
          .set_reaching_definition(sym, refined);
      }
      // Conjunction (variant index 0)
      0 => {
        use crate::records::conjunction_control_flow_graph::Conjunction;
        // index()==0 分支，命中 Conjunction 后按值拷出 lhs/rhs 两枚子句柄
        // （Handle 为 Copy），只读借用随之结束。
        let (lhs, rhs) = <Conjunction as RefinementMember>::get_if(refinement.get())
          .map(|conj| (conj.lhs, conj.rhs))
          .expect("index()==0 甄别与 get_if 同构，恰命中 Conjunction（见上 Safety 注）");
        // emitRefineInstruction(block, conj.lhs); emitRefineInstruction(block, conj.rhs);
        // 子句柄指先于父入 arena 的子节点，随父存活；递归为顺序独占。
        unsafe { self.emit_refine_instruction(block, lhs) };
        // rhs 与 lhs 同源——同一存活 Conjunction 节点的另一子句柄；此处
        // self 的可变借用随上一递归返回而重新可用。
        unsafe { self.emit_refine_instruction(block, rhs) };
      }
      // Negation (variant index 2)
      2 => {
        use crate::records::negation_control_flow_graph::Negation;
        // index()==2 分支，命中 Negation 后仅按值拷出其 refinement 子句柄。
        let inner = <Negation as RefinementMember>::get_if(refinement.get())
          .map(|neg| neg.refinement)
          .expect("index()==2 甄别与 get_if 同构，恰命中 Negation（见上 Safety 注）");
        // RefinementArena::negation pushes through And/Or via DeMorgan and cancels
        // double negation, so the only shape that reaches here is Negation(Proposition).
        // auto prop = neg.refinement->get_if<Proposition>();
        // neg.refinement 指向构造该 Negation 前先入 arena 的子节点，随父存活；
        // 非空由 `Handle` 类型编码（C++ NotNull 语义）。
        let prop = <Proposition as RefinementMember>::get_if(inner.get());
        // LUAU_ASSERT!(prop != nullptr);
        let prop =
          prop.expect("cpp LUAU_ASSERT(prop != nullptr)：Negation 的 refinement 恒为 Proposition");
        let (p_ptr, p_type, p_is_typeof, p_sense) =
          (prop.ptr, prop.r#type.clone(), prop.is_typeof, prop.sense);
        // emitRefineInstruction(block, allocator->refinementArena.typeProposition(
        //     prop->ptr, prop->type, prop->is_typeof, !prop->sense));
        let fresh = {
          // Safety: self.allocator 是 C++ NotNull<CfgAllocator> 成员的裸指针化，
          // visit 入口构造、非空且与本 builder 同生命周期；借用经本函数独占
          // 的 &mut self 派生，止于该表达式块——其下递归 emit 重新借用不重叠。
          let arena = unsafe { &mut (*self.allocator).refinement_arena };
          refinement_arena_type_proposition(arena, p_ptr, p_type, p_is_typeof, !p_sense)
        };
        // Safety: fresh 为刚经 TypedAllocator::allocate 返回的 arena 地址（已
        // 由 Handle 包装，恒非空），分块分配不搬运既有对象；block 前置条件
        // 随本函数参数原样成立。
        unsafe { self.emit_refine_instruction(block, fresh) };
      }
      // Disjunction (variant index 1)
      // CLI-205330 tracks the work needed to handle Disjunctions; no-op for now.
      _ => {}
    }
  }
}
