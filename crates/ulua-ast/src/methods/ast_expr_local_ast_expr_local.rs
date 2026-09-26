use crate::records::{
  ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_local::AstLocal, location::Location,
  node_handle::Node,
};

impl_ast_node_new!(
  AstExprLocal,
  AstExpr,
  location: Location, local: Node<AstLocal>, upvalue: bool,
);
