use alloc::string::String;

use crate::{
  functions::to_string_to_string::{
    to_string_type_id_to_string_options, to_string_type_pack_id_to_string_options,
  },
  records::to_string_options::ToStringOptions,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn to_string_vector_vector_type_id_to_string_options(
  types: &[TypeId],
  opts: &mut ToStringOptions,
) -> String {
  let mut s = String::new();
  for &ty in types.iter() {
    if !s.is_empty() {
      s.push_str(", ");
    }
    s.push_str(&to_string_type_id_to_string_options(ty, opts));
  }
  s
}

pub fn to_string_vector_vector_type_pack_id_to_string_options(
  type_packs: &[TypePackId],
  opts: &mut ToStringOptions,
) -> String {
  let mut s = String::new();
  for &type_pack in type_packs.iter() {
    if !s.is_empty() {
      s.push_str(", ");
    }
    s.push_str(&to_string_type_pack_id_to_string_options(type_pack, opts));
  }
  s
}
