use ulua_ast::{records::ast_expr_function::AstExprFunction, visit::ast_stat_block_visit};

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn gather_const_upvals(&mut self, func: *mut AstExprFunction) {
    let mut visitor = self.const_upvalue_visitor_const_upvalue_visitor();
    unsafe {
      ast_stat_block_visit(&*(*func).body, &mut visitor);
    }
    for local in visitor.upvals {
      unsafe { self.get_upval(local) };
    }
  }
}
