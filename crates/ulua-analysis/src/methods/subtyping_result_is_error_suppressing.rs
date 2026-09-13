use crate::records::subtyping_result::SubtypingResult;

impl SubtypingResult {
  pub fn is_error_suppressing(&self) -> bool {
    self.is_error_suppressing
  }
}
