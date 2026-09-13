//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:369:is_empty`
//! Source: `Analysis/src/TypePack.cpp:369-378` (hand-ported)

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::type_pack::TypePack,
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};

pub(crate) fn is_empty(tp: TypePackId) -> bool {
  unsafe {
    let tp = follow_type_pack_id(tp);
    if let Some(tpp) = TypePack::get_if(&(*tp).ty) {
      return tpp.head.is_empty()
        && match tpp.tail {
          Some(tail) => is_empty(tail),
          None => true,
        };
    }

    false
  }
}
