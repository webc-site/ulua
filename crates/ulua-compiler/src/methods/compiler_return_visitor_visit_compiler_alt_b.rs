use ulua_ast::records::ast_stat_return::AstStatReturn;

use crate::records::return_visitor::ReturnVisitor;

impl ReturnVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_return(&mut self, stat: *mut AstStatReturn) -> bool {
    unsafe {
      self.returns_one &=
        (*stat).list.size == 1 && !(*self.self_).is_expr_mult_ret(*(*stat).list.data.add(0));
    }
    false
  }
}
