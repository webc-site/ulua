use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_local::AstExprLocal,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::table_mutation_tracker::TableMutationTracker;

impl TableMutationTracker {
  pub fn mark_escaped_impl(&mut self, mut expr: *mut AstExpr) {
    loop {
      if expr.is_null() {
        return;
      }

      let expr_ptr = expr as *mut AstNode;

      let expr_local = unsafe { ast_node_as::<AstExprLocal>(expr_ptr) };
      if !expr_local.is_null() {
        unsafe {
          self.escaped.insert((*expr_local).local);
        }
        return;
      }

      let expr_group = unsafe { ast_node_as::<AstExprGroup>(expr_ptr) };
      if !expr_group.is_null() {
        expr = unsafe { (*expr_group).expr };
        continue;
      }

      let expr_assertion = unsafe { ast_node_as::<AstExprTypeAssertion>(expr_ptr) };
      if !expr_assertion.is_null() {
        expr = unsafe { (*expr_assertion).expr };
        continue;
      }

      let expr_instantiate = unsafe { ast_node_as::<AstExprInstantiate>(expr_ptr) };
      if !expr_instantiate.is_null() {
        expr = unsafe { (*expr_instantiate).expr };
        continue;
      }

      let expr_if_else = unsafe { ast_node_as::<AstExprIfElse>(expr_ptr) };
      if !expr_if_else.is_null() {
        self.mark_escaped_impl(unsafe { (*expr_if_else).true_expr });
        expr = unsafe { (*expr_if_else).false_expr };
        continue;
      }

      let bin_expr = unsafe { ast_node_as::<AstExprBinary>(expr_ptr) };
      if !bin_expr.is_null() {
        let op = unsafe { (*bin_expr).op };
        if op == AstExprBinaryOp::And || op == AstExprBinaryOp::Or {
          self.mark_escaped_impl(unsafe { (*bin_expr).left });
          expr = unsafe { (*bin_expr).right };
          continue;
        } else {
          return;
        }
      }

      return;
    }
  }
}

pub fn table_mutation_tracker_mark_escaped(tracker: &mut TableMutationTracker, expr: *mut AstExpr) {
  tracker.mark_escaped_impl(expr);
}
