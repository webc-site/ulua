use alloc::string::String;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_node::AstNode},
  rtti,
  rtti::AstNodePtr,
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
/// `parent_expr` 可为 null（cpp 可空 `const AstExpr*`），判空与 class 读取统一
/// 走 RTTI 边界门面 `rtti::ast_node_is_ptr`（null 恒返回 false，绝不解引用）。
pub fn check_overloaded_documentation_symbol(
  module: &Module,
  ty: TypeId,
  parent_expr: *const AstExpr,
  documentation_symbol: Option<String>,
) -> Option<String> {
  let documentation_symbol = documentation_symbol?;

  let follow_ty = follow_type::follow(ty);
  let intersection_ptr = get_type::get::<IntersectionType>(follow_ty);
  if intersection_ptr.is_none() {
    return Some(documentation_symbol);
  }

  // 判空 + repr(C) 基址读 class_index 的 RTTI 判别收口在边界门面
  // ast_node_is_ptr（null 恒为 false，绝不解引用；单线程只读）。
  // as_ast_node() 为零解引用的类型视图转换（rtti.rs「safe，永不解引用」），
  // 使本公开函数体不直接解引用裸指针参数（clippy not_unsafe_ptr_arg_deref
  // 同款写法同本文件既有 ast_node_try_as_ptr 调用侧）。
  // Safety: parent_expr 契约即函数级文档（null 或 arena 存活 AstExpr），只读判定。
  let matching_overload =
    if unsafe { rtti::ast_node_is_ptr::<AstExprCall>(parent_expr.as_ast_node()) } {
      let node_ptr = parent_expr as *const AstNode;
      module.ast_overload_resolved_types.find(&node_ptr).copied()
    } else {
      None
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
