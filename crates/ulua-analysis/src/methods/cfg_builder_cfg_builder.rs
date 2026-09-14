//! Source: `Analysis/src/ControlFlowGraph.cpp:127-133` (hand-ported)
//! C++ `explicit CFGBuilder::CFGBuilder(NotNull<CFGAllocator> allocator)`.
use alloc::string::ToString;
use core::ptr::null_mut;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  enums::block_kind::BlockKind,
  records::{
    cfg_allocator::CfgAllocator, cfg_builder::CfgBuilder, control_flow_graph::ControlFlowGraph,
    symbol::Symbol,
  },
};
impl CfgBuilder {
  pub fn new(allocator: *mut CfgAllocator) -> Self {
    // C++ member-init order:
    //   cfg(std::make_unique<ControlFlowGraph>(allocator))
    //   allocator(allocator)
    //   currentBlock(cfg->newBlock(BlockKind::Entry, "Entry Block"))
    let mut cfg = ControlFlowGraph::new(allocator);
    let current_block = cfg.new_block(BlockKind::Entry, "Entry Block".to_string());

    let mut builder = Self {
      cfg: Some(cfg),
      allocator,
      current_block,
      // C++ `sealedBlocks{nullptr}`
      sealed_blocks: DenseHashSet::new(null_mut()),
      // C++ `incompleteJoins{nullptr}`
      incomplete_joins: DenseHashMap::new(null_mut()),
      // C++ `versionCounter{Symbol{}}`
      version_counter: DenseHashMap::new(Symbol::default()),
    };

    // C++ constructor body: seal(currentBlock);
    builder.seal(current_block);
    builder
  }
}
