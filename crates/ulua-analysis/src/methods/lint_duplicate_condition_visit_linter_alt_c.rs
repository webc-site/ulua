use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_node::AstNode,
  },
  rtti::ast_node_as,
  visit::ast_expr_visit,
};

use crate::records::lint_duplicate_condition::LintDuplicateCondition;

impl LintDuplicateCondition {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_binary(&mut self, expr: *mut AstExprBinary) -> bool {
    unsafe {
      if expr.is_null() || ((*expr).op != AstExprBinaryOp::And && (*expr).op != AstExprBinaryOp::Or)
      {
        return true;
      }

      if (*expr).op == AstExprBinaryOp::Or {
        let la = ast_node_as::<AstExprBinary>((*expr).left as *mut AstNode);

        if !la.is_null() && (*la).op == AstExprBinaryOp::And {
          let lb = ast_node_as::<AstExprBinary>((*la).left as *mut AstNode);
          let rb = ast_node_as::<AstExprBinary>((*la).right as *mut AstNode);

          if !(lb.is_null() || (*lb).op != AstExprBinaryOp::And)
            || !(rb.is_null() || (*rb).op != AstExprBinaryOp::And)
          {
            // This is an and-chain longer than two; continue with duplicate detection.
          } else {
            ast_expr_visit((*la).left, self);
            ast_expr_visit((*la).right, self);
            ast_expr_visit((*expr).right, self);
            return false;
          }
        }
      }

      let mut conditions = Vec::with_capacity(2);
      self.extract_op_chain(&mut conditions, expr as *mut _, (*expr).op);
      self.detect_duplicates(&conditions);
    }

    false
  }
}
