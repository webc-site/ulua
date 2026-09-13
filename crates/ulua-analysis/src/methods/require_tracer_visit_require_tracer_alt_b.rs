use core::ffi::{CStr, c_void};

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::records::require_tracer::RequireTracer;
impl RequireTracer {
  pub fn visit_ast_expr_call(&mut self, node: *mut c_void) -> bool {
    let expr = unsafe { &*(node as *mut AstExprCall) };
    let global = unsafe { ast_node_as::<AstExprGlobal>(expr.func as *mut AstNode) };

    if !global.is_null() {
      let global_ref = unsafe { &*global };
      let name_ptr = global_ref.name.value;
      let is_require = unsafe {
        let c_str = CStr::from_ptr(name_ptr);
        c_str.to_bytes() == b"require"
      };

      if is_require && expr.args.size >= 1 {
        self.require_calls.push(node as *mut AstExprCall);
      }
    }

    true
  }
}
