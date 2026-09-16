use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, location::Location,
  },
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatForIn {
  pub base: AstStat,
  pub vars: AstArray<*mut AstLocal>,
  pub values: AstArray<*mut AstExpr>,
  pub body: *mut AstStatBlock,
  pub has_in: bool,
  pub in_location: Location,
  pub has_do: bool,
  pub do_location: Location,
}

impl AstNodeClass for AstStatForIn {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatForIn");
}
