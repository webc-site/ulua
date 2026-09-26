use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type, follow_type_pack, get_type, get_type_pack},
  records::constraint_graph::ConstraintGraph,
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl ConstraintGraph {
  pub fn repair_type_references_type_id(&mut self, mut ty: TypeId) {
    let root = follow_type::follow(ty);

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
    let _ = seen.insert(root);

    while !seen.contains(&ty) {
      let _ = seen.insert(ty);
      self.shift_references_type_id(ty, root);

      if let Some(bt) = get_type::get::<BoundType>(ty) {
        ty = bt.bound_to;
      } else {
        break;
      }
    }
  }

  pub(crate) fn repair_type_references_type_pack_id(&mut self, mut ty: TypePackId) {
    let root = follow_type_pack::follow(ty);

    let mut seen: DenseHashSet<TypePackId> = DenseHashSet::default();
    let _ = seen.insert(root);

    while !seen.contains(&ty) {
      let _ = seen.insert(ty);
      self.shift_references_type_pack_id(ty, root);

      if let Some(bt) = get_type_pack::get::<BoundTypePack>(ty) {
        ty = bt.bound_to;
      } else {
        break;
      }
    }
  }
}
