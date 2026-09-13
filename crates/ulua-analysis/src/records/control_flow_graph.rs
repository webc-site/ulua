//! Source: `Analysis/include/Luau/ControlFlowGraph.h:257` (hand-ported)
//! C++ `struct ControlFlowGraph`.
use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::cfg_allocator::CfgAllocator,
  type_aliases::{block_id::BlockId, definition::Definition},
};

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
  // Maps each use of a local variable (AstExpr*) to the Definition* live at
  // that point. C++ `DenseHashMap<AstExpr*, Definition*> useDefs{nullptr};`
  pub use_defs: DenseHashMap<*mut AstExpr, *mut Definition>,

  pub blocks: Vec<BlockId>,
  pub entry_idx: usize,

  // private:
  pub(crate) allocator: *mut CfgAllocator,
}

unsafe impl Send for ControlFlowGraph {}
unsafe impl Sync for ControlFlowGraph {}
