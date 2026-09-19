//! `AstExprTable::getRecord` (`Ast/src/Ast.cpp:396`).

use crate::{
  records::{
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

impl AstExprTable {
  pub fn get_record_bytes(&self, key: &[u8]) -> Option<*mut AstExpr> {
    for item in self.items.iter() {
      if item.kind == ItemKind::Record {
        let string_expr = unsafe { ast_node_as::<AstExprConstantString>(item.key as *mut AstNode) };
        if !string_expr.is_null() {
          let value = unsafe { (*string_expr).value };
          if value.as_bytes() == key {
            return Some(item.value);
          }
        }
      }
    }
    None
  }

  #[inline]
  pub fn get_record_str(&self, key: &str) -> Option<*mut AstExpr> {
    self.get_record_bytes(key.as_bytes())
  }
}
