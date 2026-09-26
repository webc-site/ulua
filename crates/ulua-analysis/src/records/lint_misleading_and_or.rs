use core::ptr::from_mut;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_attr::AstAttr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_constant_bool::AstExprConstantBool,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_try_as_ptr,
  visit::ast_stat_visit,
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
    self.visit_ast_expr_binary(from_mut(node))
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
  /// # Safety
  /// - `node` 须由 `AstVisitor::visit_expr_binary` 这类持有者以 `from_mut(&mut
  ///   AstExprBinary)` 传入：非空、对齐、指向 AST arena 中在整次 lint 遍历期间
  ///   存活的二元表达式节点，且其 `left/right` 子指针同样为 arena 写入的存活
  ///   节点。
  pub(crate) fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
    let node_ref = unsafe { &*node };
    if node_ref.op != AstExprBinaryOp::Or {
      return true;
    }
    let Some(and_) = (unsafe { ast_node_try_as_ptr::<AstExprBinary>(node_ref.left) }) else {
      return true;
    };
    if and_.op != AstExprBinaryOp::And {
      return true;
    }
    let right = and_.right;
    let alt = if matches!(right.as_expr_ref(), AstExprRef::ConstantNil(_)) {
      Some("nil")
    } else if let Some(bool_node) = unsafe { ast_node_try_as_ptr::<AstExprConstantBool>(right) } {
      if !bool_node.value {
        Some("false")
      } else {
        None
      }
    } else {
      None
    };
    if let Some(alt_val) = alt {
      emit_warning(
        self.context.get(),
        Code::MisleadingAndOr,
        node_ref.base.base.location,
        format_args!(
          "The and-or expression always evaluates to the second alternative because the first alternative is {}; consider using if-then-else expression instead",
          alt_val
        ),
      );
    }
    true
  }
}
