extern crate alloc;

use alloc::string::String;

use crate::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PrettyPrintResult {
  pub code: String,
  pub error_location: Location,
  pub parse_error: String,
}
