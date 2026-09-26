//! Source: `Analysis/include/Luau/ControlFlowGraph.h:237-243` (hand-ported)
//! C++ `template<typename T, typename... Args> InstrId CFGAllocator::newInstruction(Args&&... args)`.
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{cfg_allocator::CfgAllocator, instr_registry::register_instruction},
  type_aliases::{instr_id::InstrId, instruction::Instruction},
};

impl CfgAllocator {
  /// C++ builds `Instruction* inst = instructions.allocate(T{args...})` and
  /// returns `NotNull{inst}`. The variant `T{args...}` is constructed by the
  /// caller (`CFGBuilder::emit`) and threaded through as the `Instruction`.
  pub fn new_instruction(&mut self, inst: Instruction) -> InstrId {
    // C++: LUAU_ASSERT(!frozen);
    LUAU_ASSERT!(!self.frozen);
    // C++: Instruction* inst = instructions.allocate(...); return NotNull{inst};
    // 裸指针仅在注册点出现：arena 地址经 `register_instruction` 换成 u32
    // 句柄，之后全链（Block.instructions、incomplete_joins、转储）只持有/
    // 比较句柄（见 `records::instr_registry` 模块契约）。
    register_instruction(self.instructions.allocate(inst))
  }
}
