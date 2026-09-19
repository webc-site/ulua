use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn match_require(call: &AstExprCall) -> Option<*mut AstExpr> {
  const REQUIRE: &str = "require";

  if call.args.len() != 1 {
    return None;
  }

  // SAFETY: call.func 指向 AST arena 内节点，rtti 向下转换安全。
  let func_as_global = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };
  if func_as_global.is_null() {
    return None;
  }

  // SAFETY: func_as_global 由转换保证非空且指向存活节点。
  // AstName::as_bytes 容忍 null（空名 → 空切片，必然不等于 "require"）。
  let name_bytes = unsafe { (*func_as_global).name.as_bytes() };
  if name_bytes != REQUIRE.as_bytes() {
    return None;
  }

  // SAFETY: 前置检查 args.len() == 1，首元素必存在。
  Some(unsafe { *call.args.begin() })
}
