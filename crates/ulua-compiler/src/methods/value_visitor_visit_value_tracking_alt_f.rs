use core::ptr::null_mut;

use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    if node.is_null() {
      return false;
    }

    unsafe {
      let node_ref = &*node;
      let args = &node_ref.args;
      // C++ `variables[arg].init = nullptr`: operator[] overwrites `.init` for an
      // existing entry (try_insert would leave it unchanged).
      for &arg in args.iter() {
        if !arg.is_null() {
          self.variables.get_or_insert(arg).init = null_mut();
        }
      }
    }

    true
  }
}
