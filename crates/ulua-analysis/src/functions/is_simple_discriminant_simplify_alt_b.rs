use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::is_simple_discriminant_simplify::is_simple_discriminant, type_aliases::type_id::TypeId,
};
pub fn is_simple_discriminant_type_id(ty: TypeId) -> bool {
  let mut seen_set = DenseHashSet::new(null());
  is_simple_discriminant(ty, &mut seen_set)
}
