//! Source: `Analysis/include/Luau/ControlFlowGraph.h`

// C++ `using BlockId = NotNull<Block>;` — mirrored as a raw pointer, matching
// `InstrId = *mut Instruction` (NotNull -> raw ptr).
use crate::records::block::Block;
pub type BlockId = *mut Block;
