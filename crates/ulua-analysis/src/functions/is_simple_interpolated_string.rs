use ulua_ast::records::{ast_expr_interp_string::AstExprInterpString, ast_node::AstNode};

/// # Safety
/// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn is_simple_interpolated_string(node: *const AstNode) -> bool {
  if node.is_null() {
    return false;
  }

  let interp_string = unsafe { (*node).as_item::<AstExprInterpString>() };

  if interp_string.is_null() {
    return false;
  }

  unsafe { (*interp_string).expressions.is_empty() }
}
