use ulua_ast::records::ast_node::AstNode;

use crate::records::{
  compile_error::{CompileError, ERR_EXCEEDED_JUMP_DISTANCE_LIMIT},
  compiler::Compiler,
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn patch_jump(&mut self, node: *mut AstNode, label: usize, target: usize) {
    let ok = unsafe { (*self.bytecode).patch_jump_d(label, target) };
    if !ok {
      let location = unsafe { (*node).location };
      CompileError::raise(
        &location,
        format_args!("{ERR_EXCEEDED_JUMP_DISTANCE_LIMIT}"),
      );
    }
  }
}
