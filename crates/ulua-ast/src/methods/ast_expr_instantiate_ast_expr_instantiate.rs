use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_instantiate::AstExprInstantiate,
    ast_type_or_pack::AstTypeOrPack, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprInstantiate {
  pub fn new(location: Location, expr: *mut AstExpr, types: AstArray<AstTypeOrPack>) -> Self {
    Self {
      base: AstExpr::new(<Self as AstNodeClass>::CLASS_INDEX, location),
      expr,
      type_arguments: types,
    }
  }
}
