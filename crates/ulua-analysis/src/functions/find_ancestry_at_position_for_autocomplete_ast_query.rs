use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::{
  autocomplete_node_finder::AutocompleteNodeFinder, source_module::SourceModule,
};

pub fn find_ancestry_at_position_for_autocomplete_source_module_position(
  source: &SourceModule,
  pos: Position,
) -> Vec<*mut AstNode> {
  // cpp `if (!source.root) return {};`：缺席态即 `None`，在场态经 `Handle`
  // 物化独占借用（visit 需非 const `this`）。
  let Some(root) = source.root else {
    return Vec::new();
  };
  find_ancestry_at_position_for_autocomplete_ast_stat_block_position(root.get_mut(), pos)
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
