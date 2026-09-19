use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_type::AstType, ast_type_typeof::AstTypeTypeof,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeTypeof {
  pub fn new(location: Location, expr: *mut AstExpr) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      expr,
    }
  }
}
