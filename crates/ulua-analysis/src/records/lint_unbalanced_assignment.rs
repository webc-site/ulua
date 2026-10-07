use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_stat_assign::AstStatAssign,
    ast_stat_local::AstStatLocal, ast_visitor::AstVisitor, location::Location,
  },
  visit::ast_stat_visit_ref,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{
    arena_handle::alias_ref, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

#[derive(Debug, Clone)]
pub struct LintUnbalancedAssignment<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintUnbalancedAssignment<'ctx> {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    // 原 visit_ast_stat_local 的 `*mut ()` inherent 桥并入本覆写：分发链直接
    // 交付 &mut 借用（节点出自 parser arena、lint 访问期只读存活），
    // 指针回转与 unsafe 解引用一并作废；取 vars.size/values/location 交
    // assign 的语义不变。
    self.assign(node.vars.size, &node.values, node.base.base.location);
    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    // 同上：原 visit_ast_stat_assign 的 `*mut ()` 桥并入本覆写。
    self.assign(node.vars.size, &node.values, node.base.base.location);
    true
  }
}

// —— 原 methods/lint_unbalanced_assignment_assign.rs ——
impl<'ctx> LintUnbalancedAssignment<'ctx> {
  pub(crate) fn assign(
    &mut self,
    vars: usize,
    values: &AstArray<*mut AstExpr>,
    location: Location,
  ) {
    let vals = values.as_slice();
    if vars != vals.len() && !vals.is_empty() {
      let last = vals[vals.len() - 1];
      if vars < vals.len() {
        let msg = format!(
          "Assigning {} values to {} variables leaves some values unused",
          vals.len(),
          vars
        );
        emit_warning(
          self.context.get(),
          Code::UnbalancedAssignment,
          location,
          format_args!("{}", msg),
        );
      } else if matches!(
        alias_ref(last).as_expr_ref(),
        AstExprRef::Call(_) | AstExprRef::Varargs(_) | AstExprRef::ConstantNil(_)
      ) {
        // we don't know how many values the last expression returns
        // or last expression is nil which explicitly silences the nil-init warning
      } else {
        let msg = format!(
          "Assigning {} values to {} variables initializes extra variables with nil; add 'nil' to value list to silence",
          vals.len(),
          vars
        );
        emit_warning(
          self.context.get(),
          Code::UnbalancedAssignment,
          location,
          format_args!("{}", msg),
        );
      }
    }
  }
}

// —— 原 methods/lint_unbalanced_assignment_process.rs ——
impl<'ctx> LintUnbalancedAssignment<'ctx> {
  lint_stat_process!(
    #[inline(never)]
    LintUnbalancedAssignment
  );
}
