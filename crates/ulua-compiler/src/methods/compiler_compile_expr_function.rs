use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode};

use crate::records::{
  capture::Capture,
  compile_error::CompileError,
  compiler::{Compiler, K_MAX_AD_INDEX},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_function(&mut self, expr: *mut AstExprFunction, target: u8) {
    let f = self.functions.find(&expr).unwrap();
    let fid = f.id;
    let upvals = f.upvals.clone();
    let pid = unsafe { (*self.bytecode).add_child_function(fid) };
    if pid < 0 {
      unsafe {
        CompileError::raise(
          &(*expr).base.base.location,
          format_args!("Exceeded Closure limit"),
        );
      }
    }
    self.captures.clear();
    for uv in upvals {
      let reg = self.get_local_reg(uv);
      if reg >= 0 {
        let ul = self.variables.find(&uv);
        let immutable = ul.is_none_or(|ul| !ul.written);
        self.captures.push(Capture {
          r#type: if immutable {
            LuauCaptureType::LCT_VAL
          } else {
            LuauCaptureType::LCT_REF
          },
          data: reg as u8,
        });
      } else if let Some(uc) = self.locstants.find(&uv).copied() {
        let reg = unsafe { self.alloc_reg(expr as *mut _, 1) };
        unsafe { self.compile_expr_constant(expr as *mut _, &uc, reg) };
        self.captures.push(Capture {
          r#type: LuauCaptureType::LCT_VAL,
          data: reg,
        });
      } else {
        let uid = unsafe { self.get_upval(uv) };
        self.captures.push(Capture {
          r#type: LuauCaptureType::LCT_UPVAL,
          data: uid,
        });
      }
    }
    let mut shared = -1i16;
    if self.options.optimization_level >= 1 && self.should_share_closure(expr) && !self.setfenv_used
    {
      let cid = unsafe { (*self.bytecode).add_constant_closure(fid) };
      if (0..K_MAX_AD_INDEX).contains(&cid) {
        shared = cid as i16;
      }
    }
    unsafe {
      if shared >= 0 {
        (*self.bytecode).emit_ad(LuauOpcode::LOP_DUPCLOSURE, target, shared);
      } else {
        (*self.bytecode).emit_ad(LuauOpcode::LOP_NEWCLOSURE, target, pid);
      }
      for c in &self.captures {
        (*self.bytecode).emit_abc(LuauOpcode::LOP_CAPTURE, c.r#type as u8, c.data, 0);
      }
    }
  }
}
