use alloc::{string::String, sync::Arc};

use crate::{
  functions::has_unification_too_complex::has_unification_too_complex,
  records::{
    intersection_type::IntersectionType, type_error::TypeError, type_mismatch::TypeMismatch,
    unifier::Unifier,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn unifier_try_unify_type_with_intersection(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    uv: *const IntersectionType,
  ) {
    let uv = unsafe { &*uv };
    let mut unification_too_complex: Option<TypeError> = None;
    let mut first_failed_option: Option<TypeError> = None;

    for ty in uv.parts.iter().copied() {
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state
        .try_unify_type_id_type_id_bool_bool_literal_properties(sub_ty, ty, false, true, None);

      if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        unification_too_complex = Some(e);
      } else if !inner_state.errors.is_empty() && first_failed_option.is_none() {
        first_failed_option = inner_state.errors.first().cloned();
      }

      self.log.concat(inner_state.log);
      self.failure |= inner_state.failure;
    }

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if let Some(first) = first_failed_option {
      let context = self.unifier_mismatch_context();
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: String::from("Not all intersection parts are compatible."),
          error: Some(Arc::new(first)),
          context,
        }),
      );
    }
  }
}
