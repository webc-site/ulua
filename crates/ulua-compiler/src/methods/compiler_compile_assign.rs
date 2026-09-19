use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{compiler::Compiler, l_value::LValue};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_assign(
    &mut self,
    lv: &LValue,
    source: u8,
    target_expr: *mut AstExpr,
  ) {
    unsafe {
      self.compile_l_value_use(lv, source, true, target_expr);
    }
  }
}
