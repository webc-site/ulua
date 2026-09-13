use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_expr_if_else::AstExprIfElse, ast_node::AstNode},
  rtti::ast_node_as,
  visit::ast_expr_visit,
};

use crate::records::lint_duplicate_condition::LintDuplicateCondition;

impl LintDuplicateCondition {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_if_else(&mut self, expr: *mut AstExprIfElse) -> bool {
    unsafe {
      if expr.is_null()
        || ast_node_as::<AstExprIfElse>((*expr).false_expr as *mut AstNode).is_null()
      {
        return true;
      }

      let mut conditions = Vec::with_capacity(2);
      let mut head = expr;

      while !head.is_null() {
        ast_expr_visit((*head).condition, self);
        ast_expr_visit((*head).true_expr, self);

        conditions.push((*head).condition);

        let next = ast_node_as::<AstExprIfElse>((*head).false_expr as *mut AstNode);
        if !next.is_null() {
          head = next;
          continue;
        }

        ast_expr_visit((*head).false_expr, self);
        break;
      }

      self.detect_duplicates(&conditions);
    }

    false
  }
}
