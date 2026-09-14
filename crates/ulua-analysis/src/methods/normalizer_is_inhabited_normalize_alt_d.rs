use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    intersection_type::IntersectionType, metatable_type::MetatableType, never_type::NeverType,
    normalizer::Normalizer, table_type::TableType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

struct RecursionCountGuard {
  count: *mut i32,
}

impl RecursionCountGuard {
  fn new(count: *mut i32) -> Self {
    // SAFETY: count 指向 shared_state 内的计数器，存活期覆盖 guard。
    unsafe {
      *count += 1;
    }
    Self { count }
  }
}

impl Drop for RecursionCountGuard {
  fn drop(&mut self) {
    // SAFETY: 同上。
    unsafe {
      debug_assert!(*self.count > 0);
      *self.count -= 1;
    }
  }
}

impl Normalizer {
  pub fn is_inhabited_type_id_set_type_id(
    &mut self,
    ty: TypeId,
    seen: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    // SAFETY: shared_state 在 Normalizer 存活期内有效。
    let _rc =
      RecursionCountGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });

    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }

    self.consume_fuel();

    let ty = follow_type_id(ty);

    if get_type_id::<NeverType>(ty).is_some() {
      return NormalizationResult::False;
    }

    if get_type_id::<IntersectionType>(ty).is_none()
      && get_type_id::<UnionType>(ty).is_none()
      && get_type_id::<TableType>(ty).is_none()
      && get_type_id::<MetatableType>(ty).is_none()
    {
      return NormalizationResult::True;
    }

    if seen.contains(&ty) {
      return NormalizationResult::True;
    }

    seen.insert(ty);

    if let Some(ttv) = get_type_id::<TableType>(ty) {
      for prop in ttv.props.values() {
        if self.use_new_luau_solver() {
          if let Some(ty) = prop.read_ty {
            let res = self.is_inhabited_type_id_set_type_id(ty, seen);
            if res != NormalizationResult::True {
              return res;
            }
          }
        } else {
          let res = self.is_inhabited_type_id_set_type_id(prop.read_ty.unwrap(), seen);
          if res != NormalizationResult::True {
            return res;
          }
        }
      }
      return NormalizationResult::True;
    }

    if let Some(mtv) = get_type_id::<MetatableType>(ty) {
      let res = self.is_inhabited_type_id_set_type_id(mtv.table, seen);
      if res != NormalizationResult::True {
        return res;
      }
      return self.is_inhabited_type_id_set_type_id(mtv.metatable, seen);
    }

    let norm = self.normalize(ty);
    self.is_inhabited_normalized_type_set_type_id(norm.as_ref(), seen)
  }
}
