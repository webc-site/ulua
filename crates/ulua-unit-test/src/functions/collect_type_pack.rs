use alloc::vec::Vec;

use ulua_analysis::{
  functions::{begin_type_pack::begin, end_type_pack::end},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn collect_type_pack(type_pack: TypePackId) -> Vec<TypeId> {
  let mut result = Vec::new();
  let mut it = begin(type_pack);
  let end_it = end(type_pack);

  while it.operator_ne(&end_it) {
    result.push(*it.operator_deref());
    it.operator_inc();
  }

  result
}
