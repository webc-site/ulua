use alloc::string::String;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    negation_type::NegationType, normalization_too_complex::NormalizationTooComplex,
    type_mismatch::TypeMismatch, unifier::Unifier,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  pub fn unifier_try_unify_negations(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    if get_type_id::<NegationType>(sub_ty).is_none()
      && get_type_id::<NegationType>(super_ty).is_none()
    {
      self.ice_string("tryUnifyNegations super_ty or sub_ty must be a negation type");
    }

    // 归一化过于复杂时与 C++ 一致报错返回
    let (sub_norm, super_norm) = (
      unsafe { (*self.normalizer).try_normalize(sub_ty) },
      unsafe { (*self.normalizer).try_normalize(super_ty) },
    );
    let (Some(sub_norm), Some(super_norm)) = (sub_norm, super_norm) else {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
      );
      return;
    };

    let mut state = self.unifier_make_child_unifier();
    state.unifier_try_unify_normalized_types(
      sub_ty,
      super_ty,
      &sub_norm,
      &super_norm,
      String::new(),
      None,
    );
    if state.errors.is_empty() {
      let context = self.unifier_mismatch_context();
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: String::new(),
          error: None,
          context,
        }),
      );
    }
  }
}
