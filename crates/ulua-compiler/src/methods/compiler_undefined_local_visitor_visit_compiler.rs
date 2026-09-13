use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::undefined_local_visitor::UndefinedLocalVisitor;

impl UndefinedLocalVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    unsafe {
      if !(*node).upvalue {
        self.check((*node).local);
      }
    }
    false
  }
}
