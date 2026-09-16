use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
  },
  rtti,
};

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
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
  let parent_expr =
    if unsafe { (*func_expr).base.class_index == rtti::ast_rtti_index("AstExprIndexName") } {
      let index_name = unsafe { &*(func_expr as *mut AstExprIndexName) };
      index_name.expr
    } else if unsafe { (*func_expr).base.class_index == rtti::ast_rtti_index("AstExprIndexExpr") } {
      let index_expr = unsafe { &*(func_expr as *mut AstExprIndexExpr) };
      index_expr.expr
    } else {
      return None;
    };

  let parent_it = module.ast_types.find(&(parent_expr as *const AstExpr));
  let parent_it = *parent_it?;

  let parent_type = follow_type_id(parent_it);

  if let Some(extern_ty) = get_type_id::<ExternType>(parent_type) {
    return Some(extern_ty as *const ExternType);
  }

  if let Some(union_ty) = get_type_id::<UnionType>(parent_type) {
    // SAFETY: union_ty 由有效 TypeId 下转而来，前提与 C++ 一致
    return unsafe { return_first_nonnull_option_of_type::<ExternType>(union_ty) };
  }

  None
}
