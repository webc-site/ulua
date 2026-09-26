use core::{cmp, mem};
use std::{cmp::max, vec::Vec};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE},
  records::{
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, small_vector::SmallVector,
  },
};

use crate::{
  enums::{
    bc_block_edge_kind::BcBlockEdgeKind,
    bc_block_flag::BcBlockFlag,
    bc_op_kind::{BcOpKind, BcOpKind::Proj},
  },
  records::{
    bc_block::BcBlock, bc_block_edge::BcBlockEdge, bc_call::BcCall, bc_call_fb::BcCallFB,
    bc_cmp_proto::BcCmpProto, bc_function::BcFunction, bc_get_table_ks::BcGetTableKS,
    bc_get_var_args::BcGetVarArgs, bc_inst_helper::BcInstHelper, bc_load_nil::BcLoadNil,
    bc_move::BcMove, bc_op::BcOp, bc_op_hash::BcOpHash, bc_proj::BcProj, bc_return::BcReturn,
    bc_set_list::BcSetList,
  },
  type_aliases::{bc_edges::BcEdges, reg::Reg},
};

// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。

#[derive(Debug)]
pub struct CallInliner<'a, 'c, 't> {
  pub(crate) caller: &'a mut BcFunction<'c>,
  pub(crate) target: &'a mut BcFunction<'t>,
  /// cpp `BcCallFB<VmConst> call`。这里只存被内联 CALLFB 的 `BcOp`：`BcCallFB`
  /// 自带 `&mut BcFunction`，与上面的 `caller` 借用了同一个图，二者不可能共存；
  /// 需要读写该指令时经 [`CallInliner::call_view`] 现取一个短生命期视图。
  pub(crate) call_op: BcOp,
  pub(crate) call_params: Vec<BcOp>,
  pub(crate) target_reg: Reg,
  /// cpp `CallInliner::callerFbVecSize`：调用方已有的 feedback 槽位数。`migrateInstructions`
  /// 用它把被内联函数体的 CALLFB 槽位整体后移，避免与 caller 自己的槽位撞号。
  pub(crate) caller_fb_vec_size: u32,

  pub(crate) caller_blocks_size_before_inline: u32,
  pub(crate) caller_inst_size_before_inline: u32,
  pub(crate) caller_vm_const_size_before_inline: u32,
  pub(crate) caller_proto_size_before_inline: u32,
  pub(crate) caller_up_val_size_before_inline: u8,

  pub(crate) return_ops: Vec<BcOp>,
  /// cpp `returnSites`：`migrateBlocks` 只登记 (调用者块, 目标 RETURN) 站点，
  /// 真正的替换延后到 `migrateBlockPhis` 之后，确保 `varArgMoves` 已填好。
  pub(crate) return_sites: Vec<(BcOp, BcOp)>,
  pub(crate) call_projections: DenseHashSet<BcOp, BcOpHash>,
  pub(crate) var_arg_moves: DenseHashMap<BcOp, Vec<BcOp>, BcOpHash>,
  /// 目标 phi → 调用者 phi 备忘录：递归映射前先查表，既保证同一目标 phi
  /// 恒等映射到同一调用者 phi（op 身份一致），也避免 loop-carried phi
  /// 自引用导致的无限递归（对齐 cpp `mappedPhis`）。
  pub(crate) mapped_phis: DenseHashMap<BcOp, BcOp, BcOpHash>,
}

impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// `kMaxInlinerCombinedStackSize = 250`（cpp `BytecodeCallInliner.h:18`）。
  pub(crate) const K_MAX_INLINER_COMBINED_STACK_SIZE: u8 = 250;
}

// ── abs-r139：并自 `methods/call_inliner_add_successor.rs` ──
// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。

impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `addSuccessor(BcRef<BcBlock> from, BcRef<BcBlock> to, Kind)`。
  ///
  /// cpp 靠 `BcRef::operator->` 从共享借用上取裸指针来写 `successors`/`predecessors`；
  /// Rust 侧可变访问必须由持有图的一方（`self.caller`）提供，因此入参改为纯 `BcOp` 句柄
  /// （`BcRef` 只保留只读语义）。
  pub fn add_successor(&mut self, from_op: BcOp, to_op: BcOp, kind: BcBlockEdgeKind) {
    LUAU_ASSERT!(
      kind != BcBlockEdgeKind::Fallthrough
        || (!self.has_edge(
          &self.caller.block(from_op).operator_deref().successors,
          BcBlockEdgeKind::Fallthrough
        ) && !self.has_edge(
          &self.caller.block(to_op).operator_deref().predecessors,
          BcBlockEdgeKind::Fallthrough
        ))
    );

    self
      .caller
      .block_op(from_op)
      .successors
      .push_back(BcBlockEdge {
        kind,
        target: to_op,
      });

    self
      .caller
      .block_op(to_op)
      .predecessors
      .push_back(BcBlockEdge {
        kind,
        target: from_op,
      });
  }
}

/// `allocateBlocks/allocateInstructions` 两枚同构入口的表单点（abs-r139）：
/// 记录 caller 现规模作平移基址，再把 arena `resize` 到 caller+target 规模。
macro_rules! allocate_arena_resize {
  ($name:ident, $base:ident, $field:ident) => {
    impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
      pub(crate) fn $name(&mut self) {
        self.$base = self.caller.$field.len() as u32;
        self.caller.$field.resize(
          self.$base as usize + self.target.$field.len(),
          Default::default(),
        );
      }
    }
  };
}

// ── abs-r139：并自 `methods/call_inliner_allocate_blocks.rs` ──
allocate_arena_resize!(allocate_blocks, caller_blocks_size_before_inline, blocks);

// ── abs-r139：并自 `methods/call_inliner_allocate_graph_entities_for_target.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't>
where
  't: 'c,
{
  pub(crate) fn allocate_graph_entities_for_target(&mut self) {
    self.allocate_blocks();
    self.allocate_instructions();
    self.allocate_vm_consts();
    self.allocate_protos();
    self.allocate_up_values();
  }
}

// ── abs-r139：并自 `methods/call_inliner_allocate_instructions.rs` ──
allocate_arena_resize!(
  allocate_instructions,
  caller_inst_size_before_inline,
  instructions
);

// ── abs-r139：并自 `methods/call_inliner_allocate_protos.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `allocateProtos()`（BytecodeCallInliner.h:197-203）：把 `target.protos`
  /// 的**真实 fid** 逐个追加到 caller 尾部。
  ///
  /// 序列化期按 `caller.protos` 的下标取值挂子函数（`add_child_function`），
  /// 补零会让内联体里的 NEWCLOSURE/DUPCLOSURE 指错 child 且无任何报错。
  pub(crate) fn allocate_protos(&mut self) {
    // caller_proto_size_before_inline 必须先于任何追加取克隆前长度，
    // map_proto_op 依赖它把 target proto 下标平移进 caller 区间。
    self.caller_proto_size_before_inline = self.caller.protos.len() as u32;
    // caller/target 是 self 的不同字段，拆分借用直连即可，无需先整表 clone
    // （bytecode_builder_finalize 同款先例）；元素是 u32，纯拷贝。
    let extra_len = self.target.protos.len();
    self.caller.protos.reserve(extra_len);
    self.caller.protos.extend_from_slice(&self.target.protos);
  }
}

