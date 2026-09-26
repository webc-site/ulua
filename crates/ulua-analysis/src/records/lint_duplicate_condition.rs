use alloc::vec::Vec;
use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_attr::AstAttr,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_stat::AstStat,
    ast_stat_if::AstStatIf,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_is, ast_node_try_as, ast_node_try_as_ptr},
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, similar::similar},
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintDuplicateCondition<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintDuplicateCondition<'ctx> {
  fn visit_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    self.visit_ast_stat_if(from_mut(node))
  }

  fn visit_expr_if_else(&mut self, node: &mut AstExprIfElse) -> bool {
    self.visit_ast_expr_if_else(from_mut(node))
  }

  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    self.visit_ast_expr_binary(from_mut(node))
  }

  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_duplicate_condition_detect_duplicates.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  pub fn detect_duplicates(&mut self, conditions: &[*mut AstExpr]) {
    const K_MAX_DISTANCE: usize = 5;
    // K_MAX_DISTANCE 窗口内比较是否重复
    for (i, &cur_cond) in conditions.iter().enumerate() {
      let start = i.saturating_sub(K_MAX_DISTANCE);
      for &prev_cond in &conditions[start..i] {
        // Safety: 两指针由 visit 侧从存活 AstStatIf.condition 字段压入，parser
        // 保证条件表达式非空（语法必选）；AST 活过整轮 lint，similar 仅按
        // class index 与结构比对只读遍历二者。
        if unsafe { similar(prev_cond, cur_cond) } {
          // Safety: similar 命中已实际解引用二者无 panic，且两元素同源
          // （if/elseif 条件链，parser 非空不变量），base.location 按值读出。
          let current = unsafe { (*cur_cond).base.location };
          // Safety: 同 current——prev_cond 是条件链上的存活非空节点。
          let previous = unsafe { (*prev_cond).base.location };
          if current.begin.line == previous.begin.line {
            emit_warning(
              self.context.get(),
              Code::DuplicateCondition,
              current,
              format_args!(
                "Condition has already been checked on column {}",
                previous.begin.column + 1
              ),
            );
          } else {
            emit_warning(
              self.context.get(),
              Code::DuplicateCondition,
              current,
              format_args!(
                "Condition has already been checked on line {}",
                previous.begin.line + 1
              ),
            );
          }
          break;
        }
      }
    }
  }
}

// —— 原 methods/lint_duplicate_condition_extract_op_chain.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  pub(crate) fn extract_op_chain(
    &mut self,
    conditions: &mut Vec<*mut AstExpr>,
    expr: *mut AstExpr,
    op: AstExprBinaryOp,
  ) {
    if let Some(bin) = unsafe { ast_node_try_as_ptr::<AstExprBinary>(expr) } {
      if bin.op == op {
        // left/right 已句柄化恒非空；conditions 行走链为既有裸指针 API，经 as_ptr 桥接。
        self.extract_op_chain(conditions, bin.left.as_ptr(), op);
        self.extract_op_chain(conditions, bin.right.as_ptr(), op);
        return;
      }
    } else if let Some(group) = unsafe { ast_node_try_as_ptr::<AstExprGroup>(expr) } {
      // expr 已句柄化恒非空；conditions 行走链为既有裸指针 API，经 as_ptr 桥接。
      self.extract_op_chain(conditions, group.expr.as_ptr(), op);
      return;
    }
    conditions.push(expr);
  }
}

// —— 原 methods/lint_duplicate_condition_lint_duplicate_condition.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  pub fn lint_duplicate_condition(&mut self) {
    // Placeholder for constructor logic
  }
}

// —— 原 methods/lint_duplicate_condition_process.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  lint_stat_process!(LintDuplicateCondition);
}

