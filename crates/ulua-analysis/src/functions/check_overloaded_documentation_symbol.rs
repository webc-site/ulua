use alloc::string::String;
use core::ptr::from_ref;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_node::AstNode},
  rtti::ast_node_try_as,
};

use crate::{
  functions::{follow_type, get_type, to_string_to_string::to_string_type_id},
  records::{intersection_type::IntersectionType, module::Module},
  type_aliases::type_id::TypeId,
};

/// 对应 C++ `checkOverloadedDocumentationSymbol`：仅当 `ty` follow 后是
/// `IntersectionType` 且 `parent_expr` 动态类型为 `AstExprCall` 时，查
/// `module.ast_overload_resolved_types` 为该调用点选中的重载并追加
/// `/overload/<打印类型>` 后缀；否则原样返回文档符号。
///
/// §2：cpp 可空 `const AstExpr*` 形参收口为 `Option<&AstExpr>`——判空与
/// class 判别统一走安全 RTTI 门面 `ast_node_try_as`，`null_mut` 哨兵消失。
pub fn check_overloaded_documentation_symbol(
  module: &Module,
  ty: TypeId,
  parent_expr: Option<&AstExpr>,
  documentation_symbol: Option<String>,
) -> Option<String> {
  let documentation_symbol = documentation_symbol?;

  let follow_ty = follow_type::follow(ty);
  if get_type::get::<IntersectionType>(follow_ty).is_none() {
    return Some(documentation_symbol);
  }

  // parent 动态类型为 AstExprCall 时按节点指针身份键查选中重载。
  // ast_overload_resolved_types 以 `*const AstNode` 指针值为键（arena 身份键
  // 既有约定），此处只取地址值作查找键，不解引用。
  let matching_overload = match parent_expr {
    Some(pe) if ast_node_try_as::<AstExprCall>(pe).is_some() => module
      .ast_overload_resolved_types
      .find(&from_ref(pe).cast::<AstNode>())
      .copied(),
    _ => None,
  };

  if let Some(matching_overload) = matching_overload {
    let mut overload_symbol = documentation_symbol;
    overload_symbol.push_str("/overload/");
    let ty_str = to_string_type_id(matching_overload);
    overload_symbol.push_str(&ty_str);
    Some(overload_symbol)
  } else {
    Some(documentation_symbol)
  }
}
