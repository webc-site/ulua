//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/ToString.h:92:to_string`
//! Source: `Analysis/include/Luau/ToString.h:92-96` (hand-ported)

use alloc::string::String;

/// C++ `inline std::string to_string(TypeId ty, ToStringOptions&& opts)`.
use crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use crate::{records::to_string_options::ToStringOptions, type_aliases::type_id::TypeId};
pub fn to_string_type_id_to_string_options_mut(ty: TypeId, mut opts: ToStringOptions) -> String {
  to_string_type_id_to_string_options(ty, &mut opts)
}
