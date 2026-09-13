use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprLocal {
  pub fn new(location: Location, local: *mut AstLocal, upvalue: bool) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      local,
      upvalue,
    }
  }
}

pub fn ast_expr_local_ast_expr_local(
  location: Location,
  local: *mut AstLocal,
  upvalue: bool,
) -> AstExprLocal {
  AstExprLocal::new(location, local, upvalue)
}
