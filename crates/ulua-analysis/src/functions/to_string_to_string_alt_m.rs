//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1722:to_string`
//! Source: `Analysis/src/ToString.cpp:1722-1725` (hand-ported)

use alloc::string::String;

use crate::{
  functions::to_string_detailed_to_string::to_string_detailed,
  records::to_string_options::ToStringOptions, type_aliases::type_id::TypeId,
};

/// C++ `std::string to_string(TypeId ty, ToStringOptions& opts)`.
pub fn to_string_type_id_to_string_options(ty: TypeId, opts: &mut ToStringOptions) -> String {
  to_string_detailed(ty, opts).name
}
