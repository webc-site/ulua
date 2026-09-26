use alloc::vec::Vec;
use core::ptr::{from_mut, from_ref, null};

use ulua_ast::{
  records::{
    ast_attr::AstAttr, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_constant_number::AstExprConstantNumber, ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_name::AstName, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_visitor::AstVisitor,
    deprecated_info::DeprecatedInfo, location::Location,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as, ast_node_try_as_ptr},
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_config::enums::code::Code;

use crate::{
  functions::{
    emit_warning::emit_warning, follow_type, get_type, is_prim::is_number,
    lookup_extern_type_prop::lookup_extern_type_prop,
  },
  macros::lint_stat_process,
  records::{
    extern_type::ExternType, function_type::FunctionType, lint_context::LintContext,
    lint_context_handle::LintContextHandle, property_type::Property, table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct LintDeprecatedApi<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) function_type_scope_stack: Vec<*const FunctionType>,
}

impl<'ctx> AstVisitor for LintDeprecatedApi<'ctx> {
  fn visit_expr_index_name(&mut self, node: &mut AstExprIndexName) -> bool {
    self.visit_ast_expr_index_name(from_mut(node))
  }

  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(from_mut(node))
  }

  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(from_mut(node))
  }

  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.visit_ast_expr_call(from_mut(node))
  }

  fn visit_stat_local_function(&mut self, node: &mut AstStatLocalFunction) -> bool {
    self.visit_ast_stat_local_function(from_mut(node))
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(from_mut(node))
  }

  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_deprecated_api_check_linter.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub fn check_ast_expr_index_name_type_id(&mut self, node: &AstExprIndexName, ty: TypeId) {
    let ty = follow_type::follow(ty);
    let index = node.index.as_str().unwrap_or("");
    if let Some(extern_type) = get_type::get::<ExternType>(ty) {
      if let Some(prop) = lookup_extern_type_prop(extern_type, index) {
        if prop.deprecated {
          self.report_property(
            &node.base.base.location,
            prop,
            Some(extern_type.name.as_str()),
            index,
          );
        } else if let Some(read_ty) = prop.read_ty {
          self.report_deprecated_function_member(node, read_ty, index);
        }
      }
      return;
    }
    let Some(table) = get_type::get::<TableType>(ty) else {
      return;
    };
    let Some(prop) = table.props.get(index) else {
      return;
    };
    if prop.deprecated {
      let container_name = table.name.as_ref().map(|name| {
        if name.starts_with("typeof(") && name.ends_with(')') {
          &name[7..name.len() - 1]
        } else {
          name.as_str()
        }
      });
      self.report_property(&node.base.base.location, prop, container_name, index);
    } else if let Some(read_ty) = prop.read_ty {
      self.report_deprecated_function_member(node, read_ty, index);
    }
  }
  /// 两个分支共用：`prop.read_ty` 指向弃用函数时的成员上报
  /// （C++ `node->expr->as<AstExprGlobal>()` 取容器名）。
  fn report_deprecated_function_member(
    &mut self,
    node: &AstExprIndexName,
    read_ty: TypeId,
    index: &str,
  ) {
    if let Some(fty) = get_type::get::<FunctionType>(follow_type::follow(read_ty))
      && fty.is_deprecated_function
      && !self.in_scope(fty)
    {
      // node.expr 已句柄化恒非空：.get() 安全借用后走 RTTI 门面判型。
      let container = ast_node_try_as::<AstExprGlobal>(&node.expr.get().base)
        .map(|global| global.name.as_str().unwrap_or(""));
      if let Some(info) = fty.deprecated_info.as_deref() {
        self.report_member_info(&node.base.base.location, container, index, info);
      } else {
        self.report_member(&node.base.base.location, container, index);
      }
    }
  }
  pub fn check_location_ast_name_ast_name(
    &mut self,
    location: &Location,
    global: AstName,
    index: AstName,
  ) {
    let Some(global_value) = self.context.get().builtin_globals.find(&global) else {
      return;
    };
    let Some(table) = get_type::get::<TableType>(global_value.r#type) else {
      return;
    };
    let index_name = index.as_str().unwrap_or("");
    if let Some(prop) = table.props.get(index_name)
      && prop.deprecated
    {
      self.report_property(location, prop, global.as_str(), index_name);
    }
  }
  pub fn check_ast_expr_function(&mut self, func: *mut AstExprFunction) {
    LUAU_ASSERT!(!func.is_null());
    let fty = self.get_function_type(func.cast::<AstExpr>());
    // Safety: fty 由 get_function_type 返回、可能为 null；本行经 `!fty.is_null()` 短路，仅对
    // 非空指针解引用读 is_deprecated_function（对齐 C++ `fty && fty->isDeprecatedFunction`），
    // fty 指向 arena 存活 FunctionType。
    let is_deprecated = !fty.is_null() && unsafe { (*fty).is_deprecated_function };
    if is_deprecated {
      self.push_scope(fty);
    }
    unsafe {
      // Safety: func 由方法起始 LUAU_ASSERT 保证非空、指向 arena 存活 AstExprFunction；作为基类
      // *mut AstExpr（repr(C) 首字段同址）遍历，单线程内 &mut self 作 visitor 独占、无别名。
      ast_expr_visit(func.cast::<AstExpr>(), self);
    }
    if is_deprecated {
      self.pop_scope(fty);
    }
  }
}

