use crate::{
  records::{
    ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location,
  },
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatIf {
  pub base: AstStat,
  pub condition: *mut AstExpr,
  pub thenbody: *mut AstStatBlock,
  pub elsebody: *mut AstStat,
  pub then_location: Option<Location>,
  pub else_location: Option<Location>,
}

impl AstNodeClass for AstStatIf {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatIf");
}
