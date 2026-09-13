use alloc::string::String;
use core::{ffi::CStr, mem::swap};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString, ast_expr_global::AstExprGlobal,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::type_guard::TypeGuard;
pub fn match_type_guard(op: i32, left: *mut AstExpr, right: *mut AstExpr) -> Option<TypeGuard> {
  if op != AstExprBinaryOp::CompareEq as i32 && op != AstExprBinaryOp::CompareNe as i32 {
    return None;
  }

  let mut left = left;
  let mut right = right;

  if unsafe { !right.is_null() && !ast_node_as::<AstExprCall>(right as *mut AstNode).is_null() } {
    swap(&mut left, &mut right);
  }

  let call = unsafe { ast_node_as::<AstExprCall>(left as *mut AstNode) };
  let string = unsafe { ast_node_as::<AstExprConstantString>(right as *mut AstNode) };

  if call.is_null() || string.is_null() {
    return None;
  }

  let call = unsafe { &*call };
  let string = unsafe { &*string };

  let callee = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };
  if callee.is_null() {
    return None;
  }

  let callee_name = unsafe { (*callee).name.value };
  if callee_name.is_null() {
    return None;
  }

  let name_bytes = unsafe { CStr::from_ptr(callee_name).to_bytes() };
  let is_typeof = if name_bytes == b"typeof" {
    true
  } else if name_bytes == b"type" {
    false
  } else {
    return None;
  };

  if call.args.len() != 1 {
    return None;
  }

  let type_str = String::from_utf8_lossy(string.value.as_bytes()).into_owned();

  Some(TypeGuard {
    is_typeof,
    target: call.args.as_slice()[0],
    r#type: type_str,
  })
}