// —— 原 methods/lint_deprecated_api_get_function_type.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub fn get_function_type(&self, node: *mut AstExpr) -> *const FunctionType {
    // node 由 lint 遍历分发器保证存活；句柄是 Copy 局部副本，get 只借该副本。
    let mut handle = self.context;
    let ty = handle.get().get_type(node);
    ty.and_then(|t| get_type::get::<FunctionType>(follow_type::follow(t)))
      .map_or(null(), from_ref)
  }
}

// —— 原 methods/lint_deprecated_api_in_scope.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub fn in_scope(&self, fty: *const FunctionType) -> bool {
    self.function_type_scope_stack.contains(&fty)
  }
}

// —— 原 methods/lint_deprecated_api_pop_scope.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub fn pop_scope(&mut self, fty: *const FunctionType) {
    LUAU_ASSERT!(!fty.is_null());
    LUAU_ASSERT!(!self.function_type_scope_stack.is_empty());
    LUAU_ASSERT!(self.function_type_scope_stack.last() == Some(&fty));
    self.function_type_scope_stack.pop();
  }
}

// —— 原 methods/lint_deprecated_api_process.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  lint_stat_process!(
    #[inline(never)]
    LintDeprecatedApi {
      function_type_scope_stack: Vec::new()
    }
  );
}

// —— 原 methods/lint_deprecated_api_push_scope.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub fn push_scope(&mut self, fty: *const FunctionType) {
    LUAU_ASSERT!(!fty.is_null());
    self.function_type_scope_stack.push(fty);
  }
}

