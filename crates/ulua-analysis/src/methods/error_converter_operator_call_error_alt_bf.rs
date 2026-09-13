use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, recursive_restraint_violation::RecursiveRestraintViolation,
};

impl ErrorConverter {
  pub fn operator_call_51(&self, _e: &RecursiveRestraintViolation) -> String {
    String::from("Recursive type being used with different parameters.")
  }
}
