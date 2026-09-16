//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:135:to_string_named_function`
//! Source: `Analysis/include/Luau/ToString.h:135-139` (hand-ported)

use alloc::string::String;

/// C++ `inline std::string toStringNamedFunction(const std::string& funcName, const FunctionType& ftv)`.
use crate::functions::to_string_named_function_to_string_alt_b::to_string_named_function_string_function_type_to_string_options;
use crate::records::{function_type::FunctionType, to_string_options::ToStringOptions};
pub fn to_string_named_function_string_function_type(
  func_name: &str,
  ftv: &FunctionType,
) -> String {
  let mut opts = ToStringOptions::default();
  to_string_named_function_string_function_type_to_string_options(func_name, ftv, &mut opts)
}
