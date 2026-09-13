use core::ptr::null_mut;

use ulua_ast::records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_for::AstStatFor};
use ulua_common::FFlag;

use crate::{
  enums::{type_compiler::Type as LoopJumpType, type_constant_folding::Type},
  functions::{
    undo_changes_constant_folding::undo_changes_expr,
    undo_changes_constant_folding_alt_b::undo_changes_local,
  },
  records::{
    compiler::Compiler,
    constant::{Constant, ConstantData},
    r#loop::Loop,
    loop_jump::LoopJump,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_unrolled_for(
    &mut self,
    stat: *mut AstStatFor,
    trip_count: i32,
    from: f64,
    step: f64,
  ) {
    unsafe {
      let stat_ref = &*stat;
      let old_locals = self.local_stack.len();
      let old_jumps = self.loop_jumps.len();

      self.loops.push(Loop {
        local_offset: old_locals,
        local_offset_continue: old_locals,
        continue_used: null_mut(),
      });

      let record_changes =
        FFlag::LuauCompilePropagateTableProps2.get() && FFlag::LuauCompileFoldOptimize.get();

      if record_changes {
        self.expr_changes.clear();
        self.local_changes.clear();
      }

      for iv in 0..trip_count {
        *self.locstants.get_or_insert(stat_ref.var) = Constant {
          r#type: Type::Number,
          string_length: 0,
          data: ConstantData {
            value_number: from + f64::from(iv) * step,
          },
        };

        self.fold_constants(stat_ref.body as *mut AstNode, record_changes && iv == 0);

        let iter_jumps = self.loop_jumps.len();
        self.compile_stat(stat_ref.body as *mut AstStat);

        let cont_label = (*self.bytecode).emit_label();

        for i in iter_jumps..self.loop_jumps.len() {
          if self.loop_jumps[i].r#type == LoopJumpType::Continue {
            self.patch_jump(stat as *mut AstNode, self.loop_jumps[i].label, cont_label);
          }
        }
      }

      let end_label = (*self.bytecode).emit_label();

      for i in old_jumps..self.loop_jumps.len() {
        if self.loop_jumps[i].r#type == LoopJumpType::Break {
          self.patch_jump(stat as *mut AstNode, self.loop_jumps[i].label, end_label);
        }
      }

      self.loop_jumps.resize(
        old_jumps,
        LoopJump {
          r#type: LoopJumpType::Break,
          label: 0,
        },
      );

      self.loops.pop();

      self.locstants.get_or_insert(stat_ref.var).r#type = Type::Unknown;

      if record_changes {
        undo_changes_expr(&mut self.constants, &self.expr_changes);
        undo_changes_local(&mut self.locstants, &self.local_changes);
      } else {
        self.fold_constants(stat_ref.body as *mut AstNode, false);
      }
    }
  }
}
