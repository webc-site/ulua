use crate::records::{
  duplicate_type_definition::DuplicateTypeDefinition, invalid_name_checker::InvalidNameChecker,
};

impl InvalidNameChecker {
  pub fn operator_call_2(&self, e: &DuplicateTypeDefinition) -> bool {
    let invalid_name: &String = unsafe { &*(self as *const Self as *const String) };
    e.name() == invalid_name
  }
}
