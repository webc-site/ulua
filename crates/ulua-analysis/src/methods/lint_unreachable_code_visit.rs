use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_stat::AstStat};

use crate::records::lint_unreachable_code::LintUnreachableCode;

impl LintUnreachableCode {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit(&mut self, node: *mut AstExprFunction) -> bool {
    let node_ref = unsafe { &*node };
    if !node_ref.body.is_null() {
      self.analyze(node_ref.body as *mut AstStat);
    }
    true
  }
}
