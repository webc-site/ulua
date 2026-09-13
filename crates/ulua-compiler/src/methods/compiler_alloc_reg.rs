use core::cmp::max;

use ulua_ast::records::ast_node::AstNode;

use crate::records::{compile_error::CompileError, compiler::Compiler};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn alloc_reg(&mut self, node: *mut AstNode, count: u32) -> u8 {
    let top = self.reg_top;
    let k_max_register_count = 255;

    if top + count > k_max_register_count {
      let location = unsafe { (*node).location };
      // C++ `CompileError::raise(...)`: throw a typed CompileError (panic_any),
      // not a String panic, so `compile()`'s catch can recover it.
      CompileError::raise(
        &location,
        format_args!(
          "Out of registers when trying to allocate {} registers: exceeded limit {}",
          count, k_max_register_count
        ),
      );
    }

    self.reg_top += count;
    self.stack_size = max(self.stack_size, self.reg_top);

    top as u8
  }
}
