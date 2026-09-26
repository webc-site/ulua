//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:225-243, hand-ported)

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type_pack, get_type_pack},
  records::type_pack::TypePack,
  type_aliases::type_pack_id::TypePackId,
};
pub(crate) fn get_tail(mut tp: TypePackId) -> TypePackId {
  let mut seen: DenseHashSet<TypePackId> = DenseHashSet::default();
  while !tp.is_null() {
    tp = follow_type_pack::follow(tp);

    if seen.contains(&tp) {
      break;
    }
    seen.insert(tp);

    if let Some(pack) = get_type_pack::get::<TypePack>(tp)
      && let Some(tail) = pack.tail
    {
      tp = tail;
      continue;
    }
    break;
  }

  follow_type_pack::follow(tp)
}
