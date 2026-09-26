use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end, follow_type, follow_type_pack, get_type,
    get_type_pack,
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
  let tp = follow_type_pack::follow(tp);

  let mut it = begin(tp);
  let end_it = end(tp);

  while it != end_it {
    if !is_cacheable(*it.current(), seen) {
      return false;
    }
    it.advance();
  }

  if let Some(tail) = it.tail()
    && (get_type_pack::get::<FreeTypePack>(tail).is_some()
      || get_type_pack::get::<BlockedTypePack>(tail).is_some()
      || get_type_pack::get::<TypeFunctionInstanceTypePack>(tail).is_some())
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

  let ty = follow_type::follow(ty);

  if get_type::get::<FreeType>(ty).is_some()
    || get_type::get::<BlockedType>(ty).is_some()
    || get_type::get::<PendingExpansionType>(ty).is_some()
  {
    return false;
  }

  if let Some(tfi) = get_type::get::<TypeFunctionInstanceType>(ty) {
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

pub fn is_cacheable_type_id(ty: TypeId) -> bool {
  let mut seen = DenseHashSet::default();
  is_cacheable(ty, &mut seen)
}
