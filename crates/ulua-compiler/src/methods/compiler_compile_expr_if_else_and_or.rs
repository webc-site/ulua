use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_if_else_and_or(
    &mut self,
    and_: bool,
    creg: u8,
    other: *mut AstExpr,
    target: u8,
  ) {
    let cid = unsafe { self.get_constant_index(other) };
    unsafe {
      if (0..=255).contains(&cid) {
        (*self.bytecode).emit_abc(
          if and_ {
            LuauOpcode::LOP_ANDK
          } else {
            LuauOpcode::LOP_ORK
          },
          target,
          creg,
          cid as u8,
        );
      } else {
        let mut rs = self.reg_scope_compiler();
        let oreg = self.compile_expr_auto(other, &mut rs);
        (*self.bytecode).emit_abc(
          if and_ {
            LuauOpcode::LOP_AND
          } else {
            LuauOpcode::LOP_OR
          },
          target,
          creg,
          oreg,
        );
      }
    }
  }
}
