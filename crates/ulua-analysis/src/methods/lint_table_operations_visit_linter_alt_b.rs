//! C++ `LintTableOperations::visit(AstExprCall*)` (`Analysis/src/Linter.cpp:2618`).

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::lint_table_operations::LintTableOperations;
impl LintTableOperations {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
    unsafe {
      let func_expr = (*node).func;
      let func_global = ast_node_as::<AstExprGlobal>(func_expr as *mut AstNode);

      if !func_global.is_null() {
        if (*func_global).name == "ipairs" && (*node).args.len() == 1 {
          let arg0 = (*node).args.as_slice()[0];
          self.check_indexer(&*(node as *mut AstExpr), &*arg0, "ipairs");
        }
      } else {
        let func_index = ast_node_as::<AstExprIndexName>(func_expr as *mut AstNode);
        if !func_index.is_null() {
          let tablib = ast_node_as::<AstExprGlobal>((*func_index).expr as *mut AstNode);
          if !tablib.is_null() && (*tablib).name == "table" {
            self.check_table_call(node, func_index);
          }
        }
      }
    }

    true
  }
}
