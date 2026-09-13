use ulua_ast::records::ast_node::AstNode;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_compiler::Type,
  records::{compiler::Compiler, loop_jump::LoopJump},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn patch_loop_jumps(
    &mut self,
    node: *mut AstNode,
    old_jumps: usize,
    end_label: usize,
    cont_label: usize,
  ) {
    LUAU_ASSERT!(old_jumps <= self.loop_jumps.len());

    for i in old_jumps..self.loop_jumps.len() {
      let lj: &LoopJump = &self.loop_jumps[i];

      match lj.r#type {
        Type::Break => {
          unsafe { self.patch_jump(node, lj.label, end_label) };
        }
        Type::Continue => {
          unsafe { self.patch_jump(node, lj.label, cont_label) };
        }
      }
    }
  }
}
