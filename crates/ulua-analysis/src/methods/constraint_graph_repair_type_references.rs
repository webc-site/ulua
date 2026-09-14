use core::ptr::null;

use crate::{
  functions::{
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  records::{constraint_graph::ConstraintGraph, dense_hash_set::DenseHashSet},
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl ConstraintGraph {
  pub fn repair_type_references_type_id(&mut self, mut ty: TypeId) {
    let root = follow_type_id(ty);

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
    let _ = seen.insert(root);

    while !seen.contains(&ty) {
      let _ = seen.insert(ty);
      self.shift_references_type_id(ty, root);

      if let Some(bt) = get_type_id::<BoundType>(ty) {
        ty = bt.bound_to;
      } else {
        break;
      }
    }
  }

  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn repair_type_references_type_pack_id(&mut self, mut ty: TypePackId) {
    let root = unsafe { follow_type_pack_id(ty) };

    let mut seen: DenseHashSet<TypePackId> = DenseHashSet::new(null());
    let _ = seen.insert(root);

    while !seen.contains(&ty) {
      let _ = seen.insert(ty);
      self.shift_references_type_pack_id(ty, root);

      if let Some(bt) = get_type_pack_id::<BoundTypePack>(ty) {
        ty = bt.bound_to;
      } else {
        break;
      }
    }
  }
}
