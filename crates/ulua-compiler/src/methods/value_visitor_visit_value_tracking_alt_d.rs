use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
    unsafe {
      let node_ref = &*node;
      self.variables.get_or_insert(node_ref.name as *mut _).init = node_ref.func as *mut _;
    }

    true
  }
}
