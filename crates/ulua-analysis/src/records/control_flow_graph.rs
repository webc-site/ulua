//! Source: `Analysis/include/Luau/ControlFlowGraph.h:257` (hand-ported)
//! C++ `struct ControlFlowGraph`.
use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{arena_handle::Handle, cfg_allocator::CfgAllocator},
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
  // C++ `NotNull<CFGAllocator> allocator;`：arena 属主的别名句柄（§2 句柄化，
  // 见 `records::arena_handle` 模块契约），本结构不拥有、不释放该 arena。
  pub(crate) allocator: Handle<CfgAllocator>,
}

// Safety: use_defs 的 *mut AstExpr 键为借用自外部 AST 的裸指针、allocator 为
// 借用自外部 arena 的非拥有别名句柄（值为 sym_def_registry 发放的 u32 句柄，
// 不携带地址），本结构只透传与哈希这些地址值，从不转移其所有权；被借用的
// AST/CfgAllocator 由构造契约保证在 CFG 存活期内有效，故按值转移（Send）只是
// 转移身份句柄，不复制资源所有权。
unsafe impl Send for ControlFlowGraph {}
// Safety: 同上——所借用的 AST/SymDef/arena 在本结构内只被只读访问
// （查询 use_defs、经 allocator 发放块句柄），共享 &ControlFlowGraph 只产生
// 并发只读；底层 arena 在其存活期内不移动、不释放，故无数据竞争。
unsafe impl Sync for ControlFlowGraph {}
