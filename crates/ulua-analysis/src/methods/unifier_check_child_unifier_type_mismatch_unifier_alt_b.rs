use alloc::{string::String, sync::Arc};

use crate::{
  functions::has_unification_too_complex::has_unification_too_complex,
  records::{type_mismatch::TypeMismatch, unifier::Unifier},
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  pub fn check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
    &mut self,
    inner_errors: &ErrorVec,
    prop: &str,
    wanted_type: TypeId,
    given_type: TypeId,
  ) {
    if let Some(e) = has_unification_too_complex(inner_errors) {
      self.report_error_type_error(e);
    } else if !inner_errors.is_empty() {
      let reason: String = format!("Property '{}' is not compatible.", prop);
      let context = self.unifier_mismatch_context();

      let mismatch = TypeMismatch {
        wanted_type,
        given_type,
        reason,
        error: Some(Arc::new(inner_errors[0].clone())),
        context,
      };

      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(mismatch),
      );
    }
  }
}
