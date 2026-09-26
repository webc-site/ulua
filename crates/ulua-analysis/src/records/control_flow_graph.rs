//! Source: `Analysis/include/Luau/ControlFlowGraph.h:257` (hand-ported)
//! C++ `struct ControlFlowGraph`.
use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::cfg_allocator::CfgAllocator,
  type_aliases::{block_id::BlockId, def_id_control_flow_graph::DefId},
};

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
  // Maps each use of a local variable (AstExpr*) to the Definition* live at
  // that point. C++ `DenseHashMap<AstExpr*, Definition*> useDefs{nullptr};`
  // Rust 侧值为 `SymDefId` u32 句柄（#17，见 `records::sym_def_registry`）。
  pub use_defs: DenseHashMap<*mut AstExpr, DefId>,

  pub blocks: Vec<BlockId>,
  pub entry_idx: usize,

  // private:
  pub(crate) allocator: *mut CfgAllocator,
}

// Safety: use_defs 的 *mut AstExpr 键与 allocator 均为借用自外部
// AST/CfgAllocator 的裸指针（值为 sym_def_registry 发放的 u32 句柄，不携带
// 地址），本结构只透传与哈希这些指针值，从不解引用或释放它们；被借用的 arena
// 由构造契约保证在 CFG 存活期内有效，故按值转移（Send）只是转移地址身份，
// 不复制资源所有权。
unsafe impl Send for ControlFlowGraph {}
// Safety: 同上——所借用的 AST/SymDef/allocator 在本结构内只被只读访问
// （查询 use_defs、透传 allocator），共享 &ControlFlowGraph 只产生并发只读；
// 底层 arena 在其存活期内不移动、不释放，故无数据竞争。
unsafe impl Sync for ControlFlowGraph {}
