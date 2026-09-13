use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, ast_node::AstNode,
};

/// # Safety
/// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn is_identifier(node: *mut AstNode) -> bool {
  if node.is_null() {
    return false;
  }
  let node = unsafe { &*node };
  node.is::<AstExprGlobal>() || node.is::<AstExprLocal>()
}
