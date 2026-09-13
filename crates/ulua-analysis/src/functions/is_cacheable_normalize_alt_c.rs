use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{functions::is_cacheable_normalize_alt_b::is_cacheable, type_aliases::type_id::TypeId};
pub fn is_cacheable_type_id(ty: TypeId) -> bool {
  let mut seen = DenseHashSet::new(null());
  is_cacheable(ty, &mut seen)
}
