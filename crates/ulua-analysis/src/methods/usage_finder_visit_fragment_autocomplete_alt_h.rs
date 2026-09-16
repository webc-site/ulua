use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_node::AstNode, ast_stat_function::AstStatFunction,
  },
  rtti::ast_node_as,
};

use crate::records::usage_finder::UsageFinder;

impl UsageFinder {
  /// # Safety
  /// 调用方须保证 `function` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, function: *mut AstStatFunction) -> bool {
    let function_ref = unsafe { &*function };

    let name_expr = function_ref.name;
    if !name_expr.is_null() {
      let global = unsafe { ast_node_as::<AstExprGlobal>(name_expr as *mut AstNode) };

      if !global.is_null() {
        let global_name = unsafe { (*global).name };
        self.global_functions_referenced.push(global_name);
      }
    }

    true
  }
}
