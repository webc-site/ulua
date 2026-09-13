use alloc::string::{String, ToString};

use crate::records::{
  error_converter::ErrorConverter, generic_type_count_mismatch::GenericTypeCountMismatch,
};

impl ErrorConverter {
  pub fn operator_call_10(&self, e: &GenericTypeCountMismatch) -> String {
    let mut result = String::from("Different number of generic type parameters: subtype had ");
    result.push_str(&e.sub_ty_generic_count.to_string());
    result.push_str(", supertype had ");
    result.push_str(&e.super_ty_generic_count.to_string());
    result.push('.');
    result
  }
}
