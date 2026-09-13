use crate::{
  records::{
    ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location,
  },
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatWhile {
  pub base: AstStat,
  pub condition: *mut AstExpr,
  pub body: *mut AstStatBlock,
  pub has_do: bool,
  pub do_location: Location,
}

impl AstNodeClass for AstStatWhile {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatWhile");
}
