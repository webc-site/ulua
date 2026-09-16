//! Source: `Analysis/src/Unifier.cpp` (Unifier::occursCheck(TypeId,...), L2606-2642)
use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    intersection_type::IntersectionType, occurs_check_failed::OccursCheckFailed, r#type::Type,
    unifier::Unifier, union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_variant::TypeVariant},
};

impl Unifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `bool Unifier::occursCheck(TypeId needle, TypeId haystack, bool reversed)`
  pub(crate) fn occurs_check_type_id_type_id_bool(
    &mut self,
    needle: TypeId,
    haystack: TypeId,
    reversed: bool,
  ) -> bool {
    let shared_state = unsafe { &mut *self.shared_state };
    shared_state.temp_seen_ty.clear();

    let occurs = self.occurs_check_dense_hash_set_type_id_type_id_type_id(
      &mut shared_state.temp_seen_ty,
      needle,
      haystack,
    );

    if occurs {
      let mut inner_state = self.unifier_make_child_unifier();
      if let Some(ut) = get_type_id::<UnionType>(haystack) {
        if reversed {
          unsafe { inner_state.unifier_try_unify_union_with_type(haystack, ut, needle) };
        } else {
          unsafe {
            inner_state.unifier_try_unify_type_with_union(needle, haystack, ut, false, false)
          };
        }
      } else if let Some(it) = get_type_id::<IntersectionType>(haystack) {
        if reversed {
          unsafe {
            inner_state.unifier_try_unify_intersection_with_type(haystack, it, needle, false, false)
          };
        } else {
          unsafe { inner_state.unifier_try_unify_type_with_intersection(needle, haystack, it) };
        }
      } else {
        inner_state.failure = true;
      }

      if inner_state.failure {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
        );
        // C++: log.replace(needle, BoundType{builtinTypes->error_type});
        let error_ty = unsafe { (*self.builtin_types).error_type };
        self
          .log
          .replace_type_id_t(needle, Type::new(TypeVariant::Bound(error_ty)));
      }
    }

    occurs
  }
}
