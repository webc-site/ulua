//! Source: `Analysis/src/ControlFlowGraph.cpp:120-124` (hand-ported)
//! C++ `BlockId ControlFlowGraph::newBlock(BlockKind kind, std::string debugName)`.
use alloc::string::String;

use crate::{
  enums::block_kind::BlockKind, records::control_flow_graph::ControlFlowGraph,
  type_aliases::block_id::BlockId,
};

impl ControlFlowGraph {
  pub fn new_block(&mut self, kind: BlockKind, debug_name: String) -> BlockId {
    // C++:
    //   Block* b = allocator->newBlock(kind, debugName);
    //   return blocks.emplace_back(b);
    let b: BlockId = unsafe { (*self.allocator).new_block(kind, debug_name) };
    self.blocks.push(b);
    b
  }
}
