use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
  },
  rtti,
};

use crate::{
  functions::{
    follow_type, get_type, return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
  },
  records::{extern_type::ExternType, union_type::UnionType},
  type_aliases::module_ptr_module::ModulePtr,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_method_containing_extern_type(
  module: &ModulePtr,
  func_expr: *mut AstExpr,
) -> Option<*const ExternType> {
  // Safety: `func_expr` 依本 `unsafe fn` 契约与全部调用点实参（判型/判空后的
  // 表达式根节点）为指向存活 `AstExpr` 的非空指针；此处集中一次派生共享借用
  // 供两个 class_index 判定，读的是 `#[repr(C)]` 首字段 `base`（基址重合）。
  let expr_ref = unsafe { &*func_expr };
  let parent_expr = if expr_ref.base.class_index == rtti::ast_rtti_index("AstExprIndexName") {
    // Safety: class index 唯一命中 AstExprIndexName ⇒ `func_expr` 动态类型即
    // 为该节点；`#[repr(C)]` 继承链保证派生与基址重合，向下转借用同一存活
    // 节点，只读 `expr` 字段；其已句柄化恒非空，parent_expr 行走链经 as_ptr 桥接。
    let index_name = unsafe { &*(func_expr.cast::<AstExprIndexName>()) };
    index_name.expr.as_ptr()
  } else if expr_ref.base.class_index == rtti::ast_rtti_index("AstExprIndexExpr") {
    // Safety: 上一分支同理——命中类型唯一为 AstExprIndexExpr，基址重合成立。
    let index_expr = unsafe { &*(func_expr.cast::<AstExprIndexExpr>()) };
    // expr 已句柄化恒非空；parent_expr 行走链为既有裸指针 API，经 as_ptr 桥接。
    index_expr.expr.as_ptr()
  } else {
    return None;
  };

  let parent_it = module.ast_types.find(&(parent_expr as *const AstExpr));
  let parent_it = *parent_it?;

  let parent_type = follow_type::follow(parent_it);

  if let Some(extern_ty) = get_type::get::<ExternType>(parent_type) {
    return Some(extern_ty as *const ExternType);
  }

  if let Some(union_ty) = get_type::get::<UnionType>(parent_type) {
    // 引用 → 裸指针 upcast（安全转换，语义与 C++ 返回指针一致）。
    return return_first_nonnull_option_of_type::<ExternType>(union_ty)
      .map(|ty| ty as *const ExternType);
  }

  None
}
