use core::ptr::from_mut;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_local::AstLocal,
    ast_stat_assign::AstStatAssign, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{ast_expr_visit_ref, ast_stat_visit_ref},
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{
    arena_handle::{alias, alias_opt, alias_opt_mut, alias_ref},
    lint_context::LintContext,
    lint_context_handle::LintContextHandle,
  },
};

#[derive(Debug, Clone, Default)]
pub struct Local {
  pub(crate) defined: bool,
  pub(crate) initialized: bool,
  pub(crate) assigned: bool,
  pub(crate) first_use: *mut AstExprLocal,
}

impl DenseDefault for Local {
  fn dense_default() -> Self {
    Self::default()
  }
}

#[derive(Debug, Clone)]
pub struct LintUninitializedLocal<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) locals: DenseHashMap<*mut AstLocal, Local>,
}

impl<'ctx> LintUninitializedLocal<'ctx> {
  pub fn report(&mut self) {
    // NOTE: exact warning emission/reporting is implemented in a separate translated method file.
    // This record item only models state needed by that method.
  }

  pub fn visit_stat_local(&mut self, _node: &mut AstStatLocal) -> bool {
    true
  }

  pub fn visit_stat_assign(&mut self, _node: &mut AstStatAssign) -> bool {
    true
  }

  pub fn visit_stat_function(&mut self, _node: &mut AstStatFunction) -> bool {
    true
  }

  pub fn visit_expr_local(&mut self, _node: &mut AstExprLocal) -> bool {
    true
  }
}

impl<'ctx> AstVisitor for LintUninitializedLocal<'ctx> {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    self.visit_ast_stat_local(from_mut(node))
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(from_mut(node))
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(from_mut(node))
  }

  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(from_mut(node))
  }
}

// —— 原 methods/lint_uninitialized_local_process.rs ——
impl<'ctx> LintUninitializedLocal<'ctx> {
  pub fn process(context: &mut LintContext) {
    let root = context.root;
    let mut pass = LintUninitializedLocal {
      context: LintContextHandle::from_ref(context),
      locals: DenseHashMap::default(),
    };
    // root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；`alias_opt_mut`
    // 句柄边界折叠 null（与旧指针门面同语义）后交引用门面递归，遍历为单线程
    // 串行，宿主 LintContext 的写句柄由本 pass 独占。
    if let Some(root) = alias_opt_mut(root) {
      ast_stat_visit_ref(root, &mut pass);
    }
    lint_uninitialized_local_report(&mut pass);
  }
}

// —— 原 methods/lint_uninitialized_local_report.rs：C++ `LintUninitializedLocal::report` (`Analysis/src/Linter.cpp:2118`). ——
pub fn lint_uninitialized_local_report(pass: &mut LintUninitializedLocal<'_>) {
  let mut context = pass.context;
  for (local, l) in pass.locals.iter() {
    let local = *local;
    if l.defined && !l.initialized && !l.assigned && !l.first_use.is_null() {
      emit_warning(
        context.get(),
        Code::UninitializedLocal,
        alias_ref(l.first_use).base.base.location,
        format_args!(
          "Variable '{}' defined at line {} is never initialized or assigned; initialize with 'nil' to silence",
          alias_ref(local).name,
          alias_ref(local).location.begin.line + 1
        ),
      );
    }
  }
}

// —— 原 methods/lint_uninitialized_local_visit_assign.rs：C++ `LintUninitializedLocal::visitAssign` (`Analysis/src/Linter.cpp:2184`). ——
/// `node` 的 `&mut` 借用即「节点非空、存活且本帧可独占」的类型系统证明（原
/// `&AstExpr`+const→mut 回灌指针门面的形态退役）：非 Local 目标经引用门面
/// 递归遍历子树，写穿语义与 cpp `expr->visit(this)` 一致。
pub fn lint_uninitialized_local_visit_assign(
  pass: &mut LintUninitializedLocal,
  node: &mut AstExpr,
) {
  if let AstExprRef::Local(lv) = node.as_expr_ref() {
    // local 槽已句柄化恒非空；locals 键值为既有裸指针形态，经 as_ptr 桥接。
    let l = pass.locals.get_or_insert(lv.local.as_ptr());
    l.assigned = true;
  } else {
    ast_expr_visit_ref(node, pass);
  }
}

// —— 原 methods/lint_uninitialized_local_visit_linter.rs ——
impl<'ctx> LintUninitializedLocal<'ctx> {
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    let node_ref = alias_ref(node);
    let values = node_ref.values.as_slice();
    let vararg = values
      .last()
      .copied()
      .and_then(|last| alias_opt(last))
      .is_some_and(|l| {
        matches!(
          l.as_expr_ref(),
          AstExprRef::Varargs(_) | AstExprRef::Call(_)
        )
      });
    for (i, &var) in node_ref.vars.as_slice().iter().enumerate() {
      let l = self.locals.get_or_insert(var);
      l.defined = true;
      l.initialized = vararg || i < values.len();
    }
    true
  }
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    // node 由 `AstVisitor::visit_stat_assign` 的 `from_mut(&mut ..)` 传入——非空
    // 且指向 parser arena 存活节点；`alias` 门面物化独占借用，vars/values 裸槽
    // 经 `OptNode` 句柄边界出借 `&mut AstExpr` 喂引用门面，整链无 unsafe。
    let node_ref = alias(node);
    for &var in node_ref.vars.as_slice() {
      if let Some(var) = OptNode::from_ptr(var).get_mut() {
        lint_uninitialized_local_visit_assign(self, var);
      }
    }
    for &value in node_ref.values.as_slice() {
      if let Some(value) = OptNode::from_ptr(value).get_mut() {
        ast_expr_visit_ref(value, self);
      }
    }
    false
  }
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    // 入口同上：`alias` 物化本帧独占借用，`name`/`func` 句柄经 `get_mut()`
    // 逐级出借子节点独占借用喂引用门面，全链路 safe。
    let node_ref = alias(node);
    lint_uninitialized_local_visit_assign(self, node_ref.name.get_mut());
    ast_expr_visit_ref(node_ref.func.cast::<AstExpr>().get_mut(), self);
    false
  }
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    // node 的 `local` 子指针仅作为 `*mut AstLocal` 键值存入 self.locals（不重建
    // 引用）；下方回写的 `first_use = node` 记录的正是本次分发帧内保证存活的
    // 节点地址，后续仅做 null 比较与报告期只读使用。
    let node_ref = alias_ref(node);
    let local = node_ref.local;
    let local_ref = self.locals.get_or_insert(local.as_ptr());
    if local_ref.first_use.is_null() {
      local_ref.first_use = node;
    }
    false
  }
}
