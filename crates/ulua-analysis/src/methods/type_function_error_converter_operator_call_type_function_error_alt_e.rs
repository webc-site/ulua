use alloc::string::String;

use ulua_common::functions::format::format;

use crate::records::{
  type_function_error_converter::TypeFunctionErrorConverter,
  type_function_missing::TypeFunctionMissing,
};

impl TypeFunctionErrorConverter {
  pub fn operator_call_3(&self, e: &TypeFunctionMissing) -> String {
    format(format_args!(
      "Could not find '{}' type function in the global scope",
      e.function_name()
    ))
  }
}
