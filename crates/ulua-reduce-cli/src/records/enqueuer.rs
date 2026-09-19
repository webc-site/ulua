use core::ffi::c_void;
use std::collections::VecDeque;

use ulua_ast::records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor};

/// `Enqueuer` is a visitor that pushes visited `AstStatBlock`s into a queue.
/// 借用版（原 `*mut VecDeque` + null 契约在 release 下无守卫，Rust 可直接 `&mut`）。
#[derive(Debug)]
pub struct Enqueuer<'q> {
  pub queue: &'q mut VecDeque<*mut AstStatBlock>,
}

impl<'q> Enqueuer<'q> {
  pub fn new(queue: &'q mut VecDeque<*mut AstStatBlock>) -> Self {
    Enqueuer { queue }
  }
}

impl AstVisitor for Enqueuer<'_> {
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    let block = node as *mut AstStatBlock;
    self.queue.push_back(block);
    false
  }
}
