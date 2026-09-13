use crate::{records::type_checker::TypeChecker, type_aliases::error_vec::ErrorVec};

impl TypeChecker {
  pub fn report_errors(&mut self, errors: &ErrorVec) {
    for err in errors {
      self.report_error_type_error(err);
    }
  }
}
