use crate::{records::type_checker_2::TypeChecker2, type_aliases::error_vec::ErrorVec};

impl TypeChecker2 {
  pub fn report_errors(&mut self, errors: ErrorVec) {
    for e in errors {
      self.report_error_type_error(e);
    }
  }
}
