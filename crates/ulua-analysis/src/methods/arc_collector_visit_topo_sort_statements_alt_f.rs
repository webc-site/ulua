use core::ffi::c_void;

use crate::records::arc_collector::ArcCollector;
impl ArcCollector {
  pub fn visit_ast_stat_assign(&mut self, node: *mut c_void) -> bool {
    let _ = node;
    true
  }
}
