use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::{
  arena_handle::alias, autocomplete_node_finder::AutocompleteNodeFinder,
  source_module::SourceModule,
};

pub fn find_ancestry_at_position_for_autocomplete_source_module_position(
  source: &SourceModule,
  pos: Position,
) -> Vec<*mut AstNode> {
  if source.root.is_null() {
    return Vec::new();
  }
  find_ancestry_at_position_for_autocomplete_ast_stat_block_position(alias(source.root), pos)
}

// Alias to match the published interface name
pub fn find_ancestry_at_position_for_autocomplete(
  source: &SourceModule,
  pos: Position,
) -> Vec<*mut AstNode> {
  find_ancestry_at_position_for_autocomplete_source_module_position(source, pos)
}

pub fn find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
  root: &mut AstStatBlock,
  pos: Position,
) -> Vec<*mut AstNode> {
  let mut finder = AutocompleteNodeFinder::new(pos);
  root.visit(&mut finder);
  finder.ancestry
}
