use core::ffi::c_void;
use std::collections::VecDeque;

use ulua_ast::records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor};

/// `Enqueuer` is a visitor that pushes visited `AstStatBlock`s into a queue.
/// It is native-only and not portable to wasm32-unknown-unknown.
#[repr(C)]
#[derive(Debug)]
pub struct Enqueuer {
  pub queue: *mut VecDeque<*mut AstStatBlock>,
}

impl Enqueuer {
  pub fn new(queue: *mut VecDeque<*mut AstStatBlock>) -> Self {
    debug_assert!(!queue.is_null());
    Enqueuer { queue }
  }
}

impl AstVisitor for Enqueuer {
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    let block = node as *mut AstStatBlock;
    // SAFETY: `self.queue` is non-null as ensured in `new`.
    unsafe {
      (*self.queue).push_back(block);
    }
    false
  }
}
