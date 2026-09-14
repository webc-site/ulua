use crate::records::{ast_expr::AstExpr, ast_node::AstNode, location::Location};

impl AstExpr {
  pub fn new(class_index: i32, location: Location) -> Self {
    Self {
      base: AstNode {
        class_index,
        location,
      },
    }
  }
}

pub fn ast_expr_ast_expr(class_index: i32, location: Location) -> AstExpr {
  AstExpr::new(class_index, location)
}
