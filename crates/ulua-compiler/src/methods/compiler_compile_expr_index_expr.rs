use ulua_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use ulua_bytecode::methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash;
use ulua_common::enums::{
  luau_bytecode_type::{LBC_TYPE_NUMBER, LBC_TYPE_TABLE},
  luau_opcode::LuauOpcode,
};

use crate::{
  functions::sref_compiler_alt_c::sref_ast_array_c_char,
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
    constant::Constant,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_index_expr(&mut self, expr: *mut AstExprIndexExpr, target: u8) {
    unsafe {
      let expr_ref = &*expr;
      let mut rs = self.reg_scope_compiler();
      let cv = self.get_constant(expr_ref.index);

      match cv {
        // 整数下标 1..=256 走 GETTABLEN 快路径
        Constant::Number(n) if (1.0..=256.0).contains(&n) && (n as i32 as f64) == n => {
          let i = (n as i32 - 1) as u8;
          let rt = self.compile_expr_auto(expr_ref.expr, &mut rs);
          self.set_debug_line_location(&(*expr_ref.index).base.location);
          (*self.bytecode).emit_abc(LuauOpcode::LOP_GETTABLEN, target, rt, i);
          self.hint_temporary_expr_reg_type(expr_ref.expr, rt as i32, LBC_TYPE_TABLE, 1);
        }
        Constant::Str(_) => {
          let iname = sref_ast_array_c_char(cv.get_string());
          let cid = (*self.bytecode).add_constant_string(iname);
          if cid < 0 {
            CompileError::raise(
              &expr_ref.base.base.location,
              format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
            );
          }
          let rt = self.compile_expr_auto(expr_ref.expr, &mut rs);
          self.set_debug_line_location(&(*expr_ref.index).base.location);
          (*self.bytecode).emit_abc(
            LuauOpcode::LOP_GETTABLEKS,
            target,
            rt,
            bytecode_builder_get_string_hash(iname) as u8,
          );
          (*self.bytecode).emit_aux(cid as u32);
          self.hint_temporary_expr_reg_type(expr_ref.expr, rt as i32, LBC_TYPE_TABLE, 2);
        }
        _ => {
          let rt = self.compile_expr_auto(expr_ref.expr, &mut rs);
          let ri = self.compile_expr_auto(expr_ref.index, &mut rs);
          (*self.bytecode).emit_abc(LuauOpcode::LOP_GETTABLE, target, rt, ri);
          self.hint_temporary_expr_reg_type(expr_ref.expr, rt as i32, LBC_TYPE_TABLE, 1);
          self.hint_temporary_expr_reg_type(expr_ref.index, ri as i32, LBC_TYPE_NUMBER, 1);
        }
      }
    }
  }
}
