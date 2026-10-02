use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::{
  arena_handle::alias_ref, find_full_ancestry::FindFullAncestry, source_module::SourceModule,
};

pub fn find_ast_ancestry_of_position_source_module_position_bool(
  source: &SourceModule,
  pos: Position,
  include_types: bool,
) -> Vec<*mut AstNode> {
  // cpp `if (!source.root) return {};`：缺席态即 `None`，在场态经 `Handle`
  // 物化独占借用（`AstNode::visit` 需非 const `this`，见下方 `_bool` 形参注记）。
  let Some(root) = source.root else {
    return Vec::new();
  };
  find_ast_ancestry_of_position_ast_stat_block_position_bool(root.get_mut(), pos, include_types)
}

// Alias to match the published interface name
pub fn find_ast_ancestry_of_position(
  source: &SourceModule,
  pos: Position,
  include_types: bool,
) -> Vec<*mut AstNode> {
  find_ast_ancestry_of_position_source_module_position_bool(source, pos, include_types)
}

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
  let end = alias_ref(root_node_ptr).location.end;

  if pos > end {
    pos = end;
  }

  let mut finder = FindFullAncestry::new(pos, end, include_types);

  root.visit(&mut finder);

  finder.nodes
}
