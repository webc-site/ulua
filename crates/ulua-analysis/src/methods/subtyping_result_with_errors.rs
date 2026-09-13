use crate::{records::subtyping_result::SubtypingResult, type_aliases::error_vec::ErrorVec};

impl SubtypingResult {
  pub fn with_errors(&mut self, err: &mut ErrorVec) -> &mut Self {
    for e in err.iter() {
      self.with_error(e.clone());
    }
    self
  }
}