// ── abs-r139：并自 `methods/call_inliner_allocate_up_values.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn allocate_up_values(&mut self) {
    self.caller_up_val_size_before_inline = self.caller.nups;
    self.caller.nups += self.target.nups;
  }
}

// ── abs-r139：并自 `methods/call_inliner_allocate_vm_consts.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't>
where
  't: 'c,
{
  pub(crate) fn allocate_vm_consts(&mut self) {
    self.caller_vm_const_size_before_inline = self.caller.constants.len() as u32;
    let reserve_size = self.caller_vm_const_size_before_inline + self.target.constants.len() as u32;
    self.caller.constants.reserve(reserve_size as usize);
    self
      .caller
      .constants
      .extend_from_slice(&self.target.constants);
  }
}

// ── abs-r139：并自 `methods/call_inliner_append_cmp_proto.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `appendCmpProto(BcRef<BcBlock>& prevBlock, BcOp targetOp, uint32_t targetProtoId)`：
  /// 在 `prevBlock` 末尾追加一条 CMPProto 作为回退分支，并与调用块互连成边。
  /// 入参改为块句柄 `BcOp`（`BcRef` 已只读）。
  pub(crate) fn append_cmp_proto(
    &mut self,
    prev_block_op: BcOp,
    target_op: BcOp,
    target_proto_id: u32,
  ) {
    let call_block = self.call_block_op();
    {
      let mut cmp_proto = BcCmpProto::create(self.caller);
      cmp_proto.set_closure(target_op);
      cmp_proto.set_proto_id(target_proto_id);
      cmp_proto.set_fallback(call_block);
      cmp_proto.append_to(prev_block_op);
    }
    self.add_successor(prev_block_op, call_block, BcBlockEdgeKind::Branch);
  }
}

// ── abs-r139：并自 `methods/call_inliner_call_block_op.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `call->block`：被内联 CALLFB 指令所在块。
  pub(crate) fn call_block_op(&mut self) -> BcOp {
    self.caller.inst_op(self.call_op).block
  }
}

