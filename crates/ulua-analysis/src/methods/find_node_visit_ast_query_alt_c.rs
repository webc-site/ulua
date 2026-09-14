use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock},
  visit::ast_stat_visit,
};

use crate::records::find_node::FindNode;

impl FindNode {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
    unsafe { self.visit_ast_node(block as *mut AstNode) };

    let block_ref = unsafe { &*block };
    let body = block_ref.body;

    for &stat in body.as_slice() {
      let stat_ref = unsafe { &*stat };

      if stat_ref.base.location.end < self.pos {
        continue;
      }
      if stat_ref.base.location.begin > self.pos {
        break;
      }

      unsafe {
        ast_stat_visit(stat, self);
      }
    }

    false
  }
}
