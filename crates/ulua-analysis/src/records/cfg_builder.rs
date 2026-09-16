use alloc::collections::BTreeSet;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  block::Block, cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph, join::Join,
  symbol::Symbol,
};

#[derive(Debug, Clone)]
pub struct CfgBuilder {
  pub cfg: Option<ControlFlowGraph>,
  pub allocator: *mut CfgAllocator,
  pub current_block: *mut Block,
  pub sealed_blocks: DenseHashSet<*mut Block>,
  pub incomplete_joins: DenseHashMap<*mut Block, BTreeSet<*mut Join>>,
  pub version_counter: DenseHashMap<Symbol, usize>,
}
