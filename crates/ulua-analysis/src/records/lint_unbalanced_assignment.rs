use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_varargs::AstExprVarargs,
    ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
    location::Location,
  },
  rtti::ast_node_is_ptr,
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintUnbalancedAssignment<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintUnbalancedAssignment<'ctx> {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    // methods/ 的 inherent 桥仍以 *mut () 收口，此处仅做指针形态转换。
    self.visit_ast_stat_local(from_mut(node).cast::<()>())
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    // methods/ 的 inherent 桥仍以 *mut () 收口，此处仅做指针形态转换。
    self.visit_ast_stat_assign(from_mut(node).cast::<()>())
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
      } else if unsafe {
        // Safety: last 取自 vals 界内槽位，为 arena 存活 AstExpr 指针；边界门面
        // 只读 class_index，判型语义与 cpp 逐点一致。
        ast_node_is_ptr::<AstExprCall>(last)
          || ast_node_is_ptr::<AstExprVarargs>(last)
          || ast_node_is_ptr::<AstExprConstantNil>(last)
      } {
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

// —— 原 methods/lint_unbalanced_assignment_visit_linter.rs ——
impl<'ctx> LintUnbalancedAssignment<'ctx> {
  pub fn visit_ast_stat_local(&mut self, node: *mut ()) -> bool {
    let node = node.cast::<AstStatLocal>();
    // Safety: 本回调仅由 visit_ast_stat_local 分发（lint_unbalanced_assignment.rs）
    // 在节点 class index 判定为 LOCALSTAT 后以同一指针调用，*mut () 回转为
    // *mut AstStatLocal 类型正确；节点指向 parser arena（块地址不移动），AST 在
    // lint 访问期内存活；vars/values 为 parser 填充的数组句柄，此处仅只读取。
    unsafe {
      self.assign(
        (*node).vars.size,
        &(*node).values,
        (*node).base.base.location,
      );
    }
    true
  }
  pub fn visit_ast_stat_assign(&mut self, node: *mut ()) -> bool {
    let node = node.cast::<AstStatAssign>();
    // Safety: 同上——该回调仅在节点 class index 判定为 ASSIGNSTAT 的分发路径上被
    // 调用，回转到 *mut AstStatAssign 类型正确；parser arena 节点地址稳定且在
    // lint 访问期存活，仅重建只读访问取 vars.size/values/location。
    unsafe {
      self.assign(
        (*node).vars.size,
        &(*node).values,
        (*node).base.base.location,
      );
    }
    true
  }
}
