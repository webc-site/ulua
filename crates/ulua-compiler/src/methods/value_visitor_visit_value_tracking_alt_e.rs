use ulua_ast::{
  records::{ast_expr::AstExpr, ast_stat_function::AstStatFunction},
  visit::ast_expr_visit,
};

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    unsafe {
      let node = &*node;
      self.assign(node.name);
      // C++ `node->func->visit(this)` is the AST node's dispatch — it runs the
      // visit_expr_function callback (registers args) AND recurses into the
      // function body. Calling the bare callback skipped the body, so no local
      // declared inside the function was ever tracked (recordValue then panicked).
      ast_expr_visit(node.func as *mut AstExpr, self);
    }
    false
  }
}
