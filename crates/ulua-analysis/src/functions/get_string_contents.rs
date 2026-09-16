use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_interp_string::AstExprInterpString,
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// # Safety
/// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_string_contents(node: *const AstNode) -> Option<String> {
  if node.is_null() {
    return None;
  }

  // SAFETY: node 非 null，指向 AST arena 节点。
  let node_ref = unsafe { &*node };

  if let Some(string_node) = ast_node_try_as::<AstExprConstantString>(node_ref) {
    return Some(String::from_utf8_lossy(string_node.value.as_bytes()).into_owned());
  }
  if let Some(interp_string) = ast_node_try_as::<AstExprInterpString>(node_ref)
    && interp_string.expressions.is_empty()
  {
    LUAU_ASSERT!(interp_string.strings.len() == 1);
    let first_string_array = interp_string.strings.as_slice().first()?;
    return Some(String::from_utf8_lossy(first_string_array.as_bytes()).into_owned());
  }

  None
}
