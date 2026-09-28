use alloc::collections::BTreeSet;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph, symbol::Symbol},
  type_aliases::{block_id::BlockId, instr_id::InstrId},
};

#[derive(Debug, Clone)]
pub struct CfgBuilder {
  pub cfg: Option<ControlFlowGraph>,
  pub allocator: *mut CfgAllocator,
  pub current_block: BlockId,
  pub sealed_blocks: DenseHashSet<BlockId>,
  pub incomplete_joins: DenseHashMap<BlockId, BTreeSet<InstrId>>,
  pub version_counter: DenseHashMap<Symbol, usize>,
}
