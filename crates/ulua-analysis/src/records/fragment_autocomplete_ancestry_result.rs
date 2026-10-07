use alloc::vec::Vec;

use ulua_ast::records::{
  ast_local::AstLocal, ast_name::AstName, ast_node::AstNode, ast_stat::AstStat,
  ast_stat_block::AstStatBlock, location::Location,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::arena_handle::Handle;

#[derive(Debug, Clone)]
pub struct FragmentAutocompleteAncestryResult {
  pub local_map: DenseHashMap<AstName, *mut AstLocal>,
  pub local_stack: Vec<*mut AstLocal>,
  pub ancestry: Vec<*mut AstNode>,
  /// §2：cpp `FragmentAutocompleteAncestryResult`（port 记录）的
  /// `AstStat* nearestStatement` / `AstStatBlock* parentBlock` 可空槽：
  /// `lastGoodParse == nullptr` 的空结果早退 ≡ `None`（cpp 双 null 字段），
  /// 命中为解析 arena 存活节点（`Handle` 模块契约）。
  pub nearest_statement: Option<Handle<AstStat>>,
  pub parent_block: Option<Handle<AstStatBlock>>,
  pub fragment_selection_region: Location,
}
