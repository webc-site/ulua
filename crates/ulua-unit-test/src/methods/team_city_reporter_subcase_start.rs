use core::ffi::c_void;

use crate::records::team_city_reporter::TeamCityReporter;
impl TeamCityReporter {
  pub fn subcase_start(&mut self, _signature: *const c_void) {
    // Empty implementation: the C++ method body is empty
  }
}
