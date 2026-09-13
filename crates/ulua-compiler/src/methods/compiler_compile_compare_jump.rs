use core::mem::swap;

use ulua_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinaryOp};
use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_constant_folding::Type,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::{Compiler, K_GETIMPORT_FLAG},
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_compare_jump(&mut self, expr: *mut AstExprBinary, not_: bool) -> usize {
    unsafe {
      let expr_ref = &*expr;
      let mut left = expr_ref.left;
      let mut right = expr_ref.right;
      let is_eq =
        expr_ref.op == AstExprBinaryOp::CompareEq || expr_ref.op == AstExprBinaryOp::CompareNe;

      let mut operand_is_constant = self.is_constant(right);
      if is_eq && !operand_is_constant {
        operand_is_constant = self.is_constant(left);
        if operand_is_constant {
          swap(&mut left, &mut right);
        }
      }

      if operand_is_constant && (self.is_constant_vector(right) || self.is_constant_integer(right))
      {
        operand_is_constant = false;
      }

      let mut rs = self.reg_scope_compiler();
      let rl = self.compile_expr_auto(left, &mut rs);

      if is_eq && operand_is_constant {
        let cv = self.get_constant(right);
        LUAU_ASSERT!(cv.r#type != Type::Unknown);

        let (opc, cid_val) = match cv.r#type {
          Type::Nil => (LuauOpcode::LOP_JUMPXEQKNIL, 0),
          Type::Boolean => (LuauOpcode::LOP_JUMPXEQKB, cv.data.value_boolean as i32),
          Type::Number => (LuauOpcode::LOP_JUMPXEQKN, self.get_constant_index(right)),
          Type::String => (LuauOpcode::LOP_JUMPXEQKS, self.get_constant_index(right)),
          _ => {
            LUAU_ASSERT!(false);
            (LuauOpcode::LOP_NOP, 0)
          }
        };

        if cid_val < 0 {
          CompileError::raise(
            &expr_ref.base.base.location,
            format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
          );
        }

        let jump_label = (*self.bytecode).emit_label();
        let flip = if (expr_ref.op == AstExprBinaryOp::CompareEq) == not_ {
          K_GETIMPORT_FLAG
        } else {
          0
        };

        (*self.bytecode).emit_ad(opc, rl, 0);
        (*self.bytecode).emit_aux((cid_val as u32) | flip);

        jump_label
      } else {
        let opc = self.get_jump_op_compare(expr_ref.op, not_);
        let rr = self.compile_expr_auto(right, &mut rs);
        let jump_label = (*self.bytecode).emit_label();

        if expr_ref.op == AstExprBinaryOp::CompareGt || expr_ref.op == AstExprBinaryOp::CompareGe {
          (*self.bytecode).emit_ad(opc, rr, 0);
          (*self.bytecode).emit_aux(rl as u32);
        } else {
          (*self.bytecode).emit_ad(opc, rl, 0);
          (*self.bytecode).emit_aux(rr as u32);
        }
        jump_label
      }
    }
  }
}
