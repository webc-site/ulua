use alloc::string::String;

use ulua_analysis::records::check_result::CheckResult;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn get_errors(&mut self, cr: &CheckResult) -> String {
    let ss = String::new();
    let mut fmt_args = core::format_args!("{}", "");
    self.dump_errors_ostream_vector_type_error(&mut fmt_args, &cr.errors);
    ss
  }
}
