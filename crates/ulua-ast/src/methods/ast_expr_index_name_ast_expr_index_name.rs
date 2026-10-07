use crate::records::{
  ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, ast_name::AstName, location::Location,
  node_handle::Node, position::Position,
};

impl_ast_node_new!(
  AstExprIndexName,
  AstExpr,
  location: Location,
  expr: Node<AstExpr>,
  index: AstName,
  index_location: Location,
  op_position: Position,
  op: u8,
);
