use alloc::vec::Vec;

use crate::{
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    intersection_type::IntersectionType, never_type::NeverType, r#type::Type, type_ids::TypeIds,
    type_remover::TypeRemover, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeRemover {
  /// C++ `void TypeRemover::process(TypeId item)` (Generalization.cpp:669-719).
  pub fn process(&mut self, item: TypeId) {
    let item = follow_type_id(item);

    // If we've already visited this item, or it's outside our arena, then
    // do not try to mutate it.
    if self.seen.contains(&item)
      || unsafe { (*item).owning_arena } != self.arena
      || unsafe { (*item).persistent }
    {
      return;
    }
    self.seen.insert(item);

    if let Some(ut) = get_mutable_type_id::<UnionType>(item) {
      let options: Vec<TypeId> = ut.options.clone();
      let old_size = ut.options.len();
      let mut new_options = TypeIds::new();
      for option in options {
        self.process(option);
        let option = follow_type_id(option);
        if option != self.needle && get_type_id::<NeverType>(option).is_none() && option != item {
          new_options.insert_type_id(option);
        }
      }
      if old_size != new_options.size() {
        if new_options.empty() {
          emplace_bound_type(item, self.builtin_never_type());
        } else if new_options.size() == 1 {
          let first = new_options.front();
          emplace_bound_type(item, first);
        } else {
          let taken = new_options.take();
          let new_ty = unsafe { (*self.arena).add_type(UnionType { options: taken }) };
          emplace_bound_type(item, new_ty);
        }
      }
      return;
    }

    if let Some(it) = get_mutable_type_id::<IntersectionType>(item) {
      let parts: Vec<TypeId> = it.parts.clone();
      let old_size = it.parts.len();
      let mut new_parts = TypeIds::new();
      for part in parts {
        self.process(part);
        let part = follow_type_id(part);
        if part != self.needle && get_type_id::<UnknownType>(part).is_none() && part != item {
          new_parts.insert_type_id(part);
        }
      }
      if old_size != new_parts.size() {
        if new_parts.empty() {
          emplace_bound_type(item, self.builtin_unknown_type());
        } else if new_parts.size() == 1 {
          let first = new_parts.front();
          emplace_bound_type(item, first);
        } else {
          let taken = new_parts.take();
          let new_ty = unsafe { (*self.arena).add_type(IntersectionType { parts: taken }) };
          emplace_bound_type(item, new_ty);
        }
      }
    }
  }

  #[inline]
  fn builtin_never_type(&self) -> TypeId {
    unsafe { (*self.builtin_types).never_type }
  }

  #[inline]
  fn builtin_unknown_type(&self) -> TypeId {
    unsafe { (*self.builtin_types).unknown_type }
  }
}

/// C++ `emplaceType<BoundType>(asMutable(item), bound_to)`: replace the type's
/// variant in place with a `BoundType` pointing at `bound_to`.
fn emplace_bound_type(item: TypeId, bound_to: TypeId) {
  unsafe {
    let m = item as *mut Type;
    (*m).ty = TypeVariant::Bound(bound_to);
  }
}
