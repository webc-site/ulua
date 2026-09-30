//! `control_flow_graph` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::block_kind::BlockKind,
  records::{cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph},
  type_aliases::block_id::BlockId,
};

// Source: `Analysis/include/Luau/ControlFlowGraph.h:259-262` (hand-ported)
// C++ `explicit ControlFlowGraph::ControlFlowGraph(NotNull<CFGAllocator> allocator)`.

impl ControlFlowGraph {
  pub fn new(allocator: *mut CfgAllocator) -> Self {
    Self {
      // C++ `DenseHashMap<AstExpr*, Definition*> useDefs{nullptr};`
      use_defs: DenseHashMap::default(),
      // C++ `std::vector<BlockId> blocks;`
      blocks: Vec::new(),
      // C++ `size_t entryIdx = 0;`
      entry_idx: 0,
      // C++ member-init `: allocator(allocator)`
      allocator,
    }
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:120-124` (hand-ported)
// C++ `BlockId ControlFlowGraph::newBlock(BlockKind kind, std::string debugName)`.

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
