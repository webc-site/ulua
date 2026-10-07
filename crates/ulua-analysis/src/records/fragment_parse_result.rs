use alloc::{boxed::Box, string::String, vec::Vec};

use ulua_ast::records::{
  allocator::Allocator, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
  comment::Comment, position::Position,
};

use crate::records::arena_handle::Handle;

#[derive(Debug)]
pub struct FragmentParseResult {
  pub fragment_to_parse: String,
  /// §2：cpp `parseFragment` 成功返回前已判 `parseResult.root != nullptr`
  /// （FragmentAutocomplete.cpp），故构造点恒注入非空句柄；目标由本结构
  /// 自带的 `alloc`（fragment 解析 arena）保活，`Handle` 契约随宿主存续。
  pub root: Handle<AstStatBlock>,
  pub ancestry: Vec<*mut AstNode>,
  /// §2：cpp `FragmentParseResult::nearestStatement`：未命中时回退为根块的
  /// `AstStat` 视图（FragmentAutocomplete.cpp `nearestStatement ?: (AstStat*)root`），
  /// 可空槽以 `Option<Handle>` 表达。
  pub nearest_statement: Option<Handle<AstStat>>,
  pub comment_locations: Vec<Comment>,
  pub alloc: Box<Allocator>,
  pub scope_pos: Position,
}
