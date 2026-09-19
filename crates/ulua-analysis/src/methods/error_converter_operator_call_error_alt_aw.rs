use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, user_defined_type_function_error::UserDefinedTypeFunctionError,
};

impl ErrorConverter {
  pub fn operator_call_62(&self, e: &UserDefinedTypeFunctionError) -> String {
    String::from(e.message())
  }
}
