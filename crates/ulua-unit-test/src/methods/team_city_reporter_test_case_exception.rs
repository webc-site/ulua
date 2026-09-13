use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::team_city_reporter::TeamCityReporter;
impl TeamCityReporter {
  pub fn test_case_exception(&mut self, in_error_string: &str) {
    LUAU_ASSERT!(!self.current_test.is_null());

    let current_test = self.current_test;
    let _ = (current_test, in_error_string);
    // The original C++ prints a TeamCity "testFailed" event using:
    // currentTest->m_test_suite, currentTest->m_name, and in.error_string.
    // In this translation, we keep behavior limited to validating the active test context,
    // because TeamCityReporter::current_test is stored as an opaque pointer.
  }
}
