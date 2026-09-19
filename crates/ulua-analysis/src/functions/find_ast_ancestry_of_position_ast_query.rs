use alloc::vec::Vec;

use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  functions::find_ast_ancestry_of_position_ast_query_alt_b::find_ast_ancestry_of_position_ast_stat_block_position_bool,
  records::source_module::SourceModule,
};

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
