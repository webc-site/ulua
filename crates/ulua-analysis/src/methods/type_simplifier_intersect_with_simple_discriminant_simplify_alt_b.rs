use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{records::type_simplifier::TypeSimplifier, type_aliases::type_id::TypeId};
impl TypeSimplifier {
  pub fn intersect_with_simple_discriminant_type_id_type_id(
    &self,
    target: TypeId,
    discriminant: TypeId,
  ) -> Option<TypeId> {
    let mut seen_set = DenseHashSet::new(null());
    self.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
      target,
      discriminant,
      &mut seen_set,
    )
  }
}
