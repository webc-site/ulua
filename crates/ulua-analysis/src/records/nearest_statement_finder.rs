use core::{ffi::c_void, ptr::null_mut};

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
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    let block = node as *mut AstStatBlock;
    unsafe {
      let block_location = (*block).base.base.location;
      if block_location.contains(self.cursor) || block_location.end == self.cursor {
        self.parent = block;

        // Find last statement whose begin <= cursor.
        for v in (*block).body.iter() {
          let v = *v;
          let stmt_location = { (*v).base.location };
          if stmt_location.begin <= self.cursor {
            self.nearest = v;
          }
        }

        true
      } else {
        false
      }
    }
  }
}
