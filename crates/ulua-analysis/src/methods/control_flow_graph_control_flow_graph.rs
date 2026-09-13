//! Source: `Analysis/include/Luau/ControlFlowGraph.h:259-262` (hand-ported)
//! C++ `explicit ControlFlowGraph::ControlFlowGraph(NotNull<CFGAllocator> allocator)`.
use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph};
impl ControlFlowGraph {
  pub fn new(allocator: *mut CfgAllocator) -> Self {
    Self {
      // C++ `DenseHashMap<AstExpr*, Definition*> useDefs{nullptr};`
      use_defs: DenseHashMap::new(null_mut()),
      // C++ `std::vector<BlockId> blocks;`
      blocks: Vec::new(),
      // C++ `size_t entryIdx = 0;`
      entry_idx: 0,
      // C++ member-init `: allocator(allocator)`
      allocator,
    }
  }
}
