use crate::{
  records::{
    ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    location::Location,
  },
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatFor {
  pub base: AstStat,
  pub var: *mut AstLocal,
  pub from: *mut AstExpr,
  pub to: *mut AstExpr,
  pub step: *mut AstExpr,
  pub body: *mut AstStatBlock,
  pub has_do: bool,
  pub do_location: Location,
}

impl AstNodeClass for AstStatFor {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatFor");
}
