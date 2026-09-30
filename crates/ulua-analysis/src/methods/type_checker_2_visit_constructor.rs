//! C++ `TypeChecker2::visitConstructor`（`Analysis/src/TypeChecker2.cpp:1479-1563`）。
//!
//! classdef `__init` 构造函数的字段初始化检查：校验 `self` 形参、收集需要
//! 初始化的非 nilable 字段、随无条件赋值语句收缩集合，最后统一上报
//! `UninitializedFieldAccess`。

use alloc::string::String;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_assign::AstStatAssign, ast_stat_class::AstStatClass, ast_type::AstType,
  },
  rtti::ast_node_try_as,
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::{LUAU_ASSERT, fflag, records::dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::alias_ref, find_uninitialized_accesses::FindUninitializedAccesses,
    syntax_error::SyntaxError, type_checker_2::TypeChecker2,
    uninitialized_field_access::UninitializedFieldAccess,
  },
  type_aliases::type_error_data::TypeErrorData,
};

impl TypeChecker2 {
  /// cpp `visitConstructor`。`stat`/`method` 为当前模块 AST arena 存活节点的
  /// 共享引用（原 `*mut AstStatClass` 形参已引用化，非空由引用类型承载），
  /// `method.function` 经 alias_ref 门面换引用；指针仅保留身份比较与
  /// visit 门面（visit.rs 契约）的传参用途。
  pub fn visit_constructor(&mut self, stat: &AstStatClass, method: &AstClassMethod) {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    let function = alias_ref(method.function);

    if function.args.is_empty() {
      self.report_error_type_error_data_location(
        TypeErrorData::SyntaxError(SyntaxError::new(String::from(
          "__init must have at least one parameter.",
        ))),
        &method.name_location,
      );
      return;
    }

    let self_local = function.args[0].as_ptr();

    if function.args[0].name != "self" {
      self.report_error_type_error_data_location(
        TypeErrorData::SyntaxError(SyntaxError::new(String::from(
          "__init's first parameter must be named 'self'.",
        ))),
        &function.args[0].location,
      );
      return;
    }

    let mut uninitialized_fields: DenseHashSet<String> = DenseHashSet::default();
    let mut method_names: DenseHashSet<String> = DenseHashSet::default();
    for member in stat.members.as_slice() {
      if let Some(prop) = member.get_if::<AstClassProperty>() {
        if prop.ty.is_null() {
          // 无标注字段视作 nilable，无需初始化。
          continue;
        }

        if let Some(&prop_ty) = self
          .module_ref()
          .ast_resolved_types
          .get(&(prop.ty as *const AstType))
        {
          // TypeChecker2 推进不变量：进入任何访问前必已压入当前 scope，
          // 栈非空蕴含 last() 命中 Some（与 check_for_type_function_inhabitance 同一契约）。
          let scope = *self
            .stack
            .last()
            .expect("推进不变量：访问期栈内恒有当前 scope");
          let result = self
            .subtyping_mut()
            .is_subtype_type_id_type_id_not_null_scope(
              self.builtin_types.get().nil_type,
              prop_ty,
              // 栈顶句柄还原裸指针传参（`not_null_scope` 形参，仅取址不解引用）。
              scope.get(),
            );
          if result.is_subtype {
            continue;
          }
          // TODO CLI-222651: 还要支持 error suppressing 类型
        }

        uninitialized_fields.insert(String::from(prop.name.as_str_or_empty()));
      } else if let Some(class_method) = member.get_if::<AstClassMethod>() {
        method_names.insert(String::from(class_method.function_name.as_str_or_empty()));
      }
    }

    let fields_ptr: *mut DenseHashSet<String> = &raw mut uninitialized_fields;
    // Safety: `fields_ptr` 指向本帧的 `uninitialized_fields`（new 为 unsafe
    // fn，契约同注释）。Rust 局部变量按声明逆序析构，`finder` 后声明先析构，
    // 其裸指针在 `uninitialized_fields` 失效之前即不再被使用，与 C++ 中二者
    // 同生死的裸指针共享语义一致。
    let mut finder =
      unsafe { FindUninitializedAccesses::new(self_local, fields_ptr, method_names) };

    for stmt in function.body.body.iter_nodes() {
      let stmt_ref = stmt.get();
      // 只检查无条件执行的赋值语句。
      if let Some(assignment) = ast_node_try_as::<AstStatAssign>(stmt_ref) {
        // 先在赋值右侧搜索越界的字段访问（ast_expr_visit 为 visit.rs 的
        // 指针门面 unsafe fn，节点来自 assignment.values 的 arena 元素）。
        for value in assignment.values.iter() {
          // Safety: value 是 assignment.values 数组界内元素，parser 写入的
          // arena 存活节点。
          unsafe { ast_expr_visit(*value, &mut finder) };
        }

        // 再登记哪些字段已初始化（iter_nodes 安全门面直出元素引用）。
        for var in assignment.vars.iter_nodes() {
          match var.as_expr_ref() {
            AstExprRef::IndexName(index_name) => {
              if let AstExprRef::Local(local) = index_name.expr.as_expr_ref()
                && local.local.as_ptr() == self_local
              {
                uninitialized_fields.erase_str(index_name.index.as_str_or_empty());
              }
            }
            AstExprRef::IndexExpr(index_expr) => {
              if let AstExprRef::Local(local) = index_expr.expr.as_expr_ref()
                && local.local.as_ptr() == self_local
                && let AstExprRef::ConstantString(str_expr) = index_expr.index.as_expr_ref()
              {
                // erase_str 借用口直接吃 Cow<str> 视图，免 into_owned 物化。
                let key = String::from_utf8_lossy(str_expr.value.as_bytes());
                uninitialized_fields.erase_str(&key);
              }
            }
            _ => {}
          }
        }
      } else {
        // Safety: stmt 是 function.body.body 界内元素（arena 存活节点），
        // ast_stat_visit 为 visit.rs 的指针门面 unsafe fn。
        unsafe { ast_stat_visit(stmt.as_ptr(), &mut finder) };
      }
    }

    if let Some(expr) = finder.violating_ref {
      self.report_error_type_error_data_location(
        TypeErrorData::UninitializedFieldAccess(UninitializedFieldAccess::new(None)),
        &alias_ref(expr).base.location,
      );
    }

    for (key, expr) in finder.violating_fields.iter() {
      // 类型检查器只对常量字符串给出具体字段名的错误。
      self.report_error_type_error_data_location(
        TypeErrorData::UninitializedFieldAccess(UninitializedFieldAccess::new(Some(key.clone()))),
        &alias_ref(*expr).base.location,
      );
    }
  }
}