// —— 原 methods/lint_deprecated_api_report_linter.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub fn report_property(
    &mut self,
    location: &Location,
    prop: &Property,
    container: Option<&str>,
    field: &str,
  ) {
    let context = self.context.get();
    let suggestion = if prop.deprecated_suggestion.is_empty() {
      ""
    } else {
      prop.deprecated_suggestion.as_str()
    };
    if let Some(container) = container {
      if suggestion.is_empty() {
        emit_warning(
          context,
          Code::DeprecatedApi,
          *location,
          format_args!("Member '{}.{}' is deprecated", container, field),
        );
      } else {
        emit_warning(
          context,
          Code::DeprecatedApi,
          *location,
          format_args!(
            "Member '{}.{}' is deprecated, use '{}' instead",
            container, field, suggestion
          ),
        );
      }
    } else if suggestion.is_empty() {
      emit_warning(
        context,
        Code::DeprecatedApi,
        *location,
        format_args!("Member '{}' is deprecated", field),
      );
    } else {
      emit_warning(
        context,
        Code::DeprecatedApi,
        *location,
        format_args!(
          "Member '{}' is deprecated, use '{}' instead",
          field, suggestion
        ),
      );
    }
  }
  pub fn report_member(
    &mut self,
    location: &Location,
    table_name: Option<&str>,
    function_name: &str,
  ) {
    let context = self.context.get();
    if let Some(table_name) = table_name {
      emit_warning(
        context,
        Code::DeprecatedApi,
        *location,
        format_args!("Member '{}.{}' is deprecated", table_name, function_name),
      );
    } else {
      emit_warning(
        context,
        Code::DeprecatedApi,
        *location,
        format_args!("Member '{}' is deprecated", function_name),
      );
    }
  }
  pub fn report_member_info(
    &mut self,
    location: &Location,
    table_name: Option<&str>,
    function_name: &str,
    info: &DeprecatedInfo,
  ) {
    let use_part = info
      .use_suggestion()
      .map(|value| format!(", use '{}' instead", value))
      .unwrap_or_default();
    let reason_part = info
      .reason()
      .map(|value| format!(". {}", value))
      .unwrap_or_default();
    let context = self.context.get();
    if let Some(table_name) = table_name {
      emit_warning(
        context,
        Code::DeprecatedApi,
        *location,
        format_args!(
          "Member '{}.{}' is deprecated{}{}",
          table_name, function_name, use_part, reason_part
        ),
      );
    } else {
      emit_warning(
        context,
        Code::DeprecatedApi,
        *location,
        format_args!(
          "Member '{}' is deprecated{}{}",
          function_name, use_part, reason_part
        ),
      );
    }
  }
  pub fn report_function(&mut self, location: &Location, function_name: &str) {
    emit_warning(
      self.context.get(),
      Code::DeprecatedApi,
      *location,
      format_args!("Function '{}' is deprecated", function_name),
    );
  }
  pub fn report_function_info(
    &mut self,
    location: &Location,
    function_name: &str,
    info: &DeprecatedInfo,
  ) {
    let use_part = info
      .use_suggestion()
      .map(|value| format!(", use '{}' instead", value))
      .unwrap_or_default();
    let reason_part = info
      .reason()
      .map(|value| format!(". {}", value))
      .unwrap_or_default();
    emit_warning(
      self.context.get(),
      Code::DeprecatedApi,
      *location,
      format_args!(
        "Function '{}' is deprecated{}{}",
        function_name, use_part, reason_part
      ),
    );
  }
}

