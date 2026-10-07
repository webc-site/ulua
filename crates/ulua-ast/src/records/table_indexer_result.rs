use crate::records::{ast_table_indexer::AstTableIndexer, node_handle::Node, position::Position};

/// cpp `parseTableIndexer` 的出参元组：`node` 恒为该方法内 `allocator.alloc` 的
/// 产物（失败即中止，非空由 [`Node`] 句柄在类型层承载）。
#[derive(Debug, Clone)]
pub struct TableIndexerResult {
  pub node: Node<AstTableIndexer>,
  pub indexer_open_position: Position,
  pub indexer_close_position: Position,
  pub colon_position: Position,
}
