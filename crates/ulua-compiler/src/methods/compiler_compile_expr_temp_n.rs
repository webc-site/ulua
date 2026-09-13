use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs,
    ast_node::AstNode,
  },
  rtti,
};
use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::records::{compile_error::CompileError, compiler::Compiler};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_temp_n(
    &mut self,
    node: *mut AstExpr,
    target: u8,
    target_count: u8,
    target_top: bool,
  ) {
    LUAU_ASSERT!(!target_top || u32::from(target) + u32::from(target_count) == self.reg_top);

    if target_count == 255 {
      let location = unsafe { (*node).base.location };
      CompileError::raise(
        &location,
        core::format_args!("Exceeded result count limit; simplify the code to compile"),
      );
    }

    let expr_call = unsafe { rtti::ast_node_as::<AstExprCall>(node as *mut AstNode) };
    if !expr_call.is_null() {
      unsafe { self.compile_expr_call(expr_call, target, target_count, target_top, false) };
      return;
    }

    let expr_varargs = unsafe { rtti::ast_node_as::<AstExprVarargs>(node as *mut AstNode) };
    if !expr_varargs.is_null() {
      self.compile_expr_varargs(expr_varargs, target, target_count, false);
      return;
    }

    unsafe { self.compile_expr_temp(node, target) };

    for i in 1..target_count {
      unsafe {
        let bytecode = &mut *self.bytecode;
        bytecode.emit_abc(LuauOpcode::LOP_LOADNIL, target.wrapping_add(i), 0, 0);
      }
    }
  }
}
