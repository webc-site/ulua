use alloc::vec::Vec;

use crate::records::error_snapshot::ErrorSnapshot;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeCheckLog {
  pub errors: Vec<ErrorSnapshot>,
}
