use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) {
    let _stack_pusher = self.push_stack(block as *mut AstNode);

    unsafe {
      let body = (*block).body;
      for &stat in body.as_slice() {
        self.visit_ast_stat(stat);
      }
    }
  }
}
