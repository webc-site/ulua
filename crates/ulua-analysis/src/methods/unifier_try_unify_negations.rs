use alloc::string::String;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{negation_type::NegationType, type_mismatch::TypeMismatch, unifier::Unifier},
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  pub fn unifier_try_unify_negations(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    if get_type_id::<NegationType>(sub_ty).is_none()
      && get_type_id::<NegationType>(super_ty).is_none()
    {
      self.ice_string("tryUnifyNegations super_ty or sub_ty must be a negation type");
    }

    let sub_norm = unsafe { (*self.normalizer).normalize(sub_ty) };
    let super_norm = unsafe { (*self.normalizer).normalize(super_ty) };

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
