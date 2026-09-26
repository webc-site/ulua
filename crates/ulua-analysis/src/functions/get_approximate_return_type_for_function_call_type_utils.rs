use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{function_type::FunctionType, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(
  mut ty: TypeId,
  seen: &mut DenseHashSet<TypeId>,
) -> Option<TypePackId> {
  ty = follow_type::follow(ty);

  if seen.contains(&ty) {
    return None;
  }

  seen.insert(ty);

  if let Some(ftv) = get_type::get::<FunctionType>(ty) {
    return Some(ftv.ret_types);
  }

  // C++ `utv && begin(utv) != end(utv)` 后取 `*begin(utv)`——UnionTypeIterator
  // 展平嵌套 union 并 follow，裸取 options[0] 会拿到未展平的内层 union。
  if let Some(utv) = get_type::get::<UnionType>(ty) {
    let first_ty = begin_union_type(utv).next()?;
    return get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(
      first_ty, seen,
    );
  }

  None
}

pub fn get_approximate_return_type_for_function_call_type_id(ty: TypeId) -> Option<TypePackId> {
  let mut seen = DenseHashSet::default();
  get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(ty, &mut seen)
}