// ── abs-r139：并自 `methods/call_inliner_call_inliner.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// `caller_fb_vec_size` 对齐 cpp 同名参数：调用方在 CALLFB 之前已分配的 feedback 槽位数，
  /// 由 `BytecodeBuilder::fb_slots.len()`（或编译器侧的槽位计数）提供。
  pub fn new(
    caller: &'a mut BcFunction<'c>,
    target: &'a mut BcFunction<'t>,
    call_op: BcOp,
    caller_fb_vec_size: u32,
  ) -> Self {
    // cpp `BcCallFB<VmConst> call{caller, call.op()}`：CALLFB 视图只在构造期取实参与
    // 结果寄存器，取完即释放对图的可变借用（图的所有权仍归 `caller` 字段）。
    let call = BcCallFB::from(caller, call_op);
    let call_params = call.params();
    let target_reg = call.base.get_out_reg();

    CallInliner {
      caller,
      target,
      call_op,
      call_params,
      target_reg,
      caller_fb_vec_size,
      caller_blocks_size_before_inline: 0,
      caller_inst_size_before_inline: 0,
      caller_vm_const_size_before_inline: 0,
      caller_proto_size_before_inline: 0,
      caller_up_val_size_before_inline: 0,
      return_ops: Vec::new(),
      return_sites: Vec::new(),
      call_projections: DenseHashSet::new(BcOp::new()),
      var_arg_moves: DenseHashMap::new(BcOp::new()),
      mapped_phis: DenseHashMap::new(BcOp::new()),
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_call_view.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `CallInliner::call`（`BcCallFB<VmConst>`）。被内联的 CALLFB 视图每次现取：
  /// `BcCallFB` 持有图的唯一可变借用，不能与 `self.caller` 长期共存，因此在语句
  /// 结束时即释放借用，后续 `self.caller` 的读写不受影响。
  pub(crate) fn call_view(&mut self) -> BcCallFB<'_, 'c> {
    BcCallFB::from(self.caller, self.call_op)
  }
}

// ── abs-r139：并自 `methods/call_inliner_fill_under_call_arguments.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn fill_under_call_arguments(&mut self) {
    if self.call_params.len() as u8 >= self.target.numparams {
      return;
    }

    let inline_entry_block = self.map_block_op(self.target.entry_block);
    let call_param_size = self.call_params.len() as u8;
    self
      .call_params
      .resize(self.target.numparams as usize, Default::default());

    for param in (call_param_size..self.target.numparams).rev() {
      let mut load_nil = BcLoadNil::create(self.caller);
      load_nil.set_out_reg(self.target_reg + 1 + param);
      load_nil.prepend_to(inline_entry_block);
      self.call_params[param as usize] = load_nil.op();
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_find_target_call_projections.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `findTargetCallProjections()`：为直接引用被内联 CALLFB 的投影建聚合 phi。
  pub(crate) fn find_target_call_projections(&mut self) {
    // i 是投影在 caller arena 中的序号，直接被编进 `BcOp::Proj` 句柄（数据语义，非游标）；
    // 循环体内 `add_phi`/`phi_op` 要可变借用 caller，无法同时持有 projections 的迭代借用。
    // 范围上界只在进入时求值一次，本循环新加的投影不会被访问。
    for i in 0..self.caller.projections.len() {
      let proj = self.caller.projections[i];
      if proj.op == self.call_op {
        let proj_op = BcOp::with(Proj, i as u32);
        if self.call_projections.contains(&proj_op) {
          continue;
        }
        if (proj.index as usize) >= self.return_ops.len() {
          self.return_ops.resize(proj.index as usize + 1, BcOp::new());
        }
        let phi_op = self.caller.add_phi();
        self.caller.phi_op(phi_op).ops.push_back(proj_op);
        self.call_projections.insert(proj_op);
        self.return_ops[proj.index as usize] = phi_op;
      }
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_get_var_arg_param.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `getVarArgParam`（BytecodeCallInliner.h:334-338）：`LUAU_ASSERT` + `[]`。
  ///
  /// debug 下由断言兜底；release 下断言是 no-op，旧实现的 `unwrap()` 就是 panic
  /// 路径。仿 `migrateInstructions`（b1 S3 裁定）收敛为「断言 + 非 panic 读法」：
  /// 缺键/越界时回退 `BcOp::new()`（kind None，对齐 cpp 对缺失 map 键取默认
  /// `BcOp{}` 的可观察形态），不再从 panic 出口离开内联。该表仅由本次内联的
  /// `replaceGetVarArg` 填充，当前只有内部自产图可达，收口只消除 panic 面。
  pub(crate) fn get_var_arg_param(&self, get_var_args_op: BcOp, idx: u32) -> BcOp {
    let moves = self.var_arg_moves.get(&get_var_args_op);
    LUAU_ASSERT!(moves.is_some() && idx < moves.map_or(0, |m| m.len() as u32));
    moves
      .and_then(|m| m.get(idx as usize))
      .copied()
      .unwrap_or(BcOp::new())
  }
}

// ── abs-r139：并自 `methods/call_inliner_has_edge.rs`（消费方仅本模块
// `add_successor` 的断言，原「镜像定形保持 pub」的账目不成立，收敛为 pub(crate)）──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn has_edge(&self, edges: &BcEdges, kind: BcBlockEdgeKind) -> bool {
    edges.iter().any(|e| e.kind == kind)
  }
}

// ── abs-r139：并自 `methods/call_inliner_inline_target.rs` ──
/// 把 `edges` 中首条 fallthrough 边重定向到 `target`；无则追加。
fn upsert_fallthrough(edges: &mut BcEdges, target: BcOp) {
  for edge in edges.iter_mut() {
    if edge.kind == BcBlockEdgeKind::Fallthrough {
      edge.target = target;
      return;
    }
  }
  edges.push_back(BcBlockEdge {
    kind: BcBlockEdgeKind::Fallthrough,
    target,
  });
}

impl<'a, 'c, 't> CallInliner<'a, 'c, 't>
where
  't: 'c,
{
  /// cpp `inlineTarget(uint32_t targetProtoId)`：把 `target` 图内联进 `caller`。
  ///
  /// 全程只在图之间传 `BcOp` 句柄；任何一次读写都经 `self.caller` / `self.target`
  /// 现取现用，不再持有跨可变操作的 `BcRef` 视图（旧实现在 `&Vec<T>` 与
  /// `&mut BcFunction` 之间互造裸指针，属 Stacked Borrows UB）。
  pub(crate) fn inline_target(&mut self, target_proto_id: u32) -> bool {
    // cpp `inlineTarget` 入口 `LUAU_ASSERT(validate())`（BytecodeCallInliner.h:680）
    LUAU_ASSERT!(self.validate());

    let mut new_max_stack_size: u32 =
      u32::from(self.caller.maxstacksize) + u32::from(self.target.maxstacksize);

    if self.target.is_vararg {
      // cpp `newMaxStackSize += std::max(uint8_t(callParams.size()), target.numparams)`：
      // 实参少于形参时，缺口由 fillUnderCallArguments 用 LOADNIL 补齐，
      // 栈预算必须按 numparams 起算，否则映射后的寄存器会越过 maxstacksize。
      new_max_stack_size = new_max_stack_size.wrapping_add(u32::from(cmp::max(
        self.call_params.len() as u8,
        self.target.numparams,
      )));
    }

    // 上限常量与 cpp `kMaxInlinerCombinedStackSize` 同源，直接用 `CallInliner` 的关联常量。
    if new_max_stack_size >= u32::from(Self::K_MAX_INLINER_COMBINED_STACK_SIZE) {
      return false;
    }

    let mut call = self.call_view();
    if call.param_count() < 0 || call.return_count() < 0 {
      return false;
    }

    // inlining of upvalues is not supported yet
    if self.target.nups > 0 {
      return false;
    }

    self.caller.maxstacksize = new_max_stack_size as u8;

    let (prev_block_op, next_block_op) = self.split_block_on_op(self.call_op);

    let mut target_op: BcOp = self.call_view().target();

    // cpp：prevBlock 末尾若是 NAMECALL，先就地换成 MOVE + GETTABLEKS
    if let Some(last_op) = self.caller.block_op(prev_block_op).ops.back().copied()
      && self.caller.inst_op(last_op).op == LuauOpcode::LOP_NAMECALL
    {
      target_op = self.replace_namecall(last_op, prev_block_op);
    }

    self.append_cmp_proto(prev_block_op, target_op, target_proto_id);

    // Seal FB slot of inlined call：内联成功后回退路径上的 CALLFB 不再需要 feedback 槽位。
    self.call_view().set_fb_slot(-1);

    self.allocate_graph_entities_for_target();

    self.fill_under_call_arguments();

    self.find_target_call_projections();

    if !self.migrate_blocks(next_block_op) {
      return false;
    }

    let caller_inlined_entry_op = self.map_block_op(self.target.entry_block);

    // Remove prevBlock fallthrough to call block from its predecessors
    let call_block = self.call_block_op();
    {
      let insn_preds: &mut BcEdges = &mut self.caller.block_op(call_block).predecessors;
      *insn_preds = insn_preds
        .iter()
        .filter(|p| !(p.kind == BcBlockEdgeKind::Fallthrough && p.target == prev_block_op))
        .copied()
        .collect();
    }

    upsert_fallthrough(
      &mut self.caller.block_op(prev_block_op).successors,
      caller_inlined_entry_op,
    );
    upsert_fallthrough(
      &mut self.caller.block_op(caller_inlined_entry_op).predecessors,
      prev_block_op,
    );

    self.migrate_block_phis();

    // cpp `for (auto& [callerBlockOp, targetReturnOp] : returnSites) replaceReturn(...)`：
    // 延后到此处，保证 replaceGetVarArg 已把 varArgMoves 填好，
    // 返回 phi 里的变参投影才能解析到对应的 MOVE/LOADNIL。
    for (caller_block_op, target_return_op) in mem::take(&mut self.return_sites) {
      if !self.replace_return(next_block_op, caller_block_op, target_return_op) {
        return false;
      }
    }

    self.migrate_instructions();

    self.replace_call_usages_with_return_phis();

    // 多返回值聚合 phi 锚定到汇合块（cpp inline 尾部）：惰性过滤迭代，省去中间 Vec
    for ret_op in self
      .return_ops
      .iter()
      .copied()
      .filter(|op| op.kind == BcOpKind::Phi)
    {
      self.caller.block_op(next_block_op).phis.push(ret_op);
    }

    // cpp 出口 `LUAU_ASSERT(validate())`（h:757）：validateCfg && validatePhis
    LUAU_ASSERT!(self.validate());

    true
  }
}

// ── abs-r139：并自 `methods/call_inliner_make_fixed_consumer.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `makeFixedConsumer(BcFunction&, BcRef<BcInst>&)`：把变参消费指令
  /// （SETLIST/RETURN/CALLFB/CALL）改写为定长形态——变参尾巴被替换掉后，
  /// 输入个数即新的固定计数。
  ///
  /// 入参由 `&mut BcRef<BcInst>` 改为 `BcOp`：`BcRef` 现在只承担只读视图，
  /// 可变访问一律经持有图的 `self.caller` 现场构造 helper。
  pub(crate) fn make_fixed_consumer(&mut self, inst_op: BcOp) {
    match self.caller.inst_op(inst_op).op {
      LuauOpcode::LOP_SETLIST => {
        let mut set_list = BcSetList::from(self.caller, inst_op);
        let count = set_list.params().len() as u32;
        // cpp `makeFixedConsumer`：变参尾巴被替换成 0 个输入时，SETLIST 已无参数可写，
        // 必须整条摘掉（detach），只留下前面的 NEWTABLE。
        if count == 0 {
          set_list.base.detach();
        } else {
          set_list.set_count(count);
        }
      }
      LuauOpcode::LOP_RETURN => {
        let mut ret = BcReturn::from(self.caller, inst_op);
        let count = ret.values().len() as u32;
        ret.set_return_count(count);
      }
      LuauOpcode::LOP_CALLFB => {
        let mut call_fb = BcCallFB::from(self.caller, inst_op);
        let count = call_fb.params().len() as u32;
        call_fb.set_param_count(count);
      }
      LuauOpcode::LOP_CALL => {
        let mut call = BcCall::from(self.caller, inst_op);
        let count = call.params().len() as u32;
        call.set_param_count(count);
      }
      _ => {
        LUAU_UNREACHABLE!();
      }
    }
  }
}

