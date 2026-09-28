use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
  visit::AstVisitable,
};

use crate::records::{find_full_ancestry::FindFullAncestry, source_module::SourceModule};

pub fn find_ast_ancestry_of_position_source_module_position_bool(
  source: &SourceModule,
  pos: Position,
  include_types: bool,
) -> Vec<*mut AstNode> {
  if source.root.is_null() {
    return Vec::new();
  }
  find_ast_ancestry_of_position_ast_stat_block_position_bool(
    // SAFETY: 上方已判空，root 指向 arena 存活的 AstStatBlock；visitor 按 cpp
    // 非 const `AstNode::visit` 语义需要独占借用
    unsafe { &mut *source.root },
    pos,
    include_types,
  )
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
  // Safety: 派生指针由上方 &mut root 的存活借用转换而来；&AstStatBlock->*const AstNode
  // 是上转，依赖 AstStatBlock 为 repr(C) 且首字段 base: AstNode 基址重合，读 location
  // 即读首字段区，只读且单线程，转回共享仅经由后续 visit 的独占借用之前完成。
  let end = unsafe { (*root_node_ptr).location.end };

  if pos > end {
    pos = end;
  }

  let mut finder = FindFullAncestry::new(pos, end, include_types);

  root.visit(&mut finder);

  finder.nodes
}
