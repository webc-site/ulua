use alloc::string::String;

use crate::{
  functions::has_unification_too_complex::has_unification_too_complex,
  records::{intersection_type::IntersectionType, type_error::TypeError, unifier::Unifier},
  type_aliases::type_id::TypeId,
};

impl Unifier {
  /// Rust 形态（§2）：C++ `const IntersectionType* uv` → `&'static IntersectionType`，
  /// 判空与解引用收口在调用方的 `alias_opt` 门面，本方法无裸指针。
  pub fn unifier_try_unify_type_with_intersection(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    uv: &'static IntersectionType,
  ) {
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
      self.unifier_report_type_mismatch_ext(
        super_ty,
        sub_ty,
        String::from("Not all intersection parts are compatible."),
        Some(first),
      );
    }
  }
}
