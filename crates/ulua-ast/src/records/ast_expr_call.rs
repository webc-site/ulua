use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack, location::Location,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprCall {
  pub base: AstExpr,
  pub func: *mut AstExpr,
  pub type_arguments: AstArray<AstTypeOrPack>,
  pub args: AstArray<*mut AstExpr>,
  pub self_: bool,
  pub arg_location: Location,
}
