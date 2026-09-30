use alloc::string::String;

use crate::{
  functions::{
    dump_options::dump_options,
    to_string_vector_to_string::{
      to_string_vector_vector_type_id_to_string_options,
      to_string_vector_vector_type_pack_id_to_string_options,
    },
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn dump_vector_type_id(types: &[TypeId]) -> String {
  let mut opts = dump_options();
  to_string_vector_vector_type_id_to_string_options(types, &mut opts)
}

pub fn dump_vector_type_pack_id(type_packs: &[TypePackId]) -> String {
  let mut opts = dump_options();
  to_string_vector_vector_type_pack_id_to_string_options(type_packs, &mut opts)
}
