use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{compiler::Compiler, reg_scope::RegScope};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_auto(&mut self, node: *mut AstExpr, _rs: &mut RegScope) -> u8 {
    let reg = self.get_expr_local_reg(node);
    if reg >= 0 {
      return reg as u8;
    }

    let reg = unsafe { self.alloc_reg(node as *mut _, 1) };
    unsafe { self.compile_expr_temp(node, reg) };
    reg
  }
}
