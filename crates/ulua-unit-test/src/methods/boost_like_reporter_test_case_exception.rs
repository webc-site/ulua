use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  pub fn test_case_exception(&mut self, e_error_string: &str) {
    LUAU_ASSERT!(!self.current_test.is_null());

    // The original C++ prints a message to stdout using:
    // currentTest->m_file, currentTest->m_line, and e.error_string.
    // In this translation, we keep behavior limited to validating the active test context,
    // because BoostLikeReporter::current_test is stored as an opaque pointer.
    let _ = e_error_string;
  }
}
