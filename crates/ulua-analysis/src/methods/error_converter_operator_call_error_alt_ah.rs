use alloc::string::String;

use crate::{
  functions::{
    to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    error_converter::ErrorConverter, to_string_options::ToStringOptions,
    types_are_unrelated::TypesAreUnrelated,
  },
};

impl ErrorConverter {
  pub fn operator_call_55(&self, e: &TypesAreUnrelated) -> String {
    let opts = ToStringOptions::default();
    let left_str = to_string_type_id_to_string_options_mut(e.left, opts);
    let right_str = to_string_type_id(e.right);
    format!(
      "Cannot cast '{}' into '{}' because the types are unrelated",
      left_str, right_str
    )
  }
}
