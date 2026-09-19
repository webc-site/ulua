use ulua_ast::records::location::Location;

use crate::{
  enums::value::Value,
  functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id,
  records::{
    normalization_too_complex::NormalizationTooComplex, subtyping_result::SubtypingResult,
    type_checker_2::TypeChecker2, type_error::TypeError, type_pack_mismatch::TypePackMismatch,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_id::TypePackId},
};
impl TypeChecker2 {
  pub fn explain_error_type_pack_id_type_pack_id_location_subtyping_result(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    location: Location,
    result: &SubtypingResult,
  ) {
    if result.is_error_suppressing {
      return;
    }

    let suppression =
      should_suppress_errors_not_null_normalizer_type_pack_id(&mut self.normalizer, sub_tp)
        .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
          &mut self.normalizer,
          super_tp,
        ));

    match suppression.error_suppression_value() {
      Value::Suppress => return,
      Value::NormalizationFailed => {
        self.report_error_type_error(TypeError::type_error_location_type_error_data(
          location,
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None }),
        ));
      }
      _ => {}
    }

    let reasonings = self.explain_reasonings_type_pack_id_type_pack_id_location_subtyping_result(
      sub_tp, super_tp, location, result,
    );

    if !reasonings.suppressed {
      self.report_error_type_error(TypeError::type_error_location_type_error_data(
        location,
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp: super_tp,
          given_tp: sub_tp,
          reason: reasonings.to_string(),
        }),
      ));
    }
  }
}