// —— 原 methods/lint_duplicate_condition_visit_linter.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  pub(crate) fn visit_ast_stat_if(&mut self, stat: *mut AstStatIf) -> bool {
    let Some(stat_ref) = (unsafe { stat.as_ref() }) else {
      return true;
    };
    let Some(elsebody) = stat_ref.elsebody.get() else {
      return true;
    };
    if ast_node_try_as::<AstStatIf>(elsebody).is_none() {
      return true;
    }
    let mut conditions = Vec::with_capacity(2);
    let mut curr = Some(stat_ref);
    while let Some(head) = curr {
      unsafe {
        ast_expr_visit(head.condition.as_ptr(), self);
        ast_stat_visit(head.thenbody.cast::<AstStat>().as_ptr(), self);
      }
      conditions.push(head.condition.as_ptr());
      if let Some(else_stat) = head.elsebody.get() {
        if let Some(next) = ast_node_try_as::<AstStatIf>(else_stat) {
          curr = Some(next);
          continue;
        }
        unsafe {
          ast_stat_visit(head.elsebody.as_ptr(), self);
        }
      }
      break;
    }
    self.detect_duplicates(&conditions);
    false
  }
  pub(crate) fn visit_ast_expr_if_else(&mut self, expr: *mut AstExprIfElse) -> bool {
    let Some(expr_ref) = (unsafe { expr.as_ref() }) else {
      return true;
    };
    if !ast_node_is::<AstExprIfElse>(&expr_ref.false_expr.get().base) {
      return true;
    }
    let mut conditions = Vec::with_capacity(2);
    let mut curr = Some(expr_ref);
    while let Some(head) = curr {
      // 子节点已句柄化恒非空；ast_expr_visit 为既有裸指针门面，经 as_ptr 桥接。
      unsafe {
        ast_expr_visit(head.condition.as_ptr(), self);
        ast_expr_visit(head.true_expr.as_ptr(), self);
      }
      conditions.push(head.condition.as_ptr());
      // false_expr 句柄化后判空折叠消失，RTTI 下转走共享引用门面（与 cpp is<AstExprIfElse> 等价）。
      if let Some(next) = ast_node_try_as::<AstExprIfElse>(&head.false_expr.get().base) {
        curr = Some(next);
        continue;
      }
      unsafe {
        ast_expr_visit(head.false_expr.as_ptr(), self);
      }
      break;
    }
    self.detect_duplicates(&conditions);
    false
  }
  pub(crate) fn visit_ast_expr_binary(&mut self, expr: *mut AstExprBinary) -> bool {
    let Some(expr_ref) = (unsafe { expr.as_ref() }) else {
      return true;
    };
    if expr_ref.op != AstExprBinaryOp::And && expr_ref.op != AstExprBinaryOp::Or {
      return true;
    }
    if expr_ref.op == AstExprBinaryOp::Or
      && let Some(la) = unsafe { ast_node_try_as_ptr::<AstExprBinary>(expr_ref.left) }
      && la.op == AstExprBinaryOp::And
    {
      let lb = unsafe { ast_node_try_as_ptr::<AstExprBinary>(la.left) };
      let rb = unsafe { ast_node_try_as_ptr::<AstExprBinary>(la.right) };
      let lb_is_and = lb.is_some_and(|b| b.op == AstExprBinaryOp::And);
      let rb_is_and = rb.is_some_and(|b| b.op == AstExprBinaryOp::And);
      if lb_is_and || rb_is_and {
        // This is an and-chain longer than two; continue with duplicate detection.
      } else {
        unsafe {
          // left/right 已句柄化恒非空；ast_expr_visit 为既有裸指针门面，经 as_ptr 桥接。
          ast_expr_visit(la.left.as_ptr(), self);
          ast_expr_visit(la.right.as_ptr(), self);
          ast_expr_visit(expr_ref.right.as_ptr(), self);
        }
        return false;
      }
    }
    let mut conditions = Vec::with_capacity(2);
    self.extract_op_chain(&mut conditions, expr.cast(), expr_ref.op);
    self.detect_duplicates(&conditions);
    false
  }
}
