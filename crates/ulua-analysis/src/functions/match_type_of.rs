use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};
pub fn match_type_of(call: &AstExprCall) -> bool {
  if call.args.len() != 1 {
    return false;
  }

  let func_as_global = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };

  if func_as_global.is_null() {
    return false;
  }

  unsafe {
    // AstName::as_bytes 容忍 null（空名 → 空切片，必然不匹配）。
    let name_bytes = (*func_as_global).name.as_bytes();
    if name_bytes != b"typeof" && name_bytes != b"type" {
      return false;
    }
  }

  true
}
