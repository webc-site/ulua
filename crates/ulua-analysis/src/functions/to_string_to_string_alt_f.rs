//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:121:to_string`
//! Source: `Analysis/include/Luau/ToString.h:121-125` (hand-ported)

use alloc::string::String;

/// C++ `inline std::string to_string(const Type& tv)`.
use crate::functions::to_string_to_string_alt_o::to_string_type_item_to_string_options;
use crate::records::{to_string_options::ToStringOptions, r#type::Type};
pub fn to_string_type_item(tv: &Type) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_item_to_string_options(tv, &mut opts)
}
