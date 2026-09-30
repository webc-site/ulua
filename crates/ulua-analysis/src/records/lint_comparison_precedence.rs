use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  functions::to_string_ast::to_str_binary as to_str,
  records::{
    ast_attr::AstAttr,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_unary::AstExprUnaryOp,
    ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintComparisonPrecedence<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintComparisonPrecedence<'ctx> {
  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    self.visit_ast_expr_binary(node)
  }

  // visit_node 沿用 trait 默认实现（返回 true）
  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_comparison_precedence_is_not.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  pub fn is_not(&self, node: &AstExpr) -> bool {
    match node.as_expr_ref() {
      AstExprRef::Unary(unary) => unary.op == AstExprUnaryOp::Not,
      _ => false,
    }
  }
}

// —— 原 methods/lint_comparison_precedence_process.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  lint_stat_process!(LintComparisonPrecedence);
}

// —— 原 methods/lint_comparison_precedence_visit.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  pub(crate) fn visit_ast_expr_binary(&mut self, node: &AstExprBinary) -> bool {
    let op = node.op;
    if !op.is_comparison() {
      return true;
    }
    let left = node.left.get();
    let right = node.right.get();
    let left_is_not = self.is_not(left);
    let right_is_not = self.is_not(right);
    if left_is_not && !right_is_not {
      let op_str = to_str(op);
      if op.is_equality() {
        let opposite = if op == AstExprBinaryOp::CompareEq {
          "~="
        } else {
          "=="
        };
        emit_warning(
          self.context.get(),
          Code::ComparisonPrecedence,
          node.base.base.location,
          format_args!(
            "not X {} Y is equivalent to (not X) {} Y; consider using X {} Y, or add parentheses to silence",
            op_str, op_str, opposite
          ),
        );
      } else {
        emit_warning(
          self.context.get(),
          Code::ComparisonPrecedence,
          node.base.base.location,
          format_args!(
            "not X {} Y is equivalent to (not X) {} Y; add parentheses to silence",
            op_str, op_str
          ),
        );
      }
    } else if let AstExprRef::Binary(left_binary) = left.as_expr_ref() {
      let left_op = left_binary.op;
      if left_op.is_comparison() {
        let lop_str = to_str(left_op);
        let rop_str = to_str(op);
        if left_op.is_equality() || op.is_equality() {
          emit_warning(
            self.context.get(),
            Code::ComparisonPrecedence,
            node.base.base.location,
            format_args!(
              "X {} Y {} Z is equivalent to (X {} Y) {} Z; add parentheses to silence",
              lop_str, rop_str, lop_str, rop_str
            ),
          );
        } else {
          emit_warning(
            self.context.get(),
            Code::ComparisonPrecedence,
            node.base.base.location,
            format_args!(
              "X {} Y {} Z is equivalent to (X {} Y) {} Z; did you mean X {} Y and Y {} Z?",
              lop_str, rop_str, lop_str, rop_str, lop_str, rop_str
            ),
          );
        }
      }
    }
    true
  }
}
