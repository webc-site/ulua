use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, generic_type_count_mismatch::GenericTypeCountMismatch,
};

impl ErrorConverter {
  pub fn operator_call_10(&self, e: &GenericTypeCountMismatch) -> String {
    format!(
      "Different number of generic type parameters: subtype had {}, supertype had {}.",
      e.sub_ty_generic_count, e.super_ty_generic_count
    )
  }
}
