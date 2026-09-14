use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs},
  rtti,
};

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_temp_mult_ret(&mut self, node: *mut AstExpr, target: u8) -> bool {
    unsafe {
      let expr = rtti::ast_node_as::<AstExprCall>(node as *mut _);
      if !expr.is_null() {
        if self.options.optimization_level >= 2 && !self.is_expr_mult_ret(node) {
          self.compile_expr_temp(node, target);
          return false;
        }
        let _rs = self.reg_scope_compiler_i32(target as u32);
        self.compile_expr_call(expr, target, 0, true, true);
        true
      } else {
        let expr = rtti::ast_node_as::<AstExprVarargs>(node as *mut _);
        if !expr.is_null() {
          let _rs = self.reg_scope_compiler_i32(target as u32);
          self.compile_expr_varargs(expr, target, 0, true);
          true
        } else {
          self.compile_expr_temp(node, target);
          false
        }
      }
    }
  }
}
