//! Source: `Analysis/src/Normalize.cpp:875-923` (hand-ported)
//!
//! Two `isCacheable` overloads from Normalize.cpp:
//!   * `isCacheable(TypePackId, Set<TypeId>&)`  (Normalize.cpp:875-894)
//!   * `isCacheable(TypeId, Set<TypeId>&)`       (Normalize.cpp:896-923)
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id,
  },
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack, free_type::FreeType,
    free_type_pack::FreeTypePack, pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub(crate) fn is_cacheable_type_pack_id_set_type_id(
  tp: TypePackId,
  seen: &mut DenseHashSet<TypeId>,
) -> bool {
  let tp = unsafe { follow_type_pack_id(tp) };

  let mut it = begin_type_pack_id(tp);
  let end_it = end(tp);

  while it.operator_ne(&end_it) {
    if !is_cacheable(*it.operator_deref(), seen) {
      return false;
    }
    it.operator_inc();
  }

  if let Some(tail) = it.tail()
    && (get_type_pack_id::<FreeTypePack>(tail).is_some()
      || get_type_pack_id::<BlockedTypePack>(tail).is_some()
      || get_type_pack_id::<TypeFunctionInstanceTypePack>(tail).is_some())
  {
    return false;
  }

  true
}

pub fn is_cacheable(ty: TypeId, seen: &mut DenseHashSet<TypeId>) -> bool {
  if seen.contains(&ty) {
    return true;
  }
  seen.insert(ty);

  let ty = follow_type_id(ty);

  if get_type_id::<FreeType>(ty).is_some()
    || get_type_id::<BlockedType>(ty).is_some()
    || get_type_id::<PendingExpansionType>(ty).is_some()
  {
    return false;
  }

  if let Some(tfi) = get_type_id::<TypeFunctionInstanceType>(ty) {
    for t in &tfi.type_arguments {
      if !is_cacheable(*t, seen) {
        return false;
      }
    }
    for tp in &tfi.pack_arguments {
      if !is_cacheable_type_pack_id_set_type_id(*tp, seen) {
        return false;
      }
    }
  }

  true
}
