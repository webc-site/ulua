use crate::{
  records::{ast_expr::AstExpr, ast_local::AstLocal},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprLocal {
  pub base: AstExpr,
  pub local: *mut AstLocal,
  pub upvalue: bool,
}

impl AstNodeClass for AstExprLocal {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprLocal");
}
