use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

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
  pub unsafe fn compile_expr_constant(&mut self, node: *mut AstExpr, cv: &Constant, target: u8) {
    match *cv {
      Constant::Nil => unsafe {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADNIL, target, 0, 0);
      },
      Constant::Boolean(b) => unsafe {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_LOADB, target, b as u8, 0);
      },
      Constant::Number(d) => {
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
      Constant::Integer(l) => {
        let cid = unsafe { (*self.bytecode).add_constant_integer(l) };
        if cid < 0 {
          let location = unsafe { (*node).base.location };
          CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
        }
        self.emit_load_k(target, cid);
      }
      Constant::Vector([x, y, z, w]) => {
        let cid = unsafe { (*self.bytecode).add_constant_vector(x, y, z, w) };
        if cid < 0 {
          let location = unsafe { (*node).base.location };
          CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
        }
        self.emit_load_k(target, cid);
      }
      Constant::Str(_) => {
        let s = cv.get_string();
        let cid = unsafe { (*self.bytecode).add_constant_string(sref_ast_array_c_char(s)) };
        if cid < 0 {
          let location = unsafe { (*node).base.location };
          CompileError::raise(&location, format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"));
        }
        self.emit_load_k(target, cid);
      }
      // 仅 Unknown / Table 会到达
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
