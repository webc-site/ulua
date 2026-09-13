use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprInstantiate {
  pub base: AstExpr,
  pub expr: *mut AstExpr,
  pub type_arguments: AstArray<AstTypeOrPack>,
}

impl AstNodeClass for AstExprInstantiate {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprInstantiate");
}
