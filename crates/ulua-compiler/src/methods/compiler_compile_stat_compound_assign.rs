use ulua_ast::records::{
  ast_expr_binary::AstExprBinaryOp, ast_stat_compound_assign::AstStatCompoundAssign,
};
use ulua_common::{
  FFlag,
  enums::{luau_bytecode_type::LBC_TYPE_NUMBER, luau_opcode::LuauOpcode},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{enums::kind::Kind, records::compiler::Compiler};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_stat_compound_assign(&mut self, stat: *mut AstStatCompoundAssign) {
    unsafe {
      let stat_ref = &*stat;
      let mut rs = self.reg_scope_compiler();
      let var = self.compile_l_value(stat_ref.var, &mut rs);
      let target = if var.kind == Kind::Local {
        var.reg
      } else {
        self.alloc_reg(stat as *mut _, 1)
      };

      match stat_ref.op {
        AstExprBinaryOp::Add
        | AstExprBinaryOp::Sub
        | AstExprBinaryOp::Mul
        | AstExprBinaryOp::Div
        | AstExprBinaryOp::FloorDiv
        | AstExprBinaryOp::Mod
        | AstExprBinaryOp::Pow => {
          if var.kind != Kind::Local {
            self.compile_l_value_use(&var, target, false, stat_ref.var);
          }
          let rc = self.get_constant_number(stat_ref.value);
          if (0..=255).contains(&rc) {
            (*self.bytecode).emit_abc(
              self.get_binary_op_arith(stat_ref.op, true),
              target,
              target,
              rc as u8,
            );
          } else {
            let rr = self.compile_expr_auto(stat_ref.value, &mut rs);
            (*self.bytecode).emit_abc(
              self.get_binary_op_arith(stat_ref.op, false),
              target,
              target,
              rr,
            );
            if var.kind != Kind::Local {
              self.hint_temporary_reg_type(stat_ref.var, target as i32, LBC_TYPE_NUMBER, 1);
            }
            self.hint_temporary_expr_reg_type(stat_ref.value, rr as i32, LBC_TYPE_NUMBER, 1);
          }
        }
        AstExprBinaryOp::Concat => {
          let mut args = vec![stat_ref.value];
          self.unroll_concats(&mut args);
          let regs = self.alloc_reg(stat as *mut _, (1 + args.len()) as u32);
          self.compile_l_value_use(&var, regs, false, stat_ref.var);
          for (i, &arg) in args.iter().enumerate() {
            if FFlag::LuauCompileConcatTargetTop.get() {
              self.compile_expr_temp_top(arg, regs + 1 + i as u8);
            } else {
              self.compile_expr_temp(arg, regs + 1 + i as u8);
            }
          }
          (*self.bytecode).emit_abc(
            LuauOpcode::LOP_CONCAT,
            target,
            regs,
            regs + args.len() as u8,
          );
        }
        _ => LUAU_ASSERT!(false),
      }

      if var.kind != Kind::Local {
        self.compile_assign(&var, target, stat_ref.var);
      }
    }
  }
}
