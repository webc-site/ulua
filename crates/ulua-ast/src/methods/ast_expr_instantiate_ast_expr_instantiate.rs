use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_instantiate::AstExprInstantiate,
    ast_node::AstNode, ast_type_or_pack::AstTypeOrPack, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprInstantiate {
  pub fn new(location: Location, expr: *mut AstExpr, types: AstArray<AstTypeOrPack>) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      expr,
      type_arguments: types,
    }
  }
}

pub fn ast_expr_instantiate_ast_expr_instantiate(
  location: Location,
  expr: *mut AstExpr,
  types: AstArray<AstTypeOrPack>,
) -> AstExprInstantiate {
  AstExprInstantiate::new(location, expr, types)
}
