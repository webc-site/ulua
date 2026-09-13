use crate::{
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_table::{AstExprTable, Item},
    ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprTable {
  pub fn new(location: Location, items: AstArray<Item>) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      items,
    }
  }
}

pub fn ast_expr_table_ast_expr_table(location: Location, items: AstArray<Item>) -> AstExprTable {
  AstExprTable::new(location, items)
}
