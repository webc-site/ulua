use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location};

#[derive(Debug, Clone)]
pub struct FragmentRegion {
  pub fragment_location: Location,
  pub nearest_statement: *mut AstStat,
  pub parent_block: *mut AstStatBlock,
}
