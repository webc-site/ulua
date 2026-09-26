use core::ptr::{from_mut, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_local::AstExprLocal,
    ast_expr_varargs::AstExprVarargs, ast_local::AstLocal, ast_stat_assign::AstStatAssign,
    ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
  },
  rtti::{AstNodePtr, ast_node_is_ptr, ast_node_try_as},
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
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
  pub fn lint_uninitialized_local(&mut self) {
    self.locals = DenseHashMap::default();
  }

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

  pub fn visit_assign(&mut self, _var: *mut AstExpr) {
    // implemented in separate method files
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
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe {
      ast_stat_visit(root, &mut pass);
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
      unsafe {
        emit_warning(
          context.get(),
          Code::UninitializedLocal,
          (*l.first_use).base.base.location,
          format_args!(
            "Variable '{}' defined at line {} is never initialized or assigned; initialize with 'nil' to silence",
            (*local).name,
            (*local).location.begin.line + 1
          ),
        );
      }
    }
  }
}

// —— 原 methods/lint_uninitialized_local_visit_assign.rs：C++ `LintUninitializedLocal::visitAssign` (`Analysis/src/Linter.cpp:2184`). ——
pub fn lint_uninitialized_local_visit_assign(pass: &mut LintUninitializedLocal, node: &AstExpr) {
  if let Some(lv) = ast_node_try_as::<AstExprLocal>(&node.base) {
    // local 槽已句柄化恒非空；locals 键值为既有裸指针形态，经 as_ptr 桥接。
    let l = pass.locals.get_or_insert(lv.local.as_ptr());
    l.assigned = true;
  } else {
    // Safety: node 由调用方自 C++ 遍历传入的 `*mut AstExpr` 借出，指回 arena
    // 中存活的表达式；此处仅把同一地址转回裸指针交给表达式分发器递归只读遍历，
    // 传入的 `pass` 是 linter 状态、与 AST arena 不相交，无别名冲突。
    unsafe { ast_expr_visit((node as *const AstExpr).cast_mut(), pass) };
  }
}

// —— 原 methods/lint_uninitialized_local_visit_linter.rs ——
impl<'ctx> LintUninitializedLocal<'ctx> {
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    // Safety: 本 crate 内唯一入口是 `AstVisitor::visit_stat_local`，实参由
    // `from_mut(&mut AstStatLocal)` 取得——非空且指向 parser arena 中存活的
    // 正确类型节点；只读借用随本语句块结束，lint 状态 `self.locals` 与
    // AST arena 不相交。
    let node_ref = unsafe { &*node };
    let values = node_ref.values.as_slice();
    let last = values.last().copied().unwrap_or(null_mut());
    // 判空守卫保留 C++ `last &&` 结构；ast_node_is 改走指针门面（同为偏移 0
    // 读 class_index），不再手写 `&*` 重建引用。
    let vararg = !last.is_null()
      && (unsafe { ast_node_is_ptr::<AstExprVarargs>(last.as_ast_node()) }
        || unsafe { ast_node_is_ptr::<AstExprCall>(last.as_ast_node()) });
    for (i, &var) in node_ref.vars.as_slice().iter().enumerate() {
      let l = self.locals.get_or_insert(var);
      l.defined = true;
      l.initialized = vararg || i < values.len();
    }
    true
  }
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    // Safety: node 由 `AstVisitor::visit_stat_assign` 的 `from_mut(&mut ..)`
    // 传入——非空、对齐且指向 parser arena 存活的本类型节点；块内仅只读
    // vars/values 的 `*mut AstExpr` 元素指针并交给 visit/分发器，它们写入的
    // `self` 是 linter 状态、与 AST arena 不相交，无别名冲突。
    unsafe {
      let node_ref = &*node;
      for &var in node_ref.vars.as_slice() {
        lint_uninitialized_local_visit_assign(self, &*var);
      }
      for &value in node_ref.values.as_slice() {
        ast_expr_visit(value, self);
      }
    }
    false
  }
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    // Safety: 入口同 stat_assign——`from_mut(&mut AstStatFunction)` 保证 node
    // 非空且指向 arena 存活节点；`name`/`func` 已句柄化为 Node（非空由类型层
    // 承载），`.get()`/`as_ptr` 桥交只读遍历门面，不与 `&mut self`（linter
    // 状态）产生别名交集。
    unsafe {
      let node_ref = &*node;
      lint_uninitialized_local_visit_assign(self, node_ref.name.get());
      ast_expr_visit(node_ref.func.cast::<AstExpr>().as_ptr(), self);
    }
    false
  }
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    // Safety: node 由 `AstVisitor::visit_expr_local` 的 `from_mut(&mut ..)`
    // 得到——非空且指向 arena 存活的 AstExprLocal；其 `local` 子指针由 parser
    // 保证非空，且仅作为 `*mut AstLocal` 键值存入 self.locals（不重建引用）。
    // 下方回写的 `first_use = node` 记录的正是本次分发帧内保证存活的节点地址，
    // 后续仅做 null 比较与报告期只读使用。
    let node_ref = unsafe { &*node };
    let local = node_ref.local;
    let local_ref = self.locals.get_or_insert(local.as_ptr());
    if local_ref.first_use.is_null() {
      local_ref.first_use = node;
    }
    false
  }
}
