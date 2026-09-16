use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_type_assertion::AstExprTypeAssertion},
  visit,
};

use crate::records::type_map_visitor::TypeMapVisitor;

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_type_assertion(&mut self, node: *mut AstExprTypeAssertion) -> bool {
    unsafe {
      let node_ref = &*node;

      visit::ast_expr_visit(node_ref.expr, self);

      self.record_resolved_type_ast_expr_ast_type(
        node as *mut AstExpr,
        node_ref.annotation as *const _,
      );
    }

    false
  }
}
