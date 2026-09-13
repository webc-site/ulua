use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_group::AstExprGroup,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::lint_duplicate_condition::LintDuplicateCondition;

impl LintDuplicateCondition {
  pub fn extract_op_chain(
    &mut self,
    conditions: &mut Vec<*mut AstExpr>,
    expr: *mut AstExpr,
    op: AstExprBinaryOp,
  ) {
    unsafe {
      let bin = ast_node_as::<AstExprBinary>(expr as *mut AstNode);
      if !bin.is_null() && (*bin).op == op {
        self.extract_op_chain(conditions, (*bin).left, op);
        self.extract_op_chain(conditions, (*bin).right, op);
        return;
      }

      let group = ast_node_as::<AstExprGroup>(expr as *mut AstNode);
      if !group.is_null() {
        self.extract_op_chain(conditions, (*group).expr, op);
        return;
      }
    }

    conditions.push(expr);
  }
}
