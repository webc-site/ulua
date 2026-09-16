use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
  rtti::ast_node_as,
};

use crate::records::symbol::Symbol;
pub fn is_being_defined(ancestry: &[*mut AstNode], symbol: &Symbol) -> bool {
  if symbol.local.is_null() {
    return false;
  }

  let mut iter = ancestry.len();
  while iter > 0 {
    iter -= 1;
    let node = ancestry[iter];
    let stat_local = unsafe { ast_node_as::<AstStatLocal>(node) };
    if stat_local.is_null() {
      continue;
    }

    let vars = unsafe { &(*stat_local).vars };
    for var in vars {
      if *var == symbol.local {
        return true;
      }
    }
  }

  false
}
