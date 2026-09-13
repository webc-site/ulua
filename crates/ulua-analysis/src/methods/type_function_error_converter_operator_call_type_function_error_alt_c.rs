use alloc::string::String;

use crate::records::{
  runtime_error::RuntimeError, type_function_error_converter::TypeFunctionErrorConverter,
};

impl TypeFunctionErrorConverter {
  pub fn operator_call_2(&self, e: &RuntimeError) -> String {
    String::from(e.message())
  }
}
