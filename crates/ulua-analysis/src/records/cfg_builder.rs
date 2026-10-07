use alloc::collections::BTreeSet;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::Handle, cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph,
    symbol::Symbol,
  },
  type_aliases::{block_id::BlockId, instr_id::InstrId},
};

#[derive(Debug, Clone)]
pub struct CfgBuilder {
  pub cfg: Option<ControlFlowGraph>,
  // C++ `NotNull<CFGAllocator> allocator;`：arena 属主的别名句柄，构建期经
  // `get_mut` 物化借用（arena_handle 模块契约），本结构不拥有、不释放。
  pub allocator: Handle<CfgAllocator>,
  pub current_block: BlockId,
  pub sealed_blocks: DenseHashSet<BlockId>,
  pub incomplete_joins: DenseHashMap<BlockId, BTreeSet<InstrId>>,
  pub version_counter: DenseHashMap<Symbol, usize>,
}
