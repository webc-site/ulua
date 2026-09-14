use ulua_analysis::records::check_result::CheckResult;

use crate::records::fixture::Fixture;

impl Fixture {
  /// C++ `void Fixture::dumpErrors(const CheckResult& cr)` (Fixture.cpp):
  /// dump once, formatting the errors and emitting a doctest `MESSAGE` when
  /// non-empty. As with the sibling overloads, the actual message emission is
  /// handled by the upstream test harness (`get_errors` formatting is a
  /// translation artifact — see `dump_errors_ostream_vector_type_error`).
  pub fn dump_errors_check_result(&mut self, cr: &CheckResult) {
    if self.has_dumped_errors {
      return;
    }
    self.has_dumped_errors = true;
    let error = self.get_errors(cr);
    if !error.is_empty() {
      eprintln!("{}", error);
    }
  }
}
