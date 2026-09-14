use ulua_ast::records::{ast_local::AstLocal, location::Location};

use crate::records::{compile_error::CompileError, compiler::Compiler};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn check_exported_local(&mut self, local: *mut AstLocal, location: &Location) {
    unsafe {
      if (*local).is_exported {
        if !self.at_top_level() {
          // C++ `CompileError::raise(...)`: throw a typed CompileError, not a String.
          CompileError::raise(
            location,
            format_args!("'export' may only be applied to top-level statements"),
          );
        }

        self.exported_locals.push(local);
      }
    }
  }
}
