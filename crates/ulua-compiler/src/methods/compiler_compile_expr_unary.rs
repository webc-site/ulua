use ulua_ast::{
  records::{
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  },
  rtti,
};
use ulua_common::FFlag;

use crate::records::{
  compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
  compiler::Compiler,
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_unary(&mut self, expr: *mut AstExprUnary, target: u8) {
    unsafe {
      let expr_ref = &*expr;
      let mut rs = self.reg_scope_compiler();

      if FFlag::LuauIntegerType2.get() && expr_ref.op == AstExprUnaryOp::Minus {
        let cint = rtti::ast_node_as::<AstExprConstantInteger>(expr_ref.expr as *mut _);
        if !cint.is_null() {
          // Two's-complement negation `~v + 1 == -v`. The `+ 1` MUST
          // wrap (C semantics): for v == 0 it wraps `u64::MAX -> 0`, and
          // for v == i64::MIN it wraps back to i64::MIN — both correct.
          // A checked `+` panics on those under the fuzz build's
          // overflow-checks (found by the compile fuzzer on `-<int 0>`).
          let cid =
            (*self.bytecode).add_constant_integer((!((*cint).value as u64)).wrapping_add(1) as i64);
          if cid < 0 {
            CompileError::raise(
              &expr_ref.base.base.location,
              format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
            );
          }
          self.emit_load_k(target, cid);
          return;
        }
      }

      let re = self.compile_expr_auto(expr_ref.expr, &mut rs);
      (*self.bytecode).emit_abc(self.get_unary_op(expr_ref.op), target, re, 0);
    }
  }
}
