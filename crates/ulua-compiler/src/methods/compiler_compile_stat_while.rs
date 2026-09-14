use core::ptr::{null, null_mut};

use ulua_ast::records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_while::AstStatWhile};
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::type_compiler::Type::Break,
  records::{compiler::Compiler, r#loop::Loop, loop_jump::LoopJump},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_stat_while(&mut self, stat: *mut AstStatWhile) {
    unsafe {
      let stat_ref = &*stat;

      // Optimization: condition is always false => there's no loop!
      if self.is_constant_false(stat_ref.condition) {
        return;
      }

      let old_jumps = self.loop_jumps.len();
      let old_locals = self.local_stack.len();

      self.loops.push(Loop {
        local_offset: old_locals,
        local_offset_continue: old_locals,
        continue_used: null_mut(),
      });
      self.has_loops = true;

      let loop_label = (*self.bytecode).emit_label();

      let mut else_jump = Vec::new();
      self.compile_condition_value(stat_ref.condition, null(), &mut else_jump, false);

      self.compile_stat(stat_ref.body as *mut AstStat);

      let cont_label = (*self.bytecode).emit_label();
      let back_label = (*self.bytecode).emit_label();

      self.set_debug_line_ast_node(stat as *mut AstNode);

      // Note: this is using JUMPBACK, not JUMP, since JUMPBACK is interruptable and we want all loops to have at least one interruptable instruction
      (*self.bytecode).emit_ad(LuauOpcode::LOP_JUMPBACK, 0, 0);

      let end_label = (*self.bytecode).emit_label();

      self.patch_jump(stat as *mut AstNode, back_label, loop_label);
      self.patch_jumps(stat as *mut AstNode, &mut else_jump, end_label);

      self.patch_loop_jumps(stat as *mut AstNode, old_jumps, end_label, cont_label);
      self.loop_jumps.resize(
        old_jumps,
        LoopJump {
          r#type: Break,
          label: 0,
        },
      );

      self.loops.pop();
    }
  }
}
