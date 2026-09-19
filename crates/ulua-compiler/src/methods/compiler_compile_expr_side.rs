use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, ast_expr_varargs::AstExprVarargs,
    ast_node::AstNode,
  },
  rtti,
};

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn compile_expr_side(&mut self, node: *mut AstExpr) {
    unsafe {
      let ast_node = &*(node as *mut AstNode);
      if rtti::ast_node_is::<AstExprLocal>(ast_node)
        || rtti::ast_node_is::<AstExprGlobal>(ast_node)
        || rtti::ast_node_is::<AstExprVarargs>(ast_node)
        || rtti::ast_node_is::<AstExprFunction>(ast_node)
        || self.is_constant(node)
      {
        return;
      }

      if rtti::ast_node_as::<AstExprCall>(node as *mut AstNode).is_null() {
        (*self.bytecode)
          .add_debug_remark(format_args!("expression only compiled for side effects"));
      }

      let mut rsi = self.reg_scope_compiler();
      self.compile_expr_auto(node, &mut rsi);
    }
  }
}
