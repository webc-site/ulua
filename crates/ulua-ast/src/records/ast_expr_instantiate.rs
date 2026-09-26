use crate::records::{ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprInstantiate {
  pub base: AstExpr,
  pub expr: *mut AstExpr,
  pub type_arguments: AstArray<AstTypeOrPack>,
}
