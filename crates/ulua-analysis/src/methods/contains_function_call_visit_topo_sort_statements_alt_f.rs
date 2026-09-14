use core::ffi::c_void;

use ulua_ast::records::ast_stat_function::AstStatFunction;

use crate::records::contains_function_call::ContainsFunctionCall;
impl ContainsFunctionCall {
  pub fn visit_ast_stat_function(&mut self, node: *mut c_void) -> bool {
    let _node = node as *mut AstStatFunction;

    false
  }
}
