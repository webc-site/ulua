use alloc::vec::Vec;
use core::ptr::from_mut;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_name::AstName,
    ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_type::AstType, ast_type_pack::AstTypePack,
    ast_type_reference::AstTypeReference, ast_visitor::AstVisitor,
  },
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{
    lint_context::LintContext, lint_context_handle::LintContextHandle, local_linter::Local,
  },
};

/// C++ `LintLocalHygiene::Global` (`Analysis/src/Linter.cpp:725`).
#[derive(Debug, Clone, Copy, Default)]
pub struct Global<'ctx> {
  pub(crate) used: bool,
  pub(crate) builtin: bool,
  pub(crate) first_ref: Option<&'ctx AstExprGlobal>,
}

#[derive(Debug, Clone)]
pub struct LintLocalHygiene<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) locals: DenseHashMap<*mut AstLocal, Local>,
  pub(crate) imports: DenseHashMap<AstName, *mut AstLocal>,
  pub(crate) globals: DenseHashMap<AstName, Global<'ctx>>,
}

// —— 原 methods/lint_local_hygiene_is_require_call.rs ——
impl<'ctx> LintLocalHygiene<'ctx> {
  pub(crate) fn is_require_call(&mut self, expr: *mut AstExpr) -> bool {
    let Some(expr_ref) = (unsafe { expr.as_ref() }) else {
      return false;
    };
    if let AstExprRef::Call(call) = expr_ref.as_expr_ref()
      && let Some(func_ref) = unsafe { call.func.as_ref() }
      && let AstExprRef::Global(global) = func_ref.as_expr_ref()
    {
      global.name.as_bytes() == b"require"
    } else {
      false
    }
  }
}

// —— 原 methods/lint_local_hygiene_lint_local_hygiene.rs ——
impl<'ctx> DenseDefault for Global<'ctx> {
  fn dense_default() -> Self {
    Self::default()
  }
}
impl DenseDefault for Local {
  fn dense_default() -> Self {
    Self::default()
  }
}

// —— 原 methods/lint_local_hygiene_process.rs ——
impl<'ctx> AstVisitor for LintLocalHygiene<'ctx> {
  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(from_mut(node))
  }
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    self.visit_ast_stat_local(from_mut(node))
  }
  fn visit_stat_local_function(&mut self, node: &mut AstStatLocalFunction) -> bool {
    self.visit_ast_stat_local_function(from_mut(node))
  }
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(from_mut(node))
  }
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(from_mut(node))
  }
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit_ast_expr_function(from_mut(node))
  }
  fn visit_type(&mut self, node: &mut AstType) -> bool {
    self.visit_ast_type(from_mut(node))
  }
  fn visit_type_reference(&mut self, node: &mut AstTypeReference) -> bool {
    self.visit_ast_type_reference(from_mut(node))
  }
  fn visit_type_pack(&mut self, node: &mut AstTypePack) -> bool {
    self.visit_ast_type_pack(from_mut(node))
  }
}
impl<'ctx> LintLocalHygiene<'ctx> {
  pub fn process(context: &mut LintContext) {
    let root = context.root;
    let mut pass = LintLocalHygiene {
      context: LintContextHandle::from_ref(context),
      locals: DenseHashMap::default(),
      imports: DenseHashMap::default(),
      globals: DenseHashMap::default(),
    };
    // 内置全局预载：与 LintGlobalLocal 同款，经宿主句柄读取后写入 pass 状态。
    let mut handle = pass.context;
    for (global_name, _global) in handle.get().builtin_globals.iter() {
      let g = Global {
        builtin: true,
        ..Default::default()
      };
      pass.globals.try_insert(*global_name, g);
    }
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe { ast_stat_visit(root, &mut pass) };
    pass.report();
  }
}

// —— 原 methods/lint_local_hygiene_report.rs ——
impl<'ctx> LintLocalHygiene<'ctx> {
  pub fn report(&mut self) {
    let locals = self
      .locals
      .iter()
      .map(|(local, info)| (*local, info.clone()))
      .collect::<Vec<_>>();
    for (local, info) in locals {
      // Safety: `local` 是 self.locals 表的键——visitor 遍历期从 parser arena 写入的
      // 非空 `*mut AstLocal`（arena 比整趟 lint pass 长寿，parse 后节点字段不再改写），
      // 重建只读共享借用供两个 report 分支使用；`info` 是同一表项的克隆，`locals` 已
      // collect 成交付值，`&mut self` 无别名借用。
      let local = unsafe { &*local };
      if info.used {
        self.report_used_local(local, &info);
      } else if !info.defined.is_null() {
        self.report_unused_local(local, &info);
      }
    }
  }
}

