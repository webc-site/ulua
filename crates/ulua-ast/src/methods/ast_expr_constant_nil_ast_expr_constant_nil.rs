use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_nil::AstExprConstantNil, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprConstantNil {
  pub fn new(location: Location) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
    }
  }
}

pub fn ast_expr_constant_nil_ast_expr_constant_nil(location: Location) -> AstExprConstantNil {
  AstExprConstantNil::new(location)
}
