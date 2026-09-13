extern crate alloc;

use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Completion {
  pub completion: String,
  pub display: String,
}
