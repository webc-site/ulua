use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{function_type::FunctionType, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(
  mut ty: TypeId,
  seen: &mut DenseHashSet<TypeId>,
) -> Option<TypePackId> {
  ty = follow_type_id(ty);

  if seen.contains(&ty) {
    return None;
  }

  seen.insert(ty);

  if let Some(ftv) = get_type_id::<FunctionType>(ty) {
    return Some(ftv.ret_types);
  }

  if let Some(utv) = get_type_id::<UnionType>(ty)
    && !utv.options.is_empty()
  {
    let first_ty = utv.options[0];
    return get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(
      first_ty, seen,
    );
  }

  None
}
