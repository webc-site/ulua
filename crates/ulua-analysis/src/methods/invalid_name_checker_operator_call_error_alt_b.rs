use crate::records::{
  cannot_extend_table::CannotExtendTable, invalid_name_checker::InvalidNameChecker,
};

impl InvalidNameChecker {
  pub fn operator_call(&self, e: &CannotExtendTable) -> bool {
    let invalid_name: &String = unsafe { &*(self as *const Self as *const String) };
    e.prop() == invalid_name
  }
}
