use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type_id::follow_type_id, get_type_alt_j::get_type_id},
  records::union_type::UnionType,
  type_aliases::type_id::TypeId,
};

pub fn occurs(mut haystack: TypeId, needle: TypeId, seen: &mut DenseHashSet<TypeId>) -> bool {
  haystack = follow_type_id(haystack);

  if needle == haystack {
    return true;
  }
  if seen.contains(&haystack) {
    return false;
  }
  seen.insert(haystack);

  if let Some(ut) = get_type_id::<UnionType>(haystack) {
    for &option in &ut.options {
      if occurs(option, needle, seen) {
        return true;
      }
    }
  }
  false
}
