use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, type_error::TypeError,
  type_error_to_string_options::TypeErrorToStringOptions,
};

/// C++ `std::string to_string(const TypeError& error)`.
pub fn to_string_type_error(error: &TypeError) -> String {
  to_string_type_error_type_error_to_string_options(error, TypeErrorToStringOptions::default())
}

/// C++ `std::string to_string(const TypeError& error, TypeErrorToStringOptions options)`.
pub fn to_string_type_error_type_error_to_string_options(
  error: &TypeError,
  options: TypeErrorToStringOptions<'_>,
) -> String {
  let converter = ErrorConverter::new(options.file_resolver);
  converter.convert(&error.data)
}
