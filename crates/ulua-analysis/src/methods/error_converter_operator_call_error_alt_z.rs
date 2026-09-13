use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_d::to_string_type_pack_id,
  records::{
    error_converter::ErrorConverter,
    function_exits_without_returning::FunctionExitsWithoutReturning,
  },
};

impl ErrorConverter {
  pub fn operator_call_25(&self, e: &FunctionExitsWithoutReturning) -> String {
    let expected_type_str = to_string_type_pack_id(e.expected_return_type);
    format!(
      "Not all codepaths in this function return '{}'.",
      expected_type_str
    )
  }
}
