use core::ptr::{null, null_mut};

use ulua_ast::records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_repeat::AstStatRepeat};
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::type_compiler::Type,
  records::{compiler::Compiler, r#loop::Loop, loop_jump::LoopJump, reg_scope::RegScope},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_stat_repeat(&mut self, stat: *mut AstStatRepeat) {
    unsafe {
      let stat_ref = &*stat;

      let old_jumps = self.loop_jumps.len();
      let old_locals = self.local_stack.len();

      self.loops.push(Loop {
        local_offset: old_locals,
        local_offset_continue: old_locals,
        continue_used: null_mut(),
      });
      self.has_loops = true;

      let loop_label = (*self.bytecode).emit_label();

      let body = stat_ref.body;

      let _rs = RegScope {
        self_: self as *mut Compiler,
        old_top: self.reg_top,
      };

      let mut continue_validated = false;
      let mut condition_locals = 0;

      for (i, &body_stat) in (*body).body.iter().enumerate() {
        self.compile_stat(body_stat);

        self.loops.last_mut().unwrap().local_offset_continue = self.local_stack.len();

        if !self.loops.last().unwrap().continue_used.is_null() && !continue_validated {
          self.validate_continue_until(
            self.loops.last().unwrap().continue_used as *mut AstStat,
            stat_ref.condition,
            body,
            i + 1,
          );
          continue_validated = true;
          condition_locals = self.local_stack.len();
        }
      }

      if continue_validated {
        self.set_debug_line_end(*(*body).body.data.add((*body).body.size - 1) as *mut AstNode);

        self.close_locals(condition_locals);
        self.pop_locals(condition_locals);
      }

      let cont_label = (*self.bytecode).emit_label();

      let end_label;

      self.set_debug_line_ast_node(stat_ref.condition as *mut AstNode);

      if self.is_constant_true(stat_ref.condition) {
        self.close_locals(old_locals);

        end_label = (*self.bytecode).emit_label();
      } else {
        let mut skip_jump = Vec::new();
        self.compile_condition_value(stat_ref.condition, null(), &mut skip_jump, true);

        self.close_locals(old_locals);

        let back_label = (*self.bytecode).emit_label();

        (*self.bytecode).emit_ad(LuauOpcode::LOP_JUMPBACK, 0, 0);

        let skip_label = (*self.bytecode).emit_label();

        self.close_locals(old_locals);

        end_label = (*self.bytecode).emit_label();

        self.patch_jump(stat as *mut AstNode, back_label, loop_label);
        self.patch_jumps(stat as *mut AstNode, &mut skip_jump, skip_label);
      }

      self.pop_locals(old_locals);

      self.patch_loop_jumps(stat as *mut AstNode, old_jumps, end_label, cont_label);
      self.loop_jumps.resize(
        old_jumps,
        LoopJump {
          r#type: Type::Break,
          label: 0,
        },
      );

      self.loops.pop();
    }
  }
}
