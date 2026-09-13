use crate::{
  records::{ast_expr::AstExpr, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeTypeof {
  pub base: AstType,
  pub expr: *mut AstExpr,
}

impl AstNodeClass for AstTypeTypeof {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeTypeof");
}
