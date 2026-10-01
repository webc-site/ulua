use crate::records::{
  ast_expr::AstExpr, ast_expr_type_assertion::AstExprTypeAssertion, ast_type::AstType,
  location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstExprTypeAssertion,
  AstExpr,
  location: Location,
  expr: Node<AstExpr>,
  annotation: Node<AstType>,
);
