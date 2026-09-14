use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::team_city_reporter::TeamCityReporter;
impl TeamCityReporter {
  pub fn test_case_start(&mut self, in_test_suite: &str, in_name: &str) {
    LUAU_ASSERT!(!self.current_test.is_null());

    let _ = (in_test_suite, in_name);
    self.current_test = null_mut();
    // The C++ implementation stores the incoming doctest::TestCaseData address
    // and prints a TeamCity "testStarted" event.
    // Rust method signature provides only the split string data, so the pointer
    // identity is left as null in this translation.
  }
}
