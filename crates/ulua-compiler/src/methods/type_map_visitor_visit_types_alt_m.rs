use ulua_ast::{records::ast_expr_group::AstExprGroup, visit};

use crate::records::type_map_visitor::TypeMapVisitor;

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_group(&mut self, node: *mut AstExprGroup) -> bool {
    unsafe {
      let expr = (*node).expr;

      visit::ast_expr_visit(expr, self);

      if let Some(&ty_ptr) = self.resolved_exprs.find(&expr) {
        self.record_resolved_type_ast_expr_ast_type(node as *mut _, ty_ptr);
      }
    }

    false
  }
}
