use alloc::vec::Vec;

use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  functions::find_ancestry_at_position_for_autocomplete_ast_query_alt_b::find_ancestry_at_position_for_autocomplete_ast_stat_block_position,
  records::source_module::SourceModule,
};

pub fn find_ancestry_at_position_for_autocomplete_source_module_position(
  source: &SourceModule,
  pos: Position,
) -> Vec<*mut AstNode> {
  if source.root.is_null() {
    return Vec::new();
  }
  find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    unsafe { &mut *source.root },
    pos,
  )
}

// Alias to match the published interface name
pub fn find_ancestry_at_position_for_autocomplete(
  source: &SourceModule,
  pos: Position,
) -> Vec<*mut AstNode> {
  find_ancestry_at_position_for_autocomplete_source_module_position(source, pos)
}