/// cpp `BytecodeCallInliner.h` 的 arena 下标平移单源：
/// `(kind, 调用方内联前 arena 规模 + index)`，kind 断言随参数保留在每个入口。
fn map_arena_op(kind: BcOpKind, base_before_inline: u32, target: BcOp) -> BcOp {
  LUAU_ASSERT!(target.kind == kind);
  BcOp::with(kind, base_before_inline + target.index)
}

/// `mapBlockOp/mapInstOp/mapProtoOp/mapUpValueOp/mapVmConstOp` 五个同构入口的
/// 表单点（abs-r139：原五枚碎片文件与逐条委托 impl 坍缩于此）。基址以字段名
/// 传入、在方法体内取 `self.$field`（宏卫生不允许调用点表达式携带 `self`），
/// `u8` 域字段用 `from` 前缀走 `u32::from`。
macro_rules! map_arena_entry {
  ($(#[$attr:meta])* $name:ident, $kind:path, $field:ident) => {
    impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
      $(#[$attr])*
      pub(crate) fn $name(&self, target: BcOp) -> BcOp {
        map_arena_op($kind, self.$field, target)
      }
    }
  };
  ($(#[$attr:meta])* $name:ident, $kind:path, from $field:ident) => {
    impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
      $(#[$attr])*
      pub(crate) fn $name(&self, target: BcOp) -> BcOp {
        map_arena_op($kind, u32::from(self.$field), target)
      }
    }
  };
}

map_arena_entry!(
  /// cpp `mapBlockOp`：target 块号平移到 caller 块 arena。
  map_block_op,
  BcOpKind::Block,
  caller_blocks_size_before_inline
);
map_arena_entry!(
  /// cpp `mapInstOp`：target 指令号平移到 caller 指令 arena。
  map_inst_op,
  BcOpKind::Inst,
  caller_inst_size_before_inline
);
map_arena_entry!(
  /// cpp `mapProtoOp`：target 子函数号平移到 caller proto arena。
  map_proto_op,
  BcOpKind::VmProto,
  caller_proto_size_before_inline
);
map_arena_entry!(
  /// cpp `mapUpValueOp`：target 上值号平移到 caller 上值 arena。
  map_up_value_op, BcOpKind::VmUpvalue, from caller_up_val_size_before_inline
);
map_arena_entry!(
  /// cpp `mapVmConstOp`：target 常量号平移到 caller 常量 arena。
  map_vm_const_op,
  BcOpKind::VmConst,
  caller_vm_const_size_before_inline
);

// ── abs-r139：并自 `methods/call_inliner_map_to_caller_op.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn map_to_caller_op(&mut self, target_op: BcOp) -> BcOp {
    match target_op.kind {
      BcOpKind::Inst => self.map_inst_op(target_op),
      BcOpKind::Block => self.map_block_op(target_op),
      BcOpKind::Imm => {
        let imm = *self.target.imm_op(target_op);
        self.caller.add_imm_value(imm)
      }
      BcOpKind::Phi => {
        // cpp `mappedPhis`：递归前先备忘录查表——loop-carried phi 会（传递地）
        // 引用自身，不查表则无限递归；同时保证同一目标 phi 全程映射到同一
        // 调用者 phi，维持 op 身份一致
        if let Some(&mapped) = self.mapped_phis.find(&target_op) {
          return mapped;
        }

        let phi_op = self.caller.add_phi();
        self.mapped_phis.insert(target_op, phi_op);
        // 快照目标 phi 操作数，避免映射过程中反复重借用
        let target_phi_ops: Vec<BcOp> = self
          .target
          .phi(target_op)
          .operator_deref()
          .ops
          .as_slice()
          .to_vec();

        for target_phi_op in target_phi_ops {
          let mapped = self.map_to_caller_op(target_phi_op);
          self.caller.phi_op(phi_op).ops.push_back(mapped);
        }
        phi_op
      }
      BcOpKind::Proj => {
        let proj_ref = self.target.proj(target_op);
        let proj = *proj_ref.operator_deref();
        if self.target.is_vararg {
          let inst_ref = self.target.inst(proj.op);
          let inst = inst_ref.operator_deref();
          if inst.op == LuauOpcode::LOP_GETVARARGS {
            // BcGetVarArgs is a helper struct wrapping a BcInst reference.
            // Since we don't have the full BcGetVarArgs definition here,
            // we use the underlying inst and proj.index directly.
            LUAU_ASSERT!(inst.ops.len() > 1);
            return self.get_var_arg_param(proj.op, proj.index);
          }
        }
        let mapped_op = self.map_to_caller_op(proj.op);
        self.caller.add_proj(mapped_op, proj.index)
      }
      BcOpKind::VmReg => {
        if target_op.index < self.target.numparams as u32 {
          LUAU_ASSERT!(target_op.index < self.call_params.len() as u32);
          self.call_params[target_op.index as usize]
        } else {
          BcOp::with(
            BcOpKind::VmReg,
            self.map_to_caller_reg(target_op.index as Reg) as u32,
          )
        }
      }
      BcOpKind::VmConst => self.map_vm_const_op(target_op),
      BcOpKind::VmProto => self.map_proto_op(target_op),
      BcOpKind::VmUpvalue => self.map_up_value_op(target_op),
      _ => target_op,
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_map_to_caller_reg.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn map_to_caller_reg(&self, reg: Reg) -> Reg {
    let vararg_offset = if self.target.is_vararg {
      self.call_params.len() as u8
    } else {
      0
    };

    self.target_reg + 1 + vararg_offset + reg
  }
}

// ── abs-r139：并自 `methods/call_inliner_migrate_block_phis.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `CallInliner::migrateBlockPhis`：把 target 各块锚定的 phi 映射后挂到 caller 对应块。
  /// 独立于 migrateBlocks：phi 可能引用 GETVARARGS 投影，需等 MOVE 在 migrateBlocks 中物化后再映射。
  pub(crate) fn migrate_block_phis(&mut self) {
    // 本块 phi 句柄先快照到栈上缓冲（≤4 条不落堆，容量跨块复用）：`map_to_caller_op`
    // 走 &mut self，无法同时持有 target phis 的切片借用；快照后即可用迭代器遍历，
    // 消掉手工 k 下标与逐次越界检查。
    let mut phi_buf: SmallVector<BcOp, 4> = SmallVector::new();
    for i in 0..self.target.blocks.len() {
      // i 是块序号：caller 侧槽位 = 内联前块数 + i，属映射数据而非游标，故外层保留下标。
      let caller_block_idx = self.caller_blocks_size_before_inline as usize + i;
      phi_buf.clear();
      phi_buf.extend(self.target.blocks[i].phis.iter().copied());
      for &phi_op in &phi_buf {
        let caller_phi_op = self.map_to_caller_op(phi_op);
        self.caller.blocks[caller_block_idx]
          .phis
          .push(caller_phi_op);
      }
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_migrate_blocks.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `migrateBlocks(BcRef<BcBlock>& nextBlock)`：把目标图的块/边/指令搬进调用者
  /// 预留好的槽位。`next_block_op` 为调用者侧紧跟内联区后的块句柄。
  pub(crate) fn migrate_blocks(&mut self, next_block_op: BcOp) -> bool {
    let call_block = self.call_block_op();
    let insn_block_sort_key = self.caller.block_op(call_block).sortkey;
    let insn_block_chain_key = self.caller.block_op(call_block).chainkey;
    let blocks_base = self.caller_blocks_size_before_inline;
    let mut max_chain_key = 0;
    // 块指令句柄快照缓冲：循环体每步都要回调 &mut self（replace_get_var_arg 等），
    // 无法持有 target ops 的借用；换用缓冲后可按迭代器遍历，容量跨块复用。
    let mut ops_buf: Vec<BcOp> = Vec::new();

    for i in 0..self.target.blocks.len() {
      // i 是块序号：caller 侧槽位与块句柄都是 `blocks_base + i`，还要和
      // `target.exit_block.index` 比较——是被搬进图句柄的映射数据，不是游标，故外层保留下标。
      let target_block_sortkey = self.target.blocks[i].sortkey;
      let caller_block_idx = (blocks_base + i as u32) as usize;
      let caller_block_op = BcOp::with(BcOpKind::Block, blocks_base + i as u32);

      if i as u32 == self.target.exit_block.index {
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.sortkey = BcBlock::K_BLOCK_NO_START_PC;
        caller_block.flags |= BcBlockFlag::Dead;
        continue;
      }

      {
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.sortkey = insn_block_sort_key;
        caller_block.chainkey = insn_block_chain_key + target_block_sortkey;
        max_chain_key = max(caller_block.chainkey, max_chain_key);
      }

      // Migrate successors：块号平移是纯算术（走 mapArenaOp 单源，不回调 &mut self），
      // 故直接迭代 target 边表；exit block 的边按 cpp 语义整条丢弃。
      for e in self.target.blocks[i].successors.iter().copied() {
        if e.target == self.target.exit_block {
          continue;
        }
        let mapped_target = map_arena_op(BcOpKind::Block, blocks_base, e.target);
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.successors.push_back(BcBlockEdge {
          kind: e.kind,
          target: mapped_target,
        });
      }

      // Migrate predecessors：同上，直接迭代边表
      for e in self.target.blocks[i].predecessors.iter().copied() {
        let mapped_target = map_arena_op(BcOpKind::Block, blocks_base, e.target);
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.predecessors.push_back(BcBlockEdge {
          kind: e.kind,
          target: mapped_target,
        });
      }

      // Migrate instructions
      ops_buf.clear();
      ops_buf.extend(self.target.blocks[i].ops.iter().copied());
      for &op in &ops_buf {
        let inst_op_code = self.target.inst_op(op).op;
        if inst_op_code == LuauOpcode::LOP_GETVARARGS {
          self.replace_get_var_arg(caller_block_op, op);
        } else if inst_op_code == LuauOpcode::LOP_RETURN {
          // cpp：此处只校验变长返回并登记站点，替换延后到 migrateBlockPhis 之后
          let return_count = BcReturn::from(self.target, op).return_count();
          if return_count < 0 {
            return false;
          }
          self.return_sites.push((caller_block_op, op));
        } else if inst_op_code != LuauOpcode::LOP_PREPVARARGS {
          // cpp `else if (inst.op != LOP_PREPVARARGS)`（BytecodeCallInliner.h:382）：
          // 被内联函数的 PREPVARARGS 携带的是 *被内联函数* 的 numparams，搬进调用方
          // 就成了非法指令（`validateInstructions` 断言 `LUAU_INSN_A == func.numparams`
          // 会失败，VM 侧也会按错误的实参个数重排 L->top），必须整条丢弃；
          // 入口块此刻可能已被 `fill_under_call_arguments` 前插了补位 LOADNIL，
          // 只能在源头丢弃，不能靠「摘掉块首指令」事后补救。
          let caller_inst_op = self.map_inst_op(op);
          self.caller.blocks[caller_block_idx].append_instruction(caller_inst_op);
          self.caller.inst_op(caller_inst_op).block = caller_block_op;
        }
      }
    }

    self.caller.block_op(call_block).chainkey = max_chain_key + 1;
    self.caller.block_op(next_block_op).chainkey = max_chain_key + 2;

    true
  }
}

// ── abs-r139：并自 `methods/call_inliner_migrate_instructions.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `migrateInstructions()`：把被内联函数的指令逐条落进调用方预留好的槽位。
  ///
  /// 这条路径跑在内联热循环上，所以只读 `BcInst` 的 `Copy` 字段、操作数句柄单遍拷进
  /// 复用的 `SmallVector` 缓冲，不再 `clone()` 整条指令（含两个 `SmallVector`）或 `collect` 成 `Vec`。
  pub(crate) fn migrate_instructions(&mut self) {
    // cpp `callerInst->line = call->line`：被内联的整段目标代码统一挂到 CALLFB 所在行，
    // 否则新分配的 caller 槽位停留在默认 line 0，序列化后错误行号/coverage/debugger 全部落到第 0 行。
    let call_line = self.caller.inst_op(self.call_op).line;
    let target_insn_count = self.target.instructions.len() as u32;
    // 操作数句柄快照缓冲：`map_to_caller_op` 走 &mut self，无法同时持有 target ops 的借用。
    // 先单遍拷出 Copy 句柄，两处操作数遍历即可改用迭代器，消掉 k 下标与逐次
    // `instructions[tidx].ops[k]` 的重复越界检查；容量跨指令复用。
    let mut ops_buf: SmallVector<BcOp, 4> = SmallVector::new();

    for i in 0..target_insn_count {
      // i 是指令序号：target/caller 两侧句柄（`BcOp::Inst`）与各自 arena 下标都由它算出，
      // 且要同时随机读写两条槽位，属图句柄数据而非游标，故保留下标。
      let target_insn_op = BcOp::with(BcOpKind::Inst, i);
      let caller_insn_op = BcOp::with(BcOpKind::Inst, self.caller_inst_size_before_inline + i);
      let tidx = target_insn_op.index as usize;
      let cidx = caller_insn_op.index as usize;

      let op = self.target.instructions[tidx].op;

      // cpp `if (op == RETURN || op == GETVARARGS || op == PREPVARARGS) continue`：
      // 早退必须在任何拷贝/下钻之前；PREPVARARGS 携带被内联函数的 numparams，
      // 绝不能被复制进调用方的指令槽。
      if op == LuauOpcode::LOP_RETURN
        || op == LuauOpcode::LOP_GETVARARGS
        || op == LuauOpcode::LOP_PREPVARARGS
      {
        continue;
      }

      let block = self.target.instructions[tidx].block;
      LUAU_ASSERT!(block.kind == BcOpKind::Block);
      let mapped_block = BcOp::with(
        BcOpKind::Block,
        self.caller_blocks_size_before_inline + block.index,
      );
      self.caller.instructions[cidx].op = op;
      self.caller.instructions[cidx].block = mapped_block;
      self.caller.instructions[cidx].line = call_line;

      ops_buf.clear();
      ops_buf.extend(self.target.instructions[tidx].ops.iter().copied());

      // cpp `isMultiConsumer`：只需读那条立即数操作数，不必快照整条 ops 表。
      // RETURN 已在上面早退，故不再列入 match 分支。
      let imm_idx = match op {
        LuauOpcode::LOP_SETLIST => Some(1),
        LuauOpcode::LOP_CALLFB | LuauOpcode::LOP_CALL => Some(0),
        _ => None,
      };
      let is_multi_consumer = match imm_idx {
        Some(idx) => match ops_buf.get(idx) {
          // SETLIST/CALL 系列指令的 imm 操作数在图构建时只存整数
          Some(imm_op) => self.target.immediates[imm_op.index as usize].as_int() < 0,
          None => false,
        },
        None => false,
      };

      // cpp `target.isVarArg && isMultiConsumer(...) && isGetVarArg(ops.back())` 的短路求值：
      // 仅当变参路径成立时才判定尾操作数。无输入的操作数（如 LOADNIL，其目标寄存器记在
      // `regs` 而非 `ops`）ops 为空，没有尾元素，路径自然不成立。
      let var_arg_tail = match (self.target.is_vararg && is_multi_consumer, ops_buf.last()) {
        (true, Some(last))
          if last.kind == BcOpKind::Inst
            && self.target.instructions[last.index as usize].op == LuauOpcode::LOP_GETVARARGS =>
        {
          Some(*last)
        }
        _ => None,
      };

      if let Some(last) = var_arg_tail {
        for &inp in &ops_buf {
          if inp != last {
            let mapped = self.map_to_caller_op(inp);
            self.caller.instructions[cidx].ops.push_back(mapped);
          } else {
            // varArgMoves 条目要留到整场内联结束（`mapToCallerOp` 处理 GETVARARGS
            // 投影时还会经 `getVarArgParam` 再读），故既不 take/remove 也不 clone，
            // 直接把 Copy 句柄搬过去。cpp 此处是 LUAU_ASSERT + `[]`；
            // release 下断言是 no-op，旧实现的 `unwrap()` 就是 panic 路径。
            debug_assert!(
              self.var_arg_moves.contains_key(&inp),
              "变参尾操作数必须已登记 varArgMoves"
            );
            let caller_ops = &mut self.caller.instructions[cidx].ops;
            if let Some(moves) = self.var_arg_moves.get(&inp) {
              caller_ops.extend(moves.iter().copied());
            }
          }
        }
        // cpp `makeFixedConsumer(caller, callerInst)`：句柄化后直接把调用方指令的
        // `BcOp` 交给它，内部经 `self.caller` 现取可变视图。
        self.make_fixed_consumer(caller_insn_op);
      } else {
        for &inp in &ops_buf {
          let mapped = self.map_to_caller_op(inp);
          self.caller.instructions[cidx].ops.push_back(mapped);
        }
      }

      if let Some(reg) = self.target.regs.get(&target_insn_op).copied() {
        let mapped_reg = self.map_to_caller_reg(reg);
        self.caller.regs.insert(caller_insn_op, mapped_reg);
      }

      // cpp `migrateInstructions` 尾部的 `switch (callerInst->op)`，目前只有 LOP_CALLFB 需要
      // 特殊处理：优化后的 feedback 向量是「caller 槽位 + target 槽位」拼接布局，被内联函数体
      // 里的 CALLFB 槽位必须整体加上 caller 的槽位数，否则写到错误的 feedback 单元，静默污染
      // 类型反馈。`-1` 是已密封（sealed）的 CALLFB，保持不动。
      let migrated_is_callfb = self.caller.instructions[cidx].op == LuauOpcode::LOP_CALLFB;
      if migrated_is_callfb {
        let mut fb = BcCallFB::from(self.caller, caller_insn_op);
        let slot = fb.fb_slot();
        if slot != -1 {
          fb.set_fb_slot(slot + self.caller_fb_vec_size as i32);
        }
      }
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_replace_call_usages_in_ops.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn replace_call_usages_in_ops(&mut self, ops: &mut SmallVector<BcOp, 4>) {
    for op in ops.iter_mut() {
      if let Some(proj_op) = self.call_projections.get(op) {
        let proj: &mut BcProj = self.caller.proj_op(*proj_op);
        LUAU_ASSERT!((proj.index as usize) < self.return_ops.len());
        *op = self.return_ops[proj.index as usize];
      }
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_replace_call_usages_with_return_phis.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn replace_call_usages_with_return_phis(&mut self) {
    // To avoid simultaneous mutable borrow of `self` and `self.caller`,
    // we must swap the ops out or access them in a way that doesn't hold a borrow
    // across the method call. take 换出 ops，处理完成后放回（零拷贝）。
    // i 是 caller 指令 arena 槽号（即 `BcOp::Inst` 的 index 域）：换出/放回都要精确指名
    // 该槽位，且每步回调 `&mut self`，`iter_mut()` 的借用与两者冲突，故保留下标遍历。
    for i in 0..self.caller_inst_size_before_inline {
      let mut ops = mem::take(&mut self.caller.instructions[i as usize].ops);
      self.replace_call_usages_in_ops(&mut ops);
      self.caller.instructions[i as usize].ops = ops;
    }

    // phi 序号即 `BcOp::Phi` 的 index（数据语义），故按下标遍历；
    // 同 take 换出技巧规避 `self` 与 `self.caller` 的并存可变借用。
    for i in 0..self.caller.phis.len() {
      let candidate = BcOp::with(BcOpKind::Phi, i as u32);
      if !self.return_ops.contains(&candidate) {
        let mut ops = mem::take(&mut self.caller.phis[i].ops);
        self.replace_call_usages_in_ops(&mut ops);
        self.caller.phis[i].ops = ops;
      }
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_replace_get_var_arg.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `replaceGetVarArg(callerBlock, targetGetVarArgsOp)`：把目标图的
  /// GETVARARGS 展开成调用方栈上的逐位 MOVE/LOADNIL，并登记到 `varArgMoves`。
  pub(crate) fn replace_get_var_arg(
    &mut self,
    caller_block_op: BcOp,
    target_get_var_args_op: BcOp,
  ) {
    // 目标侧 GETVARARGS 视图只在快照个数与起始寄存器期间存在，避免与下面
    // `self.caller` 的可变借用重叠。
    let (raw_count, start_reg) = {
      let mut get_var_args = BcGetVarArgs::from(self.target, target_get_var_args_op);
      (get_var_args.values_count(), get_var_args.start_reg())
    };
    let count = if raw_count < 0 {
      cmp::max(
        0,
        self.call_params.len() as i32 - self.target.numparams as i32,
      ) as usize
    } else {
      raw_count as usize
    };

    // 迭代器生成 moves，替代 C 风格索引循环
    let moves = (0..count)
      .map(|i| {
        let target_reg = start_reg as u32 + i as u32;
        let caller_reg = self.map_to_caller_reg(target_reg as Reg) as Reg;

        if (self.target.numparams as usize + i) < self.call_params.len() {
          let mut move_op = BcMove::create(self.caller);
          move_op.set_src(self.call_params[self.target.numparams as usize + i]);
          move_op.set_out_reg(caller_reg);
          move_op.append_to(caller_block_op);
          move_op.op()
        } else {
          let mut load_nil = BcLoadNil::create(self.caller);
          load_nil.set_out_reg(caller_reg);
          load_nil.append_to(caller_block_op);
          load_nil.op()
        }
      })
      .collect::<Vec<_>>();

    self.var_arg_moves.try_insert(target_get_var_args_op, moves);
  }
}

// ── abs-r139：并自 `methods/call_inliner_replace_namecall.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `replaceNamecall(BcNamecall&, BcRef<BcBlock>& prevBlock)`：
  /// 把 `NAMECALL` 摘出 `prevBlock` 挪到调用块，原位换成 `MOVE + GETTABLEKS`。
  ///
  /// `prev_block_op` 为块句柄（cpp 传 `BcRef&` 只为写 `ops.pop_back()`）。
  pub(crate) fn replace_namecall(&mut self, namecall: BcOp, prev_block_op: BcOp) -> BcOp {
    let call_block = self.call_block_op();

    // NAMECALL 视图仅在此块作用内存活，出来后 `self.caller` 重新可用
    let (table, hint, key, out_reg) = {
      let mut helper = BcInstHelper::new(self.caller, namecall);
      let table = helper.get_bc_op(0);
      let hint_op = helper.get_bc_op(1);
      let key = helper.get_bc_op(2).index;
      // Safety:NAMECALL 的 hint 输入在图构建期只可能以 `BcImmKind::Int` 写入。
      let hint = helper.graph.imm_op(hint_op).as_int() as u32;
      let out_reg = helper.get_out_reg();
      helper.prepend_to(call_block);
      (table, hint, key, out_reg)
    };

    self.caller.block_op(prev_block_op).ops.pop_back();
    LUAU_ASSERT!(self.target_reg == out_reg);
    let table_reg = out_reg + 1;

    // and replace it with LOP_MOVE + LOP_GETTABLEKS
    let mut move_helper = BcMove::create(self.caller);
    move_helper.set_out_reg(table_reg);
    move_helper.set_src(table);
    move_helper.append_to(prev_block_op);
    let move_op = move_helper.op();

    // cpp `namecall.setTable(move.op())`（BytecodeCallInliner.h:137）：
    // GETTABLEKS 会把方法闭包写回 targetReg，可能覆写 NAMECALL 原 table 寄存器，
    // 故必须把 NAMECALL 的 Table 输入（ops[0]）重定向到新 MOVE 的 out 寄存器。
    BcInstHelper::new(self.caller, namecall).set_bc_op(0, move_op);

    let mut get_table_ks_helper = BcGetTableKS::create(self.caller);
    get_table_ks_helper.set_source(move_op);
    get_table_ks_helper.set_hint(hint);
    get_table_ks_helper.set_key(key);
    get_table_ks_helper.set_out_reg(self.target_reg);
    get_table_ks_helper.append_to(prev_block_op);

    get_table_ks_helper.op()
  }
}

// ── abs-r139：并自 `methods/call_inliner_replace_return.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `replaceReturn(BcRef<BcBlock>& nextBlock, BcOp callerBlockOp, BcOp targetReturnOp)`：
  /// 把目标图的一条定长 RETURN 换成调用方块里的 MOVE/LOADNIL 序列，并把该块接到
  /// `nextBlock`。入参全部句柄化，可变访问经 `self.caller` 完成。
  pub(crate) fn replace_return(
    &mut self,
    next_block_op: BcOp,
    caller_block_op: BcOp,
    target_return_op: BcOp,
  ) -> bool {
    let mut ret = BcReturn::from(self.target, target_return_op);
    let return_count = ret.return_count();
    if return_count < 0 {
      return false;
    }
    let values = ret.values();

    for (i, &src) in values.iter().enumerate() {
      let src = self.map_to_caller_op(src);
      let mut move_op = BcMove::create(self.caller);
      move_op.set_src(src);
      move_op.set_out_reg(self.target_reg + i as u8);
      move_op.append_to(caller_block_op);
      let op = move_op.op();
      self.set_return_op(i as u32, op);
    }

    let call_res = self.call_view().return_count();
    LUAU_ASSERT!(call_res >= 0);
    let call_res = call_res as u32;

    for i in values.len() as u32..call_res {
      let mut load_nil = BcLoadNil::create(self.caller);
      load_nil.set_out_reg(self.target_reg + i as u8);
      load_nil.append_to(caller_block_op);
      let op = load_nil.op();
      self.set_return_op(i, op);
    }

    self
      .caller
      .block_op(caller_block_op)
      .successors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: next_block_op,
      });
    self
      .caller
      .block_op(next_block_op)
      .predecessors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: caller_block_op,
      });

    true
  }
}

// ── abs-r139：并自 `methods/call_inliner_set_return_op.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `setReturnOp(idx, op)`：第 `idx` 个返回槽位累计多个候选时折叠成 phi。
  pub(crate) fn set_return_op(&mut self, idx: u32, op: BcOp) {
    if (idx as usize) >= self.return_ops.len() {
      self.return_ops.resize(idx as usize + 1, BcOp::new());
    }

    if self.return_ops[idx as usize].kind == BcOpKind::None {
      self.return_ops[idx as usize] = op;
      return;
    }

    if self.return_ops[idx as usize].kind != BcOpKind::Phi {
      let phi_op = self.caller.add_phi();
      let previous = self.return_ops[idx as usize];
      self.caller.phi_op(phi_op).ops.push_back(previous);
      self.return_ops[idx as usize] = phi_op;
    } else {
      // cpp `setReturnOp`（BytecodeCallInliner.h:262-271）：phi 槽位已含同一 op 时
      // 先去重再 addUse，避免产生重复 phi 操作数、与 cpp 图状态分歧。
      let phi_op = self.return_ops[idx as usize];
      let exists = self.caller.phi_op(phi_op).ops.contains(&op);
      if !exists {
        self.caller.phi_op(phi_op).ops.push_back(op);
      }
    }
  }
}

// ── abs-r139：并自 `methods/call_inliner_split_block_on_op.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `splitBlockOnOp(BcOp splitOp)`：把 `splitOp` 所在块切成
  /// `prev | insn | next` 三块并接好 fallthrough 边。
  ///
  /// 返回 `(prevBlock, nextBlock)` 的 `BcOp` 句柄：cpp 返回 `pair<BcRef<BcBlock>, ...>`
  /// 并把 `vec` 借用谎报成图的生命期（`&self.caller.blocks as *const _ as &'a _`），
  /// 后续再经 `BcRef::operator->` 写回——那是 Stacked Borrows UB，且 `blocks`
  /// 重分配后引用悬垂。句柄化后调用方一律经 `block_op` 现取现用。
  pub(crate) fn split_block_on_op(&mut self, split_op: BcOp) -> (BcOp, BcOp) {
    let prev_block_op = self.caller.instructions[split_op.index as usize].block;
    LUAU_ASSERT!(prev_block_op.kind == BcOpKind::Block);
    LUAU_ASSERT!(
      self.caller.blocks[prev_block_op.index as usize]
        .ops
        .iter()
        .any(|op| *op == split_op)
    );

    let insn_block_op = self.caller.add_block();
    let prev_block_sortkey = self.caller.blocks[prev_block_op.index as usize].sortkey;
    let prev_block_chainkey = self.caller.blocks[prev_block_op.index as usize].chainkey;
    self.caller.blocks[insn_block_op.index as usize].sortkey = prev_block_sortkey;
    self.caller.blocks[insn_block_op.index as usize].chainkey = prev_block_chainkey + 1;

    let next_block_op = self.caller.add_block();
    self.caller.blocks[next_block_op.index as usize].sortkey =
      self.caller.blocks[insn_block_op.index as usize].sortkey;
    self.caller.blocks[next_block_op.index as usize].chainkey =
      self.caller.blocks[insn_block_op.index as usize].chainkey + 1;

    // 把 split_op 之前的尾部指令整体搬到 next 块（split_op 本身随后落入 insn 块）
    while let Some(&back_op) = self.caller.blocks[prev_block_op.index as usize].ops.back() {
      if back_op == split_op {
        break;
      }
      self.caller.blocks[prev_block_op.index as usize]
        .ops
        .pop_back();
      self.caller.blocks[next_block_op.index as usize]
        .ops
        .push_front(back_op);
      self.caller.instructions[back_op.index as usize].block = next_block_op;
    }

    // 整表 take 而非 clone：循环里要可变借用后继块的 predecessors 改写回指，
    // 克隆只为断开与 self.caller 的借用；take 直接把旧边表所有权交给 next，
    // 后面那句 clear 也一并省去（循环体不读 prev.successors，行为等价）。
    let prev_successors =
      mem::take(&mut self.caller.blocks[prev_block_op.index as usize].successors);
    for e in &prev_successors {
      let succ = e.target;
      for pred in self.caller.blocks[succ.index as usize]
        .predecessors
        .iter_mut()
      {
        if pred.target == prev_block_op {
          pred.target = next_block_op;
        }
      }
    }
    self.caller.blocks[next_block_op.index as usize].successors = prev_successors;

    self.add_successor(prev_block_op, insn_block_op, BcBlockEdgeKind::Fallthrough);
    self.add_successor(insn_block_op, next_block_op, BcBlockEdgeKind::Fallthrough);

    // 上方搬移循环只在 `back == split_op` 时 break，而入口处已断言 split_op 属于
    // prev 块的 ops，故此刻 ops 必以 split_op 结尾：`back().unwrap()` 100% 安全。
    LUAU_ASSERT!(
      *self.caller.blocks[prev_block_op.index as usize]
        .ops
        .back()
        .unwrap()
        == split_op
    );
    self.caller.blocks[prev_block_op.index as usize]
      .ops
      .pop_back();
    self.caller.blocks[insn_block_op.index as usize]
      .ops
      .push_back(split_op);
    self.caller.instructions[split_op.index as usize].block = insn_block_op;

    (prev_block_op, next_block_op)
  }
}

