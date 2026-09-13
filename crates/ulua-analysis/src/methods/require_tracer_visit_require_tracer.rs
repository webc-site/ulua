use core::ffi::c_void;

use crate::records::require_tracer::RequireTracer;

impl RequireTracer {
  pub fn visit_ast_expr_type_assertion(&mut self, _node: *mut c_void) -> bool {
    // suppress `require() :: any`
    false
  }
}
