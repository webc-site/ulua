use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
    constant::Constant,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_constant_number(&mut self, node: *mut AstExpr) -> i32 {
    unsafe {
      if let Some(Constant::Number(n)) = self.constants.find(&node) {
        let cid = (*self.bytecode).add_constant_number(*n);
        if cid < 0 {
          CompileError::raise(
            &(*node).base.location,
            core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }
        return cid;
      }

      -1
    }
  }
}
