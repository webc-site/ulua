use core::ffi::c_void;

use ulua_ast::records::ast_stat_repeat::AstStatRepeat;

use crate::records::lint_multi_line_statement::LintMultiLineStatement;

impl LintMultiLineStatement {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) -> bool {
    let node_body = unsafe { (*node).body };
    self.visit_ast_stat_block(node_body);
    let _ = { node as *mut c_void };
    false
  }
}
