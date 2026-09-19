use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::find_full_ancestry::FindFullAncestry;

/// C++ `findAstAncestryOfPosition(AstStatBlock* root, Position, bool)`
/// （`AstQuery.cpp:247`）：形参在 cpp 侧即非 const `AstStatBlock*`，因为
/// `AstNode::visit` 需要非 const `this`；Rust 因此取 `&mut`（而非共享引用再
/// 伪造 `*mut`）。
pub fn find_ast_ancestry_of_position_ast_stat_block_position_bool(
  root: &mut AstStatBlock,
  mut pos: Position,
  include_types: bool,
) -> Vec<*mut AstNode> {
  let root_node_ptr = &*root as *const AstStatBlock as *const AstNode;
  let end = unsafe { (*root_node_ptr).location.end };

  if pos > end {
    pos = end;
  }

  let mut finder = FindFullAncestry::new(pos, end, include_types);

  root.visit(&mut finder);

  finder.nodes
}
