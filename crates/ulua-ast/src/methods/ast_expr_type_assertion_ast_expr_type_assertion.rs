use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode,
    ast_type::AstType, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprTypeAssertion {
  pub fn new(location: Location, expr: *mut AstExpr, annotation: *mut AstType) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      expr,
      annotation,
    }
  }
}

pub fn ast_expr_type_assertion_ast_expr_type_assertion(
  location: Location,
  expr: *mut AstExpr,
  annotation: *mut AstType,
) -> AstExprTypeAssertion {
  AstExprTypeAssertion::new(location, expr, annotation)
}
