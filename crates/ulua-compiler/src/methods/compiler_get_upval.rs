use ulua_ast::records::ast_local::AstLocal;

use crate::records::{compile_error::CompileError, compiler::Compiler};

const K_MAX_UPVALUE_COUNT: usize = 200;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_upval(&mut self, local: *mut AstLocal) -> u8 {
    for (uid, &upval) in self.upvals.iter().enumerate() {
      if upval == local {
        return uid as u8;
      }
    }

    if self.upvals.len() >= K_MAX_UPVALUE_COUNT {
      let local_ref = unsafe { &*local };
      CompileError::raise(
        &local_ref.location,
        format_args!(
          "Out of upvalue registers when trying to allocate {}: exceeded limit {}",
          local_ref.name, K_MAX_UPVALUE_COUNT
        ),
      );
    }

    if self.variables.find(&local).is_some_and(|v| v.written) {
      self.locals.get_or_insert(local).captured = true;
    }

    self.upvals.push(local);
    (self.upvals.len() - 1) as u8
  }
}
