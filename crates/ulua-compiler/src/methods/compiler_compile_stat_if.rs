use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_if::AstStatIf,
  },
  rtti::ast_node_as,
};
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::type_compiler::Type,
  records::{compiler::Compiler, loop_jump::LoopJump},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_stat_if(&mut self, stat: *mut AstStatIf) {
    unsafe {
      let stat = &*stat;

      if self.is_constant_false(stat.condition) {
        if !stat.elsebody.is_null() {
          self.compile_stat(stat.elsebody);
        }
        return;
      }

      let cand = ast_node_as::<AstExprBinary>(stat.condition as *mut AstNode);
      if !cand.is_null() {
        let cand = &*cand;
        if cand.op == AstExprBinaryOp::And && self.is_constant_false(cand.right) {
          self.compile_expr_side(cand.left);
          if !stat.elsebody.is_null() {
            self.compile_stat(stat.elsebody);
          }
          return;
        }
      }

      if stat.elsebody.is_null()
        && self.is_stat_break(stat.thenbody as *mut AstStat)
        && !self.are_locals_captured(self.loops.last().unwrap().local_offset)
      {
        let mut else_jump = Vec::new();
        self.compile_condition_value(stat.condition, null(), &mut else_jump, true);
        for jump in else_jump {
          self.loop_jumps.push(LoopJump {
            r#type: Type::Break,
            label: jump,
          });
        }
        return;
      }

      let continue_statement = self.extract_stat_continue(stat.thenbody);
      if stat.elsebody.is_null()
        && !continue_statement.is_null()
        && !self.are_locals_captured(self.loops.last().unwrap().local_offset_continue)
      {
        if self.loops.last().unwrap().continue_used.is_null() {
          self.loops.last_mut().unwrap().continue_used = continue_statement;
        }
        let mut else_jump = Vec::new();
        self.compile_condition_value(stat.condition, null(), &mut else_jump, true);
        for jump in else_jump {
          self.loop_jumps.push(LoopJump {
            r#type: Type::Continue,
            label: jump,
          });
        }
        return;
      }

      let mut else_jump = Vec::new();
      self.compile_condition_value(stat.condition, null(), &mut else_jump, false);
      self.compile_stat(stat.thenbody as *mut AstStat);

      if !stat.elsebody.is_null() && !else_jump.is_empty() {
        if self.always_terminates(stat.thenbody as *mut AstStat) {
          let else_label = (*self.bytecode).emit_label();
          self.compile_stat(stat.elsebody);
          self.patch_jumps(
            stat as *const AstStatIf as *mut AstNode,
            &mut else_jump,
            else_label,
          );
        } else {
          let then_label = (*self.bytecode).emit_label();
          (*self.bytecode).emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
          let else_label = (*self.bytecode).emit_label();
          self.compile_stat(stat.elsebody);
          let end_label = (*self.bytecode).emit_label();
          self.patch_jumps(
            stat as *const AstStatIf as *mut AstNode,
            &mut else_jump,
            else_label,
          );
          self.patch_jump(
            stat as *const AstStatIf as *mut AstNode,
            then_label,
            end_label,
          );
        }
      } else {
        let end_label = (*self.bytecode).emit_label();
        self.patch_jumps(
          stat as *const AstStatIf as *mut AstNode,
          &mut else_jump,
          end_label,
        );
      }
    }
  }
}
