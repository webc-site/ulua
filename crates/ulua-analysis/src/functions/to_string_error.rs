//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Error.cpp:1447:to_string`
//! Source: `Analysis/src/Error.cpp:1447-1450` (hand-ported)

use alloc::string::String;

use crate::{
  functions::to_string_error_alt_k::to_string_type_error_type_error_to_string_options,
  records::{type_error::TypeError, type_error_to_string_options::TypeErrorToStringOptions},
};

/// C++ `std::string to_string(const TypeError& error)`.
pub fn to_string_type_error(error: &TypeError) -> String {
  to_string_type_error_type_error_to_string_options(error, TypeErrorToStringOptions::default())
}
