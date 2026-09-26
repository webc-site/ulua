use std::collections::VecDeque;

use ulua_ast::records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor};

use super::node::Block;

/// `Enqueuer` is a visitor that pushes visited `AstStatBlock`s into a queue.
/// 队列元素是 arena 句柄（cpp `std::queue<AstStatBlock*>` 的等价形态）。
#[derive(Debug)]
pub struct Enqueuer<'q> {
  pub queue: &'q mut VecDeque<Block>,
}

impl AstVisitor for Enqueuer<'_> {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    // 从 visitor 交给的可变借用收编坐标；借用随本次 visit 结束，句柄仅在
    // Reducer 的串行处理窗口内再被解引用（见 `records::node` 模块契约）。
    self.queue.push_back(Block::from(node));
    false
  }
}
