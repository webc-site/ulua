//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:127:to_string`
//! Source: `Analysis/include/Luau/ToString.h:127-131` (hand-ported)

use alloc::string::String;

/// C++ `inline std::string to_string(const TypePackVar& tp)`.
use crate::functions::to_string_to_string_alt_p::to_string_type_pack_var_to_string_options;
use crate::records::{to_string_options::ToStringOptions, type_pack_var::TypePackVar};
pub fn to_string_type_pack_var(tp: &TypePackVar) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_pack_var_to_string_options(tp, &mut opts)
}
