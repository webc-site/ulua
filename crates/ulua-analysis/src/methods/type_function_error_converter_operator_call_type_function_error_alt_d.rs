use alloc::string::String;

use ulua_common::functions::format::format;

use crate::records::{
  failed_to_compile::FailedToCompile, type_function_error_converter::TypeFunctionErrorConverter,
};

impl TypeFunctionErrorConverter {
  pub fn operator_call(&self, e: &FailedToCompile) -> String {
    format(format_args!(
      "'{}' type function failed to compile with error message: {}",
      e.function_name(),
      e.compile_error()
    ))
  }
}
