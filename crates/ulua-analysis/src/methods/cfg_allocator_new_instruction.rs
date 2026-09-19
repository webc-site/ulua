//! Source: `Analysis/include/Luau/ControlFlowGraph.h:237-243` (hand-ported)
//! C++ `template<typename T, typename... Args> InstrId CFGAllocator::newInstruction(Args&&... args)`.
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::cfg_allocator::CfgAllocator,
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
    self.instructions.allocate(inst)
  }
}
