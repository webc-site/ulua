use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::type_constant_folding::Type,
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
  pub unsafe fn compile_expr_constant(&mut self, node: *mut AstExpr, cv: &Constant, target: u8) {
    match cv.r#type {
      Type::Nil => unsafe {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADNIL, target, 0, 0);
      },
      Type::Boolean => {
        let b = unsafe { cv.data.value_boolean };
        unsafe {
          (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADB, target, b as u8, 0);
        }
      }
      Type::Number => {
        let d = unsafe { cv.data.value_number };

        let fits_i16 = d >= (i16::MIN as f64)
          && d <= (i16::MAX as f64)
          && (d as i16 as f64) == d
          && !(d == 0.0 && d.is_sign_negative());

        if fits_i16 {
          unsafe {
            (*self.bytecode).emit_ad(LuauOpcode::LOP_LOADN, target, d as i16);
          }
        } else {
          let cid = unsafe { (*self.bytecode).add_constant_number(d) };
          if cid < 0 {
            let location = unsafe { (*node).base.location };
            CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
          }
          self.emit_load_k(target, cid);
        }
      }
      Type::Integer => {
        let l = unsafe { cv.data.value_integer64 };
        let cid = unsafe { (*self.bytecode).add_constant_integer(l) };
        if cid < 0 {
          let location = unsafe { (*node).base.location };
          CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
        }
        self.emit_load_k(target, cid);
      }
      Type::Vector => {
        let x = unsafe { cv.data.value_vector[0] };
        let y = unsafe { cv.data.value_vector[1] };
        let z = unsafe { cv.data.value_vector[2] };
        let w = unsafe { cv.data.value_vector[3] };

        let cid = unsafe { (*self.bytecode).add_constant_vector(x, y, z, w) };
        if cid < 0 {
          let location = unsafe { (*node).base.location };
          CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
        }
        self.emit_load_k(target, cid);
      }
      Type::String => {
        let s = cv.get_string();
        let cid = unsafe { (*self.bytecode).add_constant_string(sref_ast_array_c_char(s)) };
        if cid < 0 {
          let location = unsafe { (*node).base.location };
          CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
        }
        self.emit_load_k(target, cid);
      }
      _ => {
        ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
      }
    }
  }
}
