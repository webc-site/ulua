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
  if source.root.is_null() {
    return Vec::new();
  }
  find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    // Safety: 上方已判空；root 指向 parser arena 中存活的 AstStatBlock（bump 页、块地址
    // 不移动），&mut 重建匹配 C++ 非 const AstNode::visit 的签名语义；source 仅按共享
    // 借用读取 root 字段本身，遍历期间除该 &mut 外无其他借用者，单线程串行无别名冲突。
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

pub fn find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
  root: &mut AstStatBlock,
  pos: Position,
) -> Vec<*mut AstNode> {
  let mut finder = AutocompleteNodeFinder::new(pos);
  root.visit(&mut finder);
  finder.ancestry
}
