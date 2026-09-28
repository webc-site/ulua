use ulua_ast::records::ast_stat_continue::AstStatContinue;

use crate::records::node::Node;

#[derive(Debug, Clone, Copy)]
pub struct Loop {
  pub(crate) local_offset: usize,
  pub(crate) local_offset_continue: usize,
  /// 本循环是否已见 continue 语句（cpp null 哨兵 → Option，命中存其首现节点）
  pub(crate) continue_used: Option<Node<AstStatContinue>>,
}
