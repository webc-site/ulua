use core::ffi::c_void;

use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  pub fn test_case_reenter(&mut self, _test_case_data: &c_void) {
    // Empty implementation: the C++ method body is empty
  }
}
