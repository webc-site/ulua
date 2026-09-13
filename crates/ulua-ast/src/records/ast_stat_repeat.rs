use crate::{
  records::{ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatRepeat {
  pub base: AstStat,
  pub condition: *mut AstExpr,
  pub body: *mut AstStatBlock,
  pub deprecated_has_until: bool,
}

impl AstNodeClass for AstStatRepeat {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatRepeat");
}
