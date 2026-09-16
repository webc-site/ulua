use alloc::string::String;

use crate::{
  records::{
    type_function_error::TypeFunctionError,
    type_function_error_converter::TypeFunctionErrorConverter,
  },
  type_aliases::type_function_error_data::TypeFunctionErrorData,
};

/// C++ `std::string to_string(const TypeFunctionError& error)`
/// (TypeFunctionError.cpp:68).
pub fn to_string(error: &TypeFunctionError) -> String {
  let converter = TypeFunctionErrorConverter {};
  match &error.data {
    TypeFunctionErrorData::V0(e) => converter.operator_unsupported_type(e),
    TypeFunctionErrorData::V1(e) => converter.operator_unsupported_type_pack(e),
    TypeFunctionErrorData::V2(e) => converter.operator_runtime_error(e),
    TypeFunctionErrorData::V3(e) => converter.operator_failed_to_compile(e),
    TypeFunctionErrorData::V4(e) => converter.operator_type_function_missing(e),
  }
}

pub use to_string as to_string_type_function_error;
