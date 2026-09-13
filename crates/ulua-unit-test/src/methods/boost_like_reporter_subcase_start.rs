use core::ffi::c_void;

use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  pub fn subcase_start(&mut self, _signature: *const c_void) {
    // Empty implementation: the C++ method body is empty
  }
}
