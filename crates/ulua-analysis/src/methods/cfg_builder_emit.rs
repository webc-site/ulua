//! Source: `Analysis/include/Luau/ControlFlowGraph.h:306-312` (hand-ported)
//! C++ `template<typename T, typename... Args> NotNull<T> CFGBuilder::emit(Block* block, Args&&... args)`.
/// Bridges the C++ `T{args...}` variadic construction: maps each instruction
/// type `T` and its constructor arg-pack to a built `Instruction` variant.
/// (C++ `instructions.allocate(T{std::forward<Args>(args)...})`.)
use alloc::vec::Vec;

use ulua_ast::records::{ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal};

use crate::{
  records::{
    assign::Assign, block_registry::resolve_block_mut, cfg_builder::CfgBuilder, declare::Declare,
    join::Join, refine::Refine,
  },
  type_aliases::{
    block_id::BlockId, def_id_control_flow_graph::DefId, instr_id::InstrId,
    instruction::Instruction, refinement_id_control_flow_graph::RefinementId,
  },
};
pub trait IntoInstruction<T> {
  fn into_instruction(self) -> Instruction;
}

// emit<Declare>(block, (def, source)) -> Declare(def, source)
impl IntoInstruction<Declare> for (DefId, *mut AstStatLocal) {
  fn into_instruction(self) -> Instruction {
    Instruction::Declare(Declare::new(self.0, self.1))
  }
}

// emit<Assign>(block, (def, source)) -> Assign(def, source)
impl IntoInstruction<Assign> for (DefId, *mut AstStatAssign) {
  fn into_instruction(self) -> Instruction {
    Instruction::Assign(Assign::new(self.0, self.1))
  }
}

// emit<Join>(block, def) -> Join(definition)
impl IntoInstruction<Join> for DefId {
  fn into_instruction(self) -> Instruction {
    // C++ `explicit Join(DefId definition)` — operands start empty.
    Instruction::Join(Join {
      definition: self,
      operands: Vec::new(),
    })
  }
}

// emit<Refine>(block, (definition, prop)) -> Refine(definition, prop)
impl IntoInstruction<Refine> for (DefId, RefinementId) {
  fn into_instruction(self) -> Instruction {
    Instruction::Refine(Refine {
      definition: self.0,
      prop: self.1,
    })
  }
}

impl CfgBuilder {
  /// `template<typename T, typename... Args> NotNull<T> emit(Block* block, Args&&...)`.
  /// `block` 自 #17 续起为 `BlockId`、返回值 `NotNull<Instruction>` 为 `InstrId`
  /// u32 句柄（C++ 的 `get_if<T>` 下转在调用侧经 `resolve_instruction` +
  /// `InstructionMember::get_if` 甄别完成）。
  pub(crate) fn emit<T, Args>(&mut self, block: BlockId, args: Args) -> InstrId
  where
    Args: IntoInstruction<T>,
  {
    // C++:
    //   InstrId inst = allocator->newInstruction<T>(std::forward<Args>(args)...);
    //   block->instructions.emplace_back(inst);
    //   return NotNull{inst->template get_if<T>()};
    // Safety: allocator 构造期由 CfgBuilder 的驱动方（make_cfg/cfg_builder 系列）
    // 从其存活的 CfgAllocator 接线，非空且比 builder 长寿；emit 持有 &mut self，
    // 调用栈内此刻不存在第二条指向该 allocator 的存活借用，&mut 重建安全。
    let allocator = unsafe { &mut *self.allocator };
    let inst = allocator.new_instruction(args.into_instruction());
    // block 句柄经注册表写回（见 `block_registry` 模块契约）：inst 是上一行刚
    // 发放的存活指令句柄，只入列不解析（转储/Join 补全侧再经
    // `instr_registry` 解析）。
    resolve_block_mut(block)
      .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
      .instructions
      .push(inst);
    inst
  }
}
