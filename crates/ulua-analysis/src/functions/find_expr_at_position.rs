use core::ptr::{NonNull, null_mut};

use ulua_ast::records::{ast_expr::AstExpr, position::Position};

use crate::{
  functions::find_node_at_position_ast_query::find_node_at_position_source_module_position,
  records::{arena_handle::alias, source_module::SourceModule},
};
pub fn find_expr_at_position(source: &SourceModule, pos: Position) -> *mut AstExpr {
  let node = find_node_at_position_source_module_position(source, pos);
  if !node.is_null() {
    alias(node).as_expr().map_or(null_mut(), NonNull::as_ptr)
  } else {
    null_mut()
  }
}
