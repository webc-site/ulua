use alloc::vec::Vec;
use core::ptr::from_ref;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
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
  rtti::{ast_node_try_as, ast_node_try_as_mut},
  visit::{ast_expr_visit_ref, ast_stat_visit_ref},
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, similar::similar},
  macros::lint_stat_process,
  records::{
    arena_handle::alias_opt, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

#[derive(Debug, Clone)]
pub struct LintDuplicateCondition<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintDuplicateCondition<'ctx> {
  fn visit_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    self.visit_ast_stat_if(node)
  }

  fn visit_expr_if_else(&mut self, node: &mut AstExprIfElse) -> bool {
    self.visit_ast_expr_if_else(node)
  }

  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    self.visit_ast_expr_binary(node)
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
        let cur_ref = alias_opt(cur_cond);
        let prev_ref = alias_opt(prev_cond);
        if similar(prev_ref, cur_ref) {
          let Some(cur_ref) = cur_ref else { continue };
          let Some(prev_ref) = prev_ref else { continue };
          let current = cur_ref.base.location;
          let previous = prev_ref.base.location;
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
  /// cpp `extractOpChain(conditions, expr, op)` 的引用形态：链上全部下转走
  /// 生命周期正确的 `ast_node_try_as`/句柄 `Node::get`，`conditions` 只存身份
  /// 指针（后续判等/取 location 用），不再锻造 'static 借用。
  pub(crate) fn extract_op_chain(
    &mut self,
    conditions: &mut Vec<*mut AstExpr>,
    expr: &AstExpr,
    op: AstExprBinaryOp,
  ) {
    if let Some(bin) = ast_node_try_as::<AstExprBinary>(expr) {
      if bin.op == op {
        // left/right 已句柄化恒非空，get 直接借出存活视图。
        self.extract_op_chain(conditions, bin.left.get(), op);
        self.extract_op_chain(conditions, bin.right.get(), op);
        return;
      }
    } else if let Some(group) = ast_node_try_as::<AstExprGroup>(expr) {
      // expr 已句柄化恒非空。
      self.extract_op_chain(conditions, group.expr.get(), op);
      return;
    }
    // 身份回存：expr 引用的地址即原槽位指针（repr(C) 基址重合），无解引用。
    conditions.push(from_ref(expr).cast_mut());
  }
}

// —— 原 methods/lint_duplicate_condition_process.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  lint_stat_process!(LintDuplicateCondition);
}

// —— 原 methods/lint_duplicate_condition_visit_linter.rs ——
impl<'ctx> LintDuplicateCondition<'ctx> {
  pub(crate) fn visit_ast_stat_if(&mut self, stat: &mut AstStatIf) -> bool {
    // `&mut` 形参即非空/独占证明（hook 直接出借 `&mut AstStatIf`），原
    // `alias_opt` 判空折叠随指针形参退役。
    let Some(elsebody) = stat.elsebody.get_mut() else {
      return true;
    };
    if ast_node_try_as_mut::<AstStatIf>(elsebody).is_none() {
      return true;
    }
    let mut conditions = Vec::with_capacity(2);
    let mut curr = Some(stat);
    while let Some(head) = curr {
      // 子槽已句柄化，独占借用链由 `get_mut()` 逐级供给，`_ref` 门面全链路 safe。
      ast_expr_visit_ref(head.condition.get_mut(), self);
      ast_stat_visit_ref(head.thenbody.cast::<AstStat>().get_mut(), self);
      conditions.push(head.condition.as_ptr());
      // 共享读先探类位（等价原共享下转判别），再物化独占借用：命中推进链，
      // 未命中遍历子树（对应旧 `elsebody.as_ptr()` 门面调用）。
      let is_chain_if = head.elsebody.is::<AstStatIf>();
      if let Some(else_stat) = head.elsebody.get_mut() {
        if is_chain_if {
          let next =
            ast_node_try_as_mut::<AstStatIf>(else_stat).expect("类位已探明，下转命中恒成立");
          curr = Some(next);
          continue;
        }
        ast_stat_visit_ref(else_stat, self);
      }
      break;
    }
    self.detect_duplicates(&conditions);
    false
  }
  pub(crate) fn visit_ast_expr_if_else(&mut self, expr: &mut AstExprIfElse) -> bool {
    // `&mut` 形参退役判空折叠；前置形态判别为只读判型，共享借用止于语句内。
    if !matches!(expr.false_expr.get().as_expr_ref(), AstExprRef::IfElse(_)) {
      return true;
    }
    let mut conditions = Vec::with_capacity(2);
    let mut curr = Some(expr);
    while let Some(head) = curr {
      // 子节点已句柄化恒非空；`get_mut()` 逐级供给独占借用，`_ref` 门面全链路 safe。
      ast_expr_visit_ref(head.condition.get_mut(), self);
      ast_expr_visit_ref(head.true_expr.get_mut(), self);
      conditions.push(head.condition.as_ptr());
      // false_expr 句柄化后判空折叠消失；先共享读探类位（与 cpp is<AstExprIfElse>
      // 等价），命中则物化独占借地下转推进链，否则独占遍历子树。
      if head.false_expr.is::<AstExprIfElse>() {
        let next = ast_node_try_as_mut::<AstExprIfElse>(head.false_expr.get_mut())
          .expect("类位已探明，下转命中恒成立");
        curr = Some(next);
        continue;
      }
      ast_expr_visit_ref(head.false_expr.get_mut(), self);
      break;
    }
    self.detect_duplicates(&conditions);
    false
  }
  pub(crate) fn visit_ast_expr_binary(&mut self, expr: &mut AstExprBinary) -> bool {
    // `&mut` 形参退役判空折叠；`op` 为 Copy 字段读取，共享只读借用止于各判别式。
    if expr.op != AstExprBinaryOp::And && expr.op != AstExprBinaryOp::Or {
      return true;
    }
    if expr.op == AstExprBinaryOp::Or
      && let Some(la) = expr.left.try_as_mut::<AstExprBinary>()
      && la.op == AstExprBinaryOp::And
    {
      // left/right 句柄的只读 try_as：借用止于布尔读出；后续遍历再逐级出借独占。
      let lb = la.left.try_as::<AstExprBinary>();
      let rb = la.right.try_as::<AstExprBinary>();
      let lb_is_and = lb.is_some_and(|b| b.op == AstExprBinaryOp::And);
      let rb_is_and = rb.is_some_and(|b| b.op == AstExprBinaryOp::And);
      if !lb_is_and && !rb_is_and {
        // left/right/expr.right 已句柄化恒非空；`get_mut()` 供给独占借用喂
        // `_ref` 门面（la 借 `expr.left`、与 `expr.right` 字段互斥，借用不相交）。
        ast_expr_visit_ref(la.left.get_mut(), self);
        ast_expr_visit_ref(la.right.get_mut(), self);
        ast_expr_visit_ref(expr.right.get_mut(), self);
        return false;
      }
      // and-chain longer than two; continue with duplicate detection.
    }
    let mut conditions = Vec::with_capacity(2);
    self.extract_op_chain(&mut conditions, &expr.base, expr.op);
    self.detect_duplicates(&conditions);
    false
  }
}
