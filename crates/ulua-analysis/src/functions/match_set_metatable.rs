use core::ffi::CStr;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};
pub fn match_set_metatable(call: &AstExprCall) -> bool {
  if call.args.len() != 2 {
    return false;
  }

  let func_as_global = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };

  if func_as_global.is_null() {
    return false;
  }

  let name = unsafe { (*func_as_global).name.value };
  if name.is_null() {
    return false;
  }

  let name_bytes = unsafe { CStr::from_ptr(name).to_bytes() };
  name_bytes == b"setmetatable"
}
