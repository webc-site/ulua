use crate::records::{type_error::TypeError, unifier::Unifier};

impl Unifier {
  pub fn report_error_type_error(&mut self, err: TypeError) {
    self.errors.push(err);
    self.failure = true;
  }
}
