use core::ffi::c_void;

use crate::records::contains_function_call::ContainsFunctionCall;

impl ContainsFunctionCall {
  pub fn visit_ast_expr_function(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
