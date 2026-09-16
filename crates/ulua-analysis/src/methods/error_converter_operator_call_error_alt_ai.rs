use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, normalization_too_complex::NormalizationTooComplex,
};

impl ErrorConverter {
  pub fn operator_call_48(&self, _error: &NormalizationTooComplex) -> String {
    String::from("Code is too complex to typecheck! Consider simplifying the code around this area")
  }
}
