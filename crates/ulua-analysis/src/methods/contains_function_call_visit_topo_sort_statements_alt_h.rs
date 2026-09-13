use core::ffi::c_void;

use crate::records::contains_function_call::ContainsFunctionCall;
impl ContainsFunctionCall {
  pub fn visit_ast_type(&mut self, node: *mut c_void) -> bool {
    let _ = node;
    true
  }
}
