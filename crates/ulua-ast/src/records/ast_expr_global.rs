use crate::{
  records::{ast_expr::AstExpr, ast_name::AstName},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprGlobal {
  pub base: AstExpr,
  pub name: AstName,
}

impl AstNodeClass for AstExprGlobal {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprGlobal");
}
