use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_name::AstName, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprGlobal {
  pub fn new(location: Location, name: AstName) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      name,
    }
  }
}

pub fn ast_expr_global_ast_expr_global(location: Location, name: AstName) -> AstExprGlobal {
  AstExprGlobal::new(location, name)
}
