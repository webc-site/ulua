use ulua_ast::records::ast_node::AstNode;

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn set_debug_line_end(&mut self, node: *mut AstNode) {
    if self.options.debug_level >= 1 {
      let line = unsafe { (*node).location.end.line + 1 };
      unsafe { (*self.bytecode).set_debug_line(line as i32) };
    }
  }
}
