use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;

pub fn find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
  root: &mut AstStatBlock,
  pos: Position,
) -> Vec<*mut AstNode> {
  let mut finder = AutocompleteNodeFinder::new(pos);
  root.visit(&mut finder);
  finder.ancestry
}
