use ulua_ast::records::ast_node::AstNode;

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn patch_jumps(&mut self, node: *mut AstNode, labels: &mut [usize], target: usize) {
    for l in labels.iter().copied() {
      unsafe { self.patch_jump(node, l, target) };
    }
  }
}
