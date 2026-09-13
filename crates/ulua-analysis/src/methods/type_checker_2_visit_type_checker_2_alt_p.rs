use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local_function(&mut self, stat: *mut AstStatLocalFunction) {
    unsafe {
      let func = (*stat).func;
      // SAFETY: func 指向 AST arena 节点。
      self.visit_ast_expr_function(&*func);
    }
  }
}
