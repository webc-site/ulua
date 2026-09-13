use crate::records::{invalid_name_checker::InvalidNameChecker, unknown_property::UnknownProperty};

impl InvalidNameChecker {
  pub fn operator_call_3(&self, e: &UnknownProperty) -> bool {
    let invalid_name: &String = unsafe { &*(self as *const Self as *const String) };
    e.key() == invalid_name
  }
}
