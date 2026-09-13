use core::ptr::null_mut;

use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  functions::find_node_at_position_ast_query_alt_b::find_node_at_position_ast_stat_block_position,
  records::source_module::SourceModule,
};
pub fn find_node_at_position_source_module_position(
  source: &SourceModule,
  pos: Position,
) -> *mut AstNode {
  if source.root.is_null() {
    return null_mut();
  }
  find_node_at_position_ast_stat_block_position(unsafe { &*source.root }, pos)
}
