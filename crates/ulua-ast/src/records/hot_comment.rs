use alloc::string::String;

use crate::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HotComment {
  pub header: bool,
  pub location: Location,
  pub content: String,
}
