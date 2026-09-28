use alloc::vec::Vec;

use ulua_ast::records::{
  ast_local::AstLocal, ast_name::AstName, ast_node::AstNode, ast_stat::AstStat,
  ast_stat_block::AstStatBlock, location::Location,
};
use ulua_common::records::dense_hash_map::DenseHashMap;
#[derive(Debug, Clone)]
pub struct FragmentAutocompleteAncestryResult {
  pub local_map: DenseHashMap<AstName, *mut AstLocal>,
  pub local_stack: Vec<*mut AstLocal>,
  pub ancestry: Vec<*mut AstNode>,
  pub nearest_statement: *mut AstStat,
  pub parent_block: *mut AstStatBlock,
  pub fragment_selection_region: Location,
}
