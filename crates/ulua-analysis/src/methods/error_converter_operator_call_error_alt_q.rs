use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, unification_too_complex::UnificationTooComplex,
};

impl ErrorConverter {
  pub fn operator_call_41(&self, _: &UnificationTooComplex) -> String {
    String::from(
      "Internal error: Code is too complex to typecheck! Consider adding type annotations around this area",
    )
  }
}
