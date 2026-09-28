use core::ptr::{from_mut, null_mut};

use ulua_ast::records::{
  ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_visitor::AstVisitor, position::Position,
};
#[derive(Debug, Clone)]
pub struct NearestStatementFinder {
  pub(crate) cursor: Position,
  pub(crate) nearest: *mut AstStat,
  pub(crate) parent: *mut AstStatBlock,
}

impl NearestStatementFinder {
  pub fn new(cursor_position: Position) -> Self {
    Self {
      cursor: cursor_position,
      nearest: null_mut(),
      parent: null_mut(),
    }
  }
}

impl AstVisitor for NearestStatementFinder {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    let block_location = node.base.base.location;
    if !(block_location.contains(self.cursor) || block_location.end == self.cursor) {
      return false;
    }
    self.parent = from_mut(node);

    // 取 begin <= cursor 的最后一条语句。
    for v in node.body.iter_nodes() {
      let stmt_location = v.get().base.location;
      if stmt_location.begin <= self.cursor {
        self.nearest = v.as_ptr();
      }
    }

    true
  }
}
