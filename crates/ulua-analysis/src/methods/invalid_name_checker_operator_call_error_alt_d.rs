use crate::records::invalid_name_checker::InvalidNameChecker;

impl InvalidNameChecker {
  pub fn operator_call_4<T>(&self, _other: &T) -> bool {
    false
  }
}
