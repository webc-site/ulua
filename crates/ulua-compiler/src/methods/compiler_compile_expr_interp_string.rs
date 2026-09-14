use alloc::vec::Vec;
use core::ffi::c_char;

use ulua_ast::records::{
  ast_array::AstArray, ast_expr_interp_string::AstExprInterpString, ast_name::AstName,
};
use ulua_bytecode::methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash;
use ulua_common::{FFlag, enums::luau_opcode::LuauOpcode};

use crate::{
  enums::type_constant_folding::Type,
  functions::{
    escape_and_append::escape_and_append, sref_compiler::sref_ast_name,
    sref_compiler_alt_c::sref_ast_array_c_char,
  },
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
  },
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn compile_expr_interp_string(
    &mut self,
    expr: *mut AstExprInterpString,
    target: u8,
    target_temp: bool,
  ) {
    unsafe {
      let expr_ref = &*expr;
      let mut format_capacity = 0;
      for string in expr_ref.strings.iter() {
        format_capacity += string.size + (*string).iter().filter(|&&c| c == b'%' as c_char).count();
      }

      let mut skipped_sub_expr = 0;
      for &sub_expr in expr_ref.expressions.iter() {
        if let Some(c) = self.constants.find(&sub_expr) {
          if c.r#type == Type::String {
            format_capacity += c.string_length as usize
              + c
                .get_string()
                .iter()
                .filter(|&&c| c == b'%' as c_char)
                .count();
            skipped_sub_expr += 1;
          } else {
            format_capacity += 2;
          }
        } else {
          format_capacity += 2;
        }
      }

      let mut format_string = Vec::with_capacity(format_capacity);
      let expressions = expr_ref.expressions.as_slice();
      for (i, string) in expr_ref.strings.iter().enumerate() {
        escape_and_append(&mut format_string, string.as_bytes());
        if let Some(&sub_expr) = expressions.get(i) {
          if let Some(c) = self.constants.find(&sub_expr)
            && c.r#type == Type::String
          {
            escape_and_append(&mut format_string, c.get_string_bytes());
          } else {
            format_string.extend_from_slice(b"%*");
          }
        }
      }

      let format_string_index = if format_string.is_empty() {
        let interned = (*self.names).get_or_add_str("");
        (*self.bytecode).add_constant_string(sref_ast_name(interned))
      } else {
        let interned = (*self.names).get_or_add_slice(&format_string);
        let format_string_array = AstArray {
          data: interned.value as *mut c_char,
          size: format_string.len(),
        };
        (*self.bytecode).add_constant_string(sref_ast_array_c_char(format_string_array))
      };

      if format_string_index < 0 {
        CompileError::raise(
          &expr_ref.base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }

      let _rs = self.reg_scope_compiler();
      let reg_count = 2 + expr_ref.expressions.size - skipped_sub_expr;
      let target_top = FFlag::LuauCompileStringInterpTargetTop.get()
        && target_temp
        && target as u32 == self.reg_top - 1;
      let base_reg = if target_top {
        self.alloc_reg(expr as *mut _, (reg_count - 1) as u32) - 1
      } else {
        self.alloc_reg(expr as *mut _, reg_count as u32)
      };

      self.emit_load_k(base_reg, format_string_index);

      let mut skipped = 0;
      for (i, &sub_expr) in expr_ref.expressions.iter().enumerate() {
        if self
          .constants
          .find(&sub_expr)
          .is_none_or(|c| c.r#type != Type::String)
        {
          self.compile_expr_temp_top(sub_expr, base_reg + 2 + i as u8 - skipped as u8);
        } else {
          skipped += 1;
        }
      }

      let format_method = sref_ast_name(AstName::from_c_str(c"format"));
      let format_method_index = (*self.bytecode).add_constant_string(format_method);
      if format_method_index < 0 {
        CompileError::raise(
          &expr_ref.base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }

      (*self.bytecode).emit_abc(
        LuauOpcode::LOP_NAMECALL,
        base_reg,
        base_reg,
        bytecode_builder_get_string_hash(format_method) as u8,
      );
      (*self.bytecode).emit_aux(format_method_index as u32);
      (*self.bytecode).emit_abc(
        LuauOpcode::LOP_CALL,
        base_reg,
        (expr_ref.expressions.size + 2 - skipped_sub_expr) as u8,
        2,
      );
      if target != base_reg {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_MOVE, target, base_reg, 0);
      }
    }
  }
}
