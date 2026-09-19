use alloc::string::String;

use crate::records::{
  constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
  error_converter::ErrorConverter,
};

impl ErrorConverter {
  pub fn operator_call_18(&self, _e: &ConstraintSolvingIncompleteError) -> String {
    String::from(
      "Type inference failed to complete, you may see some confusing types and type errors.",
    )
  }
}
