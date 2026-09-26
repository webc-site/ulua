//! C++ `FindUninitializedAccesses`（`Analysis/src/TypeChecker2.cpp:1397-1476`）。
//!
//! classdef `__init` 构造函数体内的未初始化字段访问探测：只上报「确实存在、
//! 尚未初始化、且被用作右值」的字段访问；`self` 整体或经方法调用的提前使用
//! 记为无字段名的 `violating_ref`。

use alloc::string::String;
use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_local::AstLocal, ast_visitor::AstVisitor,
  },
  rtti::AstNodePtr,
};
use ulua_common::{
  LUAU_ASSERT, fflag,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

#[derive(Debug)]
pub struct FindUninitializedAccesses {
  /// C++ `NotNull<AstLocal> self`。
  pub self_: *mut AstLocal,
  /// C++ `NotNull<DenseHashSet<std::string>> uninitializedFields`：随构造函数体
  /// 逐条赋值语句被 `visit_constructor` 就地收缩，故与外层共享同一对象。
  pub uninitialized_fields: *mut DenseHashSet<String>,
  pub method_names: DenseHashSet<String>,

  /// C++ `std::optional<AstExpr*> violatingRef`。
  pub violating_ref: Option<*mut AstExpr>,
  /// C++ `DenseHashMap<std::string, AstExpr*> violatingFields`（`try_insert` 语义：
  /// 同名字段只记首个违规访问）。
  pub violating_fields: DenseHashMap<String, *mut AstExpr>,
}

impl FindUninitializedAccesses {
  /// C++ 构造函数：`self`/`uninitializedFields` 为借用式共享指针，语义同 NotNull。
  ///
  /// # Safety
  /// `self_`、`uninitialized_fields` 须指向本帧内存期内存有效的对象。
  pub unsafe fn new(
    self_: *mut AstLocal,
    uninitialized_fields: *mut DenseHashSet<String>,
    method_names: DenseHashSet<String>,
  ) -> Self {
    // 对照 C++ 构造函数体内的 `LUAU_ASSERT(FFlag::DebugLuauUserDefinedClasses)`。
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
    LUAU_ASSERT!(!self_.is_null());
    LUAU_ASSERT!(!uninitialized_fields.is_null());
    Self {
      self_,
      uninitialized_fields,
      method_names,
      violating_ref: None,
      violating_fields: DenseHashMap::default(),
    }
  }
}

impl AstVisitor for FindUninitializedAccesses {
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    // SAFETY: `uninitialized_fields` 由 `new` 注入，指向遍历期内存有效的对象。
    unsafe {
      // local 槽已句柄化恒非空；与 self.self_ 的身份比较经 as_ptr 桥接。
      if node.local.as_ptr() == self.self_ && !(*self.uninitialized_fields).empty() {
        self.violating_ref = Some(from_mut(&mut node.base));
      }
    }
    // C++：命中或未命中都不再下钻（AstExprLocal 无子节点）。
    false
  }

  fn visit_expr_index_name(&mut self, node: &mut AstExprIndexName) -> bool {
    // SAFETY: `node.expr` 指向遍历期间存活的 AST 节点，`uninitialized_fields` 同上。
    unsafe {
      let expr = &*(node.expr.as_ast_node());
      if !expr.is::<AstExprLocal>() {
        return true;
      }
      if (node.expr.cast::<AstExprLocal>()).local.as_ptr() != self.self_ {
        return true;
      }

      // 字段不存在的情况由其它机制上报；这里只管「存在、未初始化、被右值读取」。
      let field_name = String::from(node.index.as_str_or_empty());
      let uninitialized = &*self.uninitialized_fields;
      if uninitialized.contains(&field_name) {
        self
          .violating_fields
          .try_insert(field_name, from_mut(&mut node.base));
      } else if self.method_names.contains(&field_name) && !uninitialized.empty() {
        self.violating_ref = Some(from_mut(&mut node.base));
      }
    }
    false
  }

  fn visit_expr_index_expr(&mut self, node: &mut AstExprIndexExpr) -> bool {
    // SAFETY: `node.expr` / `node.index` 指向遍历期间存活的 AST 节点，`uninitialized_fields` 同上。
    unsafe {
      let expr = &*(node.expr.as_ast_node());
      if !expr.is::<AstExprLocal>()
        || (node.expr.cast::<AstExprLocal>()).local.as_ptr() != self.self_
      {
        return true;
      }
      let index = &*(node.index.as_ast_node());
      if !index.is::<AstExprConstantString>() {
        return true;
      }
      let str_expr = &*(node.index.cast::<AstExprConstantString>());
      let key = String::from_utf8_lossy(str_expr.value.as_bytes()).into_owned();

      let uninitialized = &*self.uninitialized_fields;
      if uninitialized.contains(&key) {
        self
          .violating_fields
          .try_insert(key, from_mut(&mut node.base));
      } else if self.method_names.contains(&key) && !uninitialized.empty() {
        self.violating_ref = Some(from_mut(&mut node.base));
      }
      false
    }
  }

  fn visit_expr_type_assertion(&mut self, node: &mut AstExprTypeAssertion) -> bool {
    // `self :: T` 断言视为逃逸口，不再下钻（C++ 同款豁免）。
    // SAFETY: `node.expr` 指向遍历期间存活的 AST 节点。
    unsafe {
      let expr = &*(node.expr.as_ast_node());
      if expr.is::<AstExprLocal>()
        && (node.expr.cast::<AstExprLocal>()).local.as_ptr() == self.self_
      {
        return false;
      }
    }
    true
  }
}
