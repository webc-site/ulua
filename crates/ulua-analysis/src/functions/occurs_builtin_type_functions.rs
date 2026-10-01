use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::union_type::UnionType,
  type_aliases::type_id::TypeId,
};

pub fn occurs_with_seen(
  mut haystack: TypeId,
  needle: TypeId,
  seen: &mut DenseHashSet<TypeId>,
) -> bool {
  haystack = follow_type::follow(haystack);

  if needle == haystack {
    return true;
  }
  if seen.contains(&haystack) {
    return false;
  }
  seen.insert(haystack);

  // C++ `for (auto option : ut)`——UnionTypeIterator 展平嵌套 union 并
  // follow,裸遍历 options 会漏掉嵌套成员。
  if let Some(ut) = get_type::get::<UnionType>(haystack) {
    for option in begin_union_type(ut) {
      if occurs_with_seen(option, needle, seen) {
        return true;
      }
    }
  }
  false
}

pub fn occurs(haystack: TypeId, needle: TypeId) -> bool {
  let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
  occurs_with_seen(haystack, needle, &mut seen)
}