// ── abs-r139：并自 `methods/call_inliner_validate_cfg.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  pub(crate) fn validate_cfg(&self) -> bool {
    let validate_edges =
      |from: u32, edges: &BcEdges, mirror_dir: fn(&BcBlock) -> &BcEdges| -> bool {
        for edge in edges {
          if edge.target.kind != BcOpKind::Block
            || edge.target.index as usize >= self.caller.blocks.len()
          {
            return false;
          }

          let other = &self.caller.blocks[edge.target.index as usize];

          if BcBlockFlag::Dead.is_set(other.flags) {
            return false;
          }

          let mirror = mirror_dir(other);
          if !mirror.iter().any(|e| {
            e.kind == edge.kind && e.target.kind == BcOpKind::Block && e.target.index == from
          }) {
            return false;
          }
        }
        true
      };

    for (i, block) in self.caller.blocks.iter().enumerate() {
      if BcBlockFlag::Dead.is_set(block.flags) {
        continue;
      }

      if !validate_edges(i as u32, &block.successors, |b| &b.predecessors) {
        return false;
      }

      if !validate_edges(i as u32, &block.predecessors, |b| &b.successors) {
        return false;
      }
    }

    true
  }
}

// ── abs-r139：并自 `methods/call_inliner_validate.rs` ──
impl<'a, 'c, 't> CallInliner<'a, 'c, 't> {
  /// cpp `validate()`（BytecodeCallInliner.h:762-769）：图状态 = validateCfg 与
  /// validatePhis 两道网，内联入口/出口各跑一次（h:680/757）。
  pub(crate) fn validate(&self) -> bool {
    self.validate_cfg() && self.validate_phis()
  }

  /// cpp `validatePhis()`（BytecodeCallInliner.h:771-777）：phi 的操作数只允许
  /// Inst / VmReg / Proj / Phi 四类输入；内联图损坏时这里是第一层 debug 网。
  pub(crate) fn validate_phis(&self) -> bool {
    for phi in &self.caller.phis {
      for op in phi.ops.iter() {
        LUAU_ASSERT!(
          op.kind == BcOpKind::Inst
            || op.kind == BcOpKind::VmReg
            || op.kind == BcOpKind::Proj
            || op.kind == BcOpKind::Phi
        );
      }
    }
    true
  }
}
