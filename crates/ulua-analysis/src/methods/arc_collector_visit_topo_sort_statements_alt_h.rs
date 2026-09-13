use core::ffi::c_void;

use crate::records::arc_collector::ArcCollector;
impl ArcCollector {
  pub fn visit_ast_type(&mut self, _node: *mut c_void) -> bool {
    true
  }
}
