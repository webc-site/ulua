use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
    location::Location,
  },
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatLocal {
  pub base: AstStat,
  pub vars: AstArray<*mut AstLocal>,
  pub values: AstArray<*mut AstExpr>,
  pub is_const: bool,
  pub is_exported: bool,
  pub equals_sign_location: Option<Location>,
}

impl AstNodeClass for AstStatLocal {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatLocal");
}
