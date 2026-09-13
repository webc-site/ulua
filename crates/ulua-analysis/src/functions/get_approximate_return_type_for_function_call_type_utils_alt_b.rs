use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::get_approximate_return_type_for_function_call_type_utils::get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn get_approximate_return_type_for_function_call_type_id(ty: TypeId) -> Option<TypePackId> {
  let mut seen = DenseHashSet::new(null());
  get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(ty, &mut seen)
}
