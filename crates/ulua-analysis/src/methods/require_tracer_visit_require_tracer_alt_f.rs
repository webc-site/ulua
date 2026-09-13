use core::ffi::c_void;

use crate::records::require_tracer::RequireTracer;
impl RequireTracer {
  pub fn visit_ast_type_pack(&mut self, _node: *mut c_void) -> bool {
    true
  }
}
