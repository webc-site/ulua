use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{Item, ItemKind},
    ast_node::AstNode,
  },
  rtti::ast_node_is,
};
pub fn is_record(item: &Item) -> bool {
  if item.kind == ItemKind::Record {
    true
  } else if item.kind == ItemKind::General {
    if item.key.is_null() {
      return false;
    }

    if ast_node_is::<AstExprConstantString>(unsafe { &*(item.key as *mut AstNode) }) {
      return true;
    }

    false
  } else {
    false
  }
}
