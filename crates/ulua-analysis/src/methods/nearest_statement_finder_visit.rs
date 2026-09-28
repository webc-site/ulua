use ulua_ast::records::{ast_stat_block::AstStatBlock, location::Location};

use crate::records::nearest_statement_finder::NearestStatementFinder;

impl NearestStatementFinder {
  /// # Safety
  /// - `block` 须为非空、对齐且指向 arena 存活 `AstStatBlock` 的句柄，存活期覆盖本次
  ///   遍历；`&mut self` 仅借用 finder 字段，与 AST 节点无别名（对应 C++ 契约）。
  pub unsafe fn visit(&mut self, block: *mut AstStatBlock) -> bool {
    // Safety: 契约保证 block 为 arena 存活的非空对齐节点，重建只读借用无并存可变别名。
    let block_ref = unsafe { &*block };
    let block_location: Location = block_ref.base.base.location;

    if block_location.contains(self.cursor) {
      self.parent = block;

      let body = &block_ref.body;
      for stat in body.iter_nodes() {
        let stat_ref = stat.get();
        let stat_location = stat_ref.base.location;

        if stat_location.begin <= self.cursor {
          self.nearest = stat.as_ptr();
        }
      }

      true
    } else {
      false
    }
  }
}