// —— 原 methods/lint_deprecated_api_visit_linter.rs ——
impl<'ctx> LintDeprecatedApi<'ctx> {
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    // Safety: node 由 AstVisitor::visit_expr_local 以 `from_mut(&mut AstExprLocal)` 派生，非空、
    // 对齐且其 `&mut` 引用者在本调用期间存活。fty 经 `!fty.is_null()` 判空后才解引用
    // （get_function_type 可返回 null）；`(*node).local` 为 parser 保证非空的 AstExprLocal 子节点，
    // location 经 base 链同源有效。
    unsafe {
      let fty = self.get_function_type(node.cast::<AstExpr>());
      let should_report = !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty);
      if should_report {
        // 显式把 raw 指针后的节点句柄解引用为引用，再走常规自动解引用链
        // （implicit autoref 与显式自动解引用两条 lint 在此行互斥，绑定中间变量消解）
        let local = &*(*node).local;
        let name = local.name.as_str().unwrap_or("");
        if let Some(info) = (*fty).deprecated_info.as_deref() {
          self.report_function_info(&(*node).base.base.location, name, info);
        } else {
          self.report_function(&(*node).base.base.location, name);
        }
      }
    }
    true
  }
  pub(crate) fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
    // Safety: node 由 visit_expr_global 以 `from_mut(&mut AstExprGlobal)` 派生，非空对齐且引用者
    // 在本调用内存活。fty 经 `!fty.is_null()` 判空后才解引用；`(*node).name` 为节点内嵌 AstName
    // （按值读取），location 走 base 链有效。
    unsafe {
      let fty = self.get_function_type(node.cast::<AstExpr>());
      let should_report = !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty);
      if should_report {
        let name = (*node).name.as_str().unwrap_or("");
        if let Some(info) = (*fty).deprecated_info.as_deref() {
          self.report_function_info(&(*node).base.base.location, name, info);
        } else {
          self.report_function(&(*node).base.base.location, name);
        }
      }
    }
    true
  }
  pub(crate) fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
    // Safety: node 由 visit_stat_local_function 以 `from_mut(&mut AstStatLocalFunction)` 派生，
    // 非空对齐、本调用内存活；`(*node).func` 已句柄化为 Node（parser 保证非空），`as_ptr`
    // 桥交仍以指针形态消费的 check_ast_expr_function。
    unsafe {
      self.check_ast_expr_function((*node).func.as_ptr());
    }
    false
  }
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    // Safety: node 由 visit_stat_function 以 `from_mut(&mut AstStatFunction)` 派生，非空对齐、
    // 本调用内存活；`(*node).func` 已句柄化为 Node（parser 保证非空），`as_ptr` 桥交
    // 仍以指针形态消费的 check_ast_expr_function。
    unsafe {
      self.check_ast_expr_function((*node).func.as_ptr());
    }
    false
  }
  pub(crate) fn visit_ast_expr_index_name(&mut self, node: *mut AstExprIndexName) -> bool {
    // Safety: node 由 visit_expr_index_name 以 `from_mut(&mut AstExprIndexName)` 派生，非空对齐、
    // 本调用内存活。`(*node).expr` 为 parser 保证非空的子表达式；ast_node_try_as_ptr 依 RTTI
    // class index 分派；location/index 为节点内嵌字段。
    unsafe {
      if let Some(ty) = self.context.get().get_type((*node).expr.as_ptr()) {
        self.check_ast_expr_index_name_type_id(&*node, ty);
      } else if let Some(global) = ast_node_try_as_ptr::<AstExprGlobal>((*node).expr) {
        self.check_location_ast_name_ast_name(
          &(*node).base.base.location,
          global.name,
          (*node).index,
        );
      }
    }
    true
  }
  pub(crate) fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
    // Safety: node 由 visit_expr_call 以 `from_mut(&mut AstExprCall)` 派生，非空对齐、本调用内存活；
    // self_/args 为节点内嵌字段。fenv 由 ast_node_try_as_ptr 判空并安全下转；
    // `node.args[0]` 由上方 `args.is_empty()` 提前返回保证 size≥1，level 为 parser 非空实参。
    let node = unsafe { &*node };
    if node.self_ || node.args.is_empty() {
      return true;
    }
    let Some(fenv) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(node.func) }) else {
      return true;
    };
    let fenv_bytes = fenv.name.as_bytes();
    if fenv_bytes != b"getfenv" && fenv_bytes != b"setfenv" {
      return true;
    }
    let level = node.args[0];
    let ty = self.context.get().get_type(level);
    let level_is_number =
      ty.is_some_and(is_number) || unsafe { ast_node_is_ptr::<AstExprConstantNumber>(level) };
    if level_is_number {
      let suggestion = if fenv_bytes == b"getfenv" {
        "; consider using 'debug.info' instead"
      } else {
        ""
      };
      let function_name = fenv.name;
      emit_warning(
        self.context.get(),
        Code::DeprecatedApi,
        node.base.base.location,
        format_args!("Function '{}' is deprecated{}", function_name, suggestion),
      );
    }
    true
  }
}
