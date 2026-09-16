use alloc::string::String;

use crate::records::{
  duplicate_type_definition::DuplicateTypeDefinition, error_converter::ErrorConverter,
};

impl ErrorConverter {
  pub fn operator_call_22(&self, e: &DuplicateTypeDefinition) -> String {
    match e.previous_location() {
      Some(previous_location) => format!(
        "Redefinition of type '{}', previously defined at line {}",
        e.name(),
        previous_location.begin.line + 1
      ),
      None => format!("Redefinition of type '{}'", e.name()),
    }
  }
}
