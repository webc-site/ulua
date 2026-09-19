use core::ptr::null_mut;

use crate::records::lint_format_string::LintFormatString;
impl LintFormatString {
  pub fn fuzz(&self, data: &[u8]) {
    let mut pass = self.clone();
    pass.context = null_mut();

    pass.check_string_format(data);
    pass.check_string_pack(data, false);
    let _ = pass.check_string_match(data);
    pass.check_string_replace(data, -1);
    pass.check_date_format(data);
  }
}
