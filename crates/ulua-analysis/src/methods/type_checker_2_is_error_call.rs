use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_global::AstExprGlobal, ast_node::AstNode,
  },
  rtti,
};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `call` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn is_error_call(&mut self, call: *const AstExprCall) -> bool {
    let call = unsafe { &*call };
    let global = unsafe { rtti::ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };

    if global.is_null() {
      return false;
    }

    let name = unsafe { CStr::from_ptr((*global).name.value).to_str().unwrap_or("") };

    if name == "error" {
      return true;
    } else if name == "assert" {
      if call.args.size == 0 {
        return true;
      }

      let first_arg = unsafe { *call.args.data.add(0) };
      let constant_bool =
        unsafe { rtti::ast_node_as::<AstExprConstantBool>(first_arg as *mut AstNode) };

      if !constant_bool.is_null() && !unsafe { (*constant_bool).value } {
        return true;
      }
    }

    false
  }
}
