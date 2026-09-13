use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::const_upvalue_visitor::ConstUpvalueVisitor;

impl ConstUpvalueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    unsafe {
      if (*node).upvalue && (*self.self_).is_constant(node as *mut _) {
        self.upvals.push((*node).local);
      }
    }
    false
  }
}
