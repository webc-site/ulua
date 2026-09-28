use crate::records::{
  ast_array::AstArray,
  ast_expr::AstExpr,
  ast_expr_table::{AstExprTable, Item},
  location::Location,
};

impl_ast_node_new!(AstExprTable, AstExpr, location: Location, items: AstArray<Item>);
