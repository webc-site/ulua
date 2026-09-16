use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::find_full_ancestry::FindFullAncestry;

pub fn find_ast_ancestry_of_position_ast_stat_block_position_bool(
  root: &AstStatBlock,
  mut pos: Position,
  include_types: bool,
) -> Vec<*mut AstNode> {
  let root_node_ptr = root as *const AstStatBlock as *const AstNode;
  let end = unsafe { (*root_node_ptr).location.end };

  if pos > end {
    pos = end;
  }

  let mut finder = FindFullAncestry::new(pos, end, include_types);

  // AstStatBlock inherits from AstStat, which inherits from AstNode.
  // In Rust, the base field chain is root.base.base for AstNode.
  // However, the compiler error indicates AstStat might not have a 'base' field in this version,
  // or it's accessed differently. Based on the AstNodeClass pattern, we can use the visit trait.
  root.visit(&mut finder);

  finder.nodes
}
