use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{functions::get_builtin_info::get_builtin_info, records::compiler::Compiler};

impl Compiler {
  pub fn is_expr_mult_ret(&mut self, node: *mut AstExpr) -> bool {
    unsafe {
      let expr = ast_node_as::<AstExprCall>(node as *mut AstNode);
      if expr.is_null() {
        return !ast_node_as::<AstExprVarargs>(node as *mut AstNode).is_null();
      }

      if self.options.optimization_level <= 1 {
        return true;
      }

      if self.is_constant(expr as *mut AstExpr) {
        return false;
      }

      if self.options.optimization_level >= 2
        && let Some(bfid) = self.builtins.find(&expr)
        && *bfid != LuauBuiltinFunction::LBF_NONE as i32
      {
        return get_builtin_info(*bfid).results != 1;
      }

      let func = self.get_function_expr((*expr).func);
      let fi = if func.is_null() {
        None
      } else {
        self.functions.find(&func)
      };

      !fi.is_some_and(|fi| fi.returns_one)
    }
  }
}
