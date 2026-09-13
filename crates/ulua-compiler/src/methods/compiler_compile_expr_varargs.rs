use ulua_ast::records::{ast_expr_varargs::AstExprVarargs, ast_node::AstNode};
use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn compile_expr_varargs(
    &mut self,
    expr: *mut AstExprVarargs,
    target: u8,
    target_count: u8,
    mult_ret: bool,
  ) {
    LUAU_ASSERT!(target_count < 255);
    LUAU_ASSERT!(!mult_ret || u32::from(target) + u32::from(target_count) == self.reg_top);

    unsafe { self.set_debug_line_ast_node(expr as *mut AstNode) };

    unsafe {
      let bytecode = &mut *self.bytecode;
      bytecode.emit_abc(
        LuauOpcode::LOP_GETVARARGS,
        target,
        if mult_ret {
          0
        } else {
          target_count.wrapping_add(1)
        },
        0,
      );
    }
  }
}
