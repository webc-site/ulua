use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat_type_function(&mut self, stat: *mut AstStatTypeFunction) {
    // SAFETY: body 指向 AST arena 节点。
    self.visit_ast_expr_function(unsafe { &*(*stat).body });
  }
}
