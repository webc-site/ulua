use ulua_ast::records::ast_expr::AstExpr;

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_temp(&mut self, node: *mut AstExpr, target: u8) {
    unsafe { self.compile_expr(node, target, true) };
  }
}
