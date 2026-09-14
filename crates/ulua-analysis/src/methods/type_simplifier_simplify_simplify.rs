use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{records::type_simplifier::TypeSimplifier, type_aliases::type_id::TypeId};
impl TypeSimplifier {
  pub fn simplify_type_id(&mut self, ty: TypeId) -> TypeId {
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());
    self.simplify_type_id_dense_hash_set_type_id(ty, &mut seen)
  }
}
