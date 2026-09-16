//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1877:dump`
//! Source: `Analysis/src/ToString.cpp:1877-1880` (hand-ported)

use alloc::string::String;

/// C++ `std::string dump(const std::vector<TypePackId>& typePacks)`.
use crate::functions::to_string_vector_to_string_alt_b::to_string_vector_vector_type_pack_id_to_string_options;
use crate::{functions::dump_options::dump_options, type_aliases::type_pack_id::TypePackId};
pub fn dump_vector_type_pack_id(type_packs: &[TypePackId]) -> String {
  to_string_vector_vector_type_pack_id_to_string_options(type_packs, dump_options())
}
