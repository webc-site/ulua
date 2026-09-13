//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ControlFlowGraph.h:36:block_id`
//! Source: `Analysis/include/Luau/ControlFlowGraph.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/ControlFlowGraph.h
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TypedAllocator.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/ControlFlowGraph.h
//!   - type_ref <- record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref <- record ControlFlowGraph (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref <- record CFGBuilder (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref <- method Block::getPredecessors (Analysis/src/ControlFlowGraph.cpp)
//!   - type_ref <- method Block::getSuccessors (Analysis/src/ControlFlowGraph.cpp)
//!   - type_ref <- method ControlFlowGraph::newBlock (Analysis/src/ControlFlowGraph.cpp)
//!   - type_ref <- method CFGBuilder::readVariable (Analysis/src/ControlFlowGraph.cpp)
//!   - type_ref <- method CFGBuilder::fillJoinOperands (Analysis/src/ControlFlowGraph.cpp)
//!   - type_ref <- function dumpCFG (Analysis/src/DumpCFG.cpp)
//!   - type_ref <- function indexOfBlock (Analysis/src/DumpCFG.cpp)
//!   - type_ref <- function dumpCFGJson (Analysis/src/DumpCFG.cpp)
//!   - type_ref <- function checkSuccessors (tests/ControlFlowGraph.test.cpp)
//!   - type_ref <- function checkPredecessors (tests/ControlFlowGraph.test.cpp)
//! - outgoing:
//!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item BlockId

// C++ `using BlockId = NotNull<Block>;` — mirrored as a raw pointer, matching
// `InstrId = *mut Instruction` (NotNull -> raw ptr).
use crate::records::block::Block;
pub type BlockId = *mut Block;
