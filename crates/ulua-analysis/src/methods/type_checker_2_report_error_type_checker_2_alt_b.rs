use crate::records::{type_checker_2::TypeChecker2, type_error::TypeError};

impl TypeChecker2 {
  pub fn report_error_type_error(&mut self, e: TypeError) {
    self.report_error_type_error_data_location(e.data, &e.location);
  }
}
