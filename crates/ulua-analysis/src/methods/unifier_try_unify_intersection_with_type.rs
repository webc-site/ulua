use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use crate::{
  functions::has_unification_too_complex::has_unification_too_complex,
  records::{
    intersection_type::IntersectionType, txn_log::TxnLog, type_error::TypeError,
    type_mismatch::TypeMismatch, unifier::Unifier,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn unifier_try_unify_intersection_with_type(
    &mut self,
    sub_ty: TypeId,
    uv: *const IntersectionType,
    super_ty: TypeId,
    cache_enabled: bool,
    is_function_call: bool,
  ) {
    let uv = unsafe { &*uv };
    let mut found = false;
    let mut errors_suppressed = false;
    let mut unification_too_complex: Option<TypeError> = None;
    let mut start_index = 0usize;

    if cache_enabled {
      for (i, ty) in uv.parts.iter().enumerate() {
        if unsafe {
          (*self.shared_state)
            .cached_unify
            .find(&(*ty, super_ty))
            .is_some()
        } {
          start_index = i;
          break;
        }
      }
    }

    let mut logs: Vec<TxnLog> = Vec::new();

    for i in 0..uv.parts.len() {
      let ty = uv.parts[(i + start_index) % uv.parts.len()];
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state.normalize = false;
      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        ty,
        super_ty,
        is_function_call,
        false,
        None,
      );

      if inner_state.errors.is_empty() {
        found = true;
        errors_suppressed = inner_state.failure;
        if inner_state.failure {
          logs.push(inner_state.log);
        } else {
          errors_suppressed = false;
          self.log.concat(inner_state.log);
          break;
        }
      } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        unification_too_complex = Some(e);
      }
    }

    if errors_suppressed && !logs.is_empty() {
      self.log.concat(logs.remove(0));
    }

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if !found && self.normalize {
      let sub_norm = unsafe { (*self.normalizer).normalize(sub_ty) };
      let super_norm = unsafe { (*self.normalizer).normalize(super_ty) };
      self.unifier_try_unify_normalized_types(
        sub_ty,
        super_ty,
        &sub_norm,
        &super_norm,
        "none of the intersection parts are compatible".to_string(),
        None,
      );
    } else if !found {
      let context = self.unifier_mismatch_context();
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: String::from("none of the intersection parts are compatible"),
          error: None,
          context,
        }),
      );
    } else if errors_suppressed {
      self.failure = true;
    }
  }
}
