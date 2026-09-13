use alloc::string::String;

use crate::{enums::status_require_navigator::Status, records::navigator::Navigator};
impl Navigator<'_> {
  pub fn navigate(&mut self, mut path: String) -> Status {
    // Replace backslashes with forward slashes
    path = path.replace('\\', "/");

    if let Some(error) = self.navigate_impl(path.as_str()) {
      self.error_handler.report_error(error);
      return Status::ErrorReported;
    }

    Status::Success
  }
}
