//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:225:getTail`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:225-243, hand-ported)

use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id},
  records::type_pack::TypePack,
  type_aliases::type_pack_id::TypePackId,
};
pub(crate) fn get_tail(mut tp: TypePackId) -> TypePackId {
  let mut seen: DenseHashSet<TypePackId> = DenseHashSet::new(null());
  while !tp.is_null() {
    tp = unsafe { follow_type_pack_id(tp) };

    if seen.contains(&tp) {
      break;
    }
    seen.insert(tp);

    if let Some(pack) = get_type_pack_id::<TypePack>(tp)
      && let Some(tail) = pack.tail
    {
      tp = tail;
      continue;
    }
    break;
  }

  unsafe { follow_type_pack_id(tp) }
}
