//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2141:to_string`
//! Source: `Analysis/src/ToString.cpp:2141-2149` (hand-ported)

use alloc::string::String;

/// C++ `std::string to_string(const TypeOrPack& tyOrTp, ToStringOptions& opts)`.
use crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use crate::{
  functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
  records::to_string_options::ToStringOptions,
  type_aliases::{
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
  },
};
pub fn to_string_type_or_pack_to_string_options(
  ty_or_tp: &TypeOrPack,
  opts: &mut ToStringOptions,
) -> String {
  if let Some(ty) = TypeId::get_if(ty_or_tp) {
    to_string_type_id_to_string_options(*ty, opts)
  } else if let Some(tp) = TypePackId::get_if(ty_or_tp) {
    to_string_type_pack_id_to_string_options(*tp, opts)
  } else {
    unreachable!("LUAU_UNREACHABLE: TypeOrPack has exactly two members")
  }
}
