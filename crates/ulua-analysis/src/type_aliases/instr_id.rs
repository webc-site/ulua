//! Source: `Analysis/include/Luau/ControlFlowGraph.h:38` (hand-ported)
// C++ `using InstrId = NotNull<Instruction>` over ControlFlowGraph.h's
// OWN Instruction variant (previously mis-aliased to Instruction.h's unrelated
// InstrId).
use crate::type_aliases::instruction::Instruction;
pub type InstrId = *mut Instruction;
