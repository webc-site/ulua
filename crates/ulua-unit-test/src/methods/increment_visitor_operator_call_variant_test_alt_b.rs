use core::ffi::c_int;

use crate::records::increment_visitor::IncrementVisitor;
impl IncrementVisitor {
  pub fn operator_call_mut(&self, v: &mut c_int) {
    *v += 1;
  }
}
