use ulua_ast::records::{ast_stat_block::AstStatBlock, location::Location};

use crate::records::nearest_statement_finder::NearestStatementFinder;

impl NearestStatementFinder {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit(&mut self, block: *mut AstStatBlock) -> bool {
    let block_ref = unsafe { &*block };
    let block_location: Location = block_ref.base.base.location;

    if block_location.contains(self.cursor) {
      self.parent = block;

      let body = block_ref.body;
      for &stat in body.as_slice() {
        let stat_ref = unsafe { &*stat };
        let stat_location = stat_ref.base.location;

        if stat_location.begin <= self.cursor {
          self.nearest = stat;
        }
      }

      true
    } else {
      false
    }
  }
}
