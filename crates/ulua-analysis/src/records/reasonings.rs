use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Reasonings {
  /// the list of reasons
  pub reasons: Vec<String>,
  /// this should be true if _all_ of the reasons have an error suppressing type, and false otherwise.
  pub suppressed: bool,
}

unsafe impl Send for Reasonings {}
unsafe impl Sync for Reasonings {}
