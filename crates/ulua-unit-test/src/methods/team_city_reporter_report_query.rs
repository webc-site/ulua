use core::ffi::c_void;

use crate::records::team_city_reporter::TeamCityReporter;
impl TeamCityReporter {
  pub fn report_query(&mut self, _query_data: &c_void) {
    // Empty implementation: the C++ method body is empty
  }
}
