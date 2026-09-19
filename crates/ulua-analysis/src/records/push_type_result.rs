use alloc::vec::Vec;

use crate::records::incomplete_inference::IncompleteInference;
#[derive(Debug, Clone, Default)]
pub struct PushTypeResult {
  pub incomplete_types: Vec<IncompleteInference>,
}
