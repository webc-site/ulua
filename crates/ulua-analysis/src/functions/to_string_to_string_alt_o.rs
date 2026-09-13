//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1732:to_string`
//! Source: `Analysis/src/ToString.cpp:1732-1735` (hand-ported)

use alloc::string::String;

/// C++ `std::string to_string(const Type& tv, ToStringOptions& opts)`.
use crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use crate::{
  records::{to_string_options::ToStringOptions, r#type::Type},
  type_aliases::type_id::TypeId,
};
pub fn to_string_type_item_to_string_options(tv: &Type, opts: &mut ToStringOptions) -> String {
  to_string_type_id_to_string_options(tv as *const Type as TypeId, opts)
}
