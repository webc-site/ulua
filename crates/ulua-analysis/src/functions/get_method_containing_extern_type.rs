use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr::AstExpr};

use crate::{
  functions::{
    follow_type, get_type, return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
  },
  records::{arena_handle::alias_opt, extern_type::ExternType, union_type::UnionType},
  type_aliases::module_ptr_module::ModulePtr,
};

/// `func_expr` 为 parse-arena 节点句柄（cpp 原样指针身份）；解引用统一收口
/// 至 `alias_opt`，本函数对业务侧为 safe 调用。
pub fn get_method_containing_extern_type(
  module: &ModulePtr,
  func_expr: *mut AstExpr,
) -> Option<*const ExternType> {
  let parent_expr = match alias_opt(func_expr)?.as_expr_ref() {
    AstExprRef::IndexName(index_name) => index_name.expr.as_ptr(),
    AstExprRef::IndexExpr(index_expr) => index_expr.expr.as_ptr(),
    _ => return None,
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
