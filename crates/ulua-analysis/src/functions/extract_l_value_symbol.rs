use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::symbol::Symbol;
pub fn extract_l_value_symbol(target: &AstExpr) -> Option<Symbol> {
  unsafe {
    let local = ast_node_as::<AstExprLocal>(target as *const AstExpr as *mut AstNode);
    if !local.is_null() {
      return Some(Symbol::from_local((*local).local));
    }

    let global = ast_node_as::<AstExprGlobal>(target as *const AstExpr as *mut AstNode);
    if !global.is_null() {
      return Some(Symbol::from_global((*global).name));
    }

    None
  }
}