// —— 原 methods/lint_local_hygiene_report_unused_local.rs ——
impl<'ctx> LintLocalHygiene<'ctx> {
  /// cpp `reportUnusedLocal(AstLocal*)`：`local` 为分析期存活、由 arena 持有的局部
  /// 节点共享借用（cpp 裸指针形参的 Rust 对应），本方法只读其 `name`/`location`；
  /// 告警经 `self.context` 写句柄发出。
  pub fn report_unused_local(&mut self, local: &AstLocal, info: &Local) {
    let name = local.name;
    let bytes = name.as_bytes();
    if bytes.is_empty() || bytes[0] == b'_' {
      return;
    }
    let (code, prefix) = if info.function {
      (Code::FunctionUnused, "Function")
    } else if info.import {
      (Code::ImportUnused, "Import")
    } else {
      (Code::LocalUnused, "Variable")
    };
    emit_warning(
      self.context.get(),
      code,
      local.location,
      format_args!(
        "{} '{}' is never used; prefix with '_' to silence",
        prefix, name
      ),
    );
  }
}

// —— 原 methods/lint_local_hygiene_report_used_local.rs ——
impl<'ctx> LintLocalHygiene<'ctx> {
  /// cpp `reportUsedLocal(AstLocal*, const Local&)`：`local` 为分析期存活、由 arena
  /// 持有的局部节点共享借用（cpp 裸指针形参的 Rust 对应），本方法只读其 `shadow`/
  /// `name`/`location`/`function_depth` 字段；`info` 为同一条目的值拷贝，仅参与
  /// `defined`/`function` 比较。告警经 `self.context` 写句柄发出。
  pub fn report_used_local(&mut self, local: &AstLocal, info: &Local) {
    let mut handle = self.context;
    let context = handle.get();
    let shadow = local.shadow;
    if !shadow.is_null() {
      let shadow_local = self.locals.find(&shadow);
      let duplicate_function_enabled = context.options.is_enabled(Code::DuplicateFunction);
      let duplicate_local_enabled = context.options.is_enabled(Code::DuplicateLocal);
      if duplicate_function_enabled
        && info.function
        && shadow_local.is_some_and(|shadow_info| shadow_info.function)
      {
        return;
      }
      if duplicate_local_enabled
        && shadow_local.is_some_and(|shadow_info| shadow_info.defined == info.defined)
      {
        return;
      }
      // Safety: parser `push_local` 把 `shadow` 写为被遮蔽的前一同名 `AstLocal`
      // （同一 arena 的存活槽位）或 null（cpp `if (AstLocal* shadow =
      // local->shadow)` 的隐式前提）；上方判空已排除 null，此处共享借用只读
      // `function_depth`/`location`，遍历期间 AST 无写入。
      let shadow = unsafe { &*shadow };
      if shadow.function_depth == local.function_depth {
        emit_warning(
          context,
          Code::LocalShadow,
          local.location,
          format_args!(
            "Variable '{}' shadows previous declaration at line {}",
            local.name,
            shadow.location.begin.line + 1
          ),
        );
      }
      return;
    }
    if let Some(global) = self.globals.find(&local.name) {
      if global.builtin {
        return;
      }
      if let Some(first_ref) = global.first_ref {
        emit_warning(
          context,
          Code::LocalShadow,
          local.location,
          format_args!(
            "Variable '{}' shadows a global variable used at line {}",
            local.name,
            first_ref.base.base.location.begin.line + 1
          ),
        );
      } else {
        emit_warning(
          context,
          Code::LocalShadow,
          local.location,
          format_args!("Variable '{}' shadows a global variable", local.name),
        );
      }
    }
  }
}

