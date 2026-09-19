use ulua_ast::records::ast_local::AstLocal;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{compile_error::CompileError, compiler::Compiler};

const K_DEFAULT_ALLOC_PC: u32 = !0u32;
const K_MAX_LOCAL_COUNT: usize = 200;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn push_local(&mut self, local: *mut AstLocal, reg: u8, allocpc: u32) {
    if self.local_stack.len() >= K_MAX_LOCAL_COUNT {
      let local_ref = unsafe { &*local };
      CompileError::raise(
        &local_ref.location,
        format_args!(
          "Out of local registers when trying to allocate {}: exceeded limit {}",
          local_ref.name, K_MAX_LOCAL_COUNT
        ),
      );
    }

    self.local_stack.push(local);

    let debugpc = unsafe { (*self.bytecode).get_debug_pc() };
    let l = self.locals.get_or_insert(local);
    LUAU_ASSERT!(!l.allocated);

    l.reg = reg;
    l.allocated = true;
    l.debugpc = debugpc;
    l.allocpc = if allocpc == K_DEFAULT_ALLOC_PC {
      l.debugpc
    } else {
      allocpc
    };
  }
}
