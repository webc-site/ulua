use core::ffi::CStr;

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
  let name_ptr = unsafe { (*func_as_global).name.value };
  if name_ptr.is_null() {
    return None;
  }

  // SAFETY: AST 名称字面量为 arena 内 NUL 结尾字符串。
  let name_bytes = unsafe { CStr::from_ptr(name_ptr).to_bytes() };
  if name_bytes != REQUIRE.as_bytes() {
    return None;
  }

  // SAFETY: 前置检查 args.len() == 1，首元素必存在。
  Some(unsafe { *call.args.begin() })
}