// —— 原 methods/lint_local_hygiene_visit_linter.rs ——
impl<'ctx> LintLocalHygiene<'ctx> {
  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstStatAssign`；本 crate 内
  /// 唯一入口是 `AstVisitor::visit_stat_assign`（实参经 `from_mut` 取自分发给
  /// 出的 `&mut` 借用）。`vars`/`values` 数组元素由 parser 构造，均为指向 arena
  /// 存活节点的非常空指针；`self` 只持有 linter 记账状态，与 AST arena 不相交。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    let node_ref = unsafe { &*node };
    for &var in node_ref.vars.as_slice() {
      let Some(var_ref) = (unsafe { var.as_ref() }) else {
        continue;
      };
      if !matches!(var_ref.as_expr_ref(), AstExprRef::Local(_)) {
        unsafe {
          ast_expr_visit(var, self);
        }
      }
    }
    for &val in node_ref.values.as_slice() {
      unsafe {
        ast_expr_visit(val, self);
      }
    }
    false
  }

  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstStatLocal`（唯一入口
  /// `AstVisitor::visit_stat_local` 的 `from_mut` 转换）；`vars`/`values` 元素
  /// 均为 arena 内存活的非常空 `AstLocal`/`AstExpr` 指针，且 `&mut self` 借用
  /// 期内无人以其他路径改写这些节点。
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    let node_ref = unsafe { &*node };
    let vars = node_ref.vars.as_slice();
    let values = node_ref.values.as_slice();
    if vars.len() == 1 && values.len() == 1 {
      let local = vars[0];
      let value = values[0];
      let is_import = self.is_require_call(value);
      {
        let info = self.locals.get_or_insert(local);
        info.defined = node.cast();
        info.import = is_import;
      }
      if is_import
        && let Some(local_ref) = unsafe { local.as_ref() }
      {
        *self.imports.get_or_insert(local_ref.name) = local;
      }
    } else {
      for &local in vars {
        let info = self.locals.get_or_insert(local);
        info.defined = node.cast();
      }
    }
    true
  }

  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstStatLocalFunction`（唯一
  /// 入口 `AstVisitor::visit_stat_local_function`）；其 `name` 字段按 parser
  /// 不变量恒为非空 `*mut AstLocal`（local function 必有绑定名）。
  pub(crate) fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
    let node_ref = unsafe { &*node };
    let info = self.locals.get_or_insert(node_ref.name.as_ptr());
    info.defined = node.cast();
    info.function = true;
    true
  }

  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstExprLocal`（唯一入口
  /// `AstVisitor::visit_expr_local`）；其 `local` 字段由 parser 构造，恒为非空
  /// 且指向同一 arena 中存活的 `AstLocal`。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    let node_ref = unsafe { &*node };
    self.locals.get_or_insert(node_ref.local.as_ptr()).used = true;
    true
  }

  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstExprGlobal`（唯一入口
  /// `AstVisitor::visit_expr_global`）；其 `name` 指向 `AstNameTable` 持有的
  /// 常量字符串，生命周期覆盖整个 lint 遍历。
  pub(crate) fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
    let Some(node_ref) = (unsafe { node.as_ref() }) else {
      return true;
    };
    let global = self.globals.get_or_insert(node_ref.name);
    global.used = true;
    if global.first_ref.is_none() {
      global.first_ref = Some(node_ref);
    }
    true
  }
  pub fn visit_ast_type(&mut self, node: *mut AstType) -> bool {
    let _ = node;
    true
  }
  pub fn visit_ast_type_pack(&mut self, node: *mut AstTypePack) -> bool {
    let _ = node;
    true
  }
  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstTypeReference`（唯一入口
  /// `AstVisitor::visit_type_reference`）；遍历帧内调用方持有该节点的 `&mut`
  /// 借用，本函数立即降级为共享再借用读取 `prefix`，期间无其他写路径。
  pub(crate) fn visit_ast_type_reference(&mut self, node: *mut AstTypeReference) -> bool {
    // Safety: node 由 visitor 的 `&mut AstTypeReference` 转成（from_mut），此处
    // `&*node` 是同一位置的共享再借用，存活、对齐且非空；imports/locals 更新
    // 不触碰该节点。
    let node_ref = unsafe { &*node };
    let Some(prefix) = node_ref.prefix else {
      return true;
    };
    if let Some(ast_local) = self.imports.find(&prefix) {
      let local = self.locals.get_or_insert(*ast_local);
      debug_assert!(local.import);
      local.used = true;
    }
    true
  }
  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstExprFunction`（唯一入口
  /// `AstVisitor::visit_expr_function`）；`self_`/`args` 中的 `*mut AstLocal`
  /// 均为 parser 构造的非常空指针且指向存活 `AstLocal`。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    // Safety: node 由 visitor 分发的 `&mut` 经 from_mut 而来，`&*node` 为共享
    // 再借用，存活且对齐；本函数只读 self_/args 的指针值作 map 键，不写节点，
    // 与 `self` 状态无别名冲突。
    let node_ref = unsafe { &*node };
    if !node_ref.self_.is_null() {
      self.locals.get_or_insert(node_ref.self_.as_ptr()).arg = true;
    }
    for arg in node_ref.args.iter_nodes() {
      self.locals.get_or_insert(arg.as_ptr()).arg = true;
    }
    true
  }
}
