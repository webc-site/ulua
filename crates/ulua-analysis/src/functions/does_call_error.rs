use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_global::AstExprGlobal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn does_call_error(call: &AstExprCall) -> bool {
  let global = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };

  if global.is_null() {
    return false;
  }

  unsafe {
    let name = (*global).name.value;
    if name.is_null() {
      return false;
    }

    let name_bytes = CStr::from_ptr(name).to_bytes();
    if name_bytes == b"error" {
      return true;
    }

    if name_bytes == b"assert" {
      // assert() will error because it is missing the first argument
      let first_arg = match call.args.iter().next() {
        Some(arg) => *arg,
        None => return true,
      };

      if first_arg.is_null() {
        return false;
      }

      let expr = ast_node_as::<AstExprConstantBool>(first_arg as *mut AstNode);

      if expr.is_null() {
        return false;
      }

      if !(*expr).value {
        return true;
      }
    }
  }

  false
}
