use alloc::string::String;

use crate::{
  functions::{
    to_string_to_string_alt_c::to_string_type_id, to_string_to_string_alt_d::to_string_type_pack_id,
  },
  records::{ambiguous_function_call::AmbiguousFunctionCall, error_converter::ErrorConverter},
};

impl ErrorConverter {
  pub fn operator_call(&self, afc: &AmbiguousFunctionCall) -> String {
    let function_str = to_string_type_id(afc.function);
    let arguments_str = to_string_type_pack_id(afc.arguments);
    format!(
      "Calling function {} with argument pack {} is ambiguous.",
      function_str, arguments_str
    )
  }
}
