use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_attr::AstAttr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit_ref,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintMisleadingAndOr<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintMisleadingAndOr<'ctx> {
  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    self.visit_ast_expr_binary(node)
  }

  // visit_node 沿用 trait 默认实现（返回 true）
  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_misleading_and_or_process.rs ——
impl<'ctx> LintMisleadingAndOr<'ctx> {
  lint_stat_process!(LintMisleadingAndOr);
}

// —— 原 methods/lint_misleading_and_or_visit.rs ——
impl<'ctx> LintMisleadingAndOr<'ctx> {
  pub(crate) fn visit_ast_expr_binary(&mut self, node: &AstExprBinary) -> bool {
    if node.op != AstExprBinaryOp::Or {
      return true;
    }
    let AstExprRef::Binary(and_) = node.left.get().as_expr_ref() else {
      return true;
    };
    if and_.op != AstExprBinaryOp::And {
      return true;
    }
    let alt = match and_.right.get().as_expr_ref() {
      AstExprRef::ConstantNil(_) => Some("nil"),
      AstExprRef::ConstantBool(b) if !b.value => Some("false"),
      _ => None,
    };
    if let Some(alt_val) = alt {
      emit_warning(
        self.context.get(),
        Code::MisleadingAndOr,
        node.base.base.location,
        format_args!(
          "The and-or expression always evaluates to the second alternative because the first alternative is {}; consider using if-then-else expression instead",
          alt_val
        ),
      );
    }
    true
  }
}
