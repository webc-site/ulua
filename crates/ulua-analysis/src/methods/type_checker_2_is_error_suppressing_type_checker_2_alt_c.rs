use ulua_ast::records::location::Location;

use crate::{
  enums::value::Value,
  functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id,
  records::{normalization_too_complex::NormalizationTooComplex, type_checker_2::TypeChecker2},
  type_aliases::{type_error_data::TypeErrorData, type_pack_id::TypePackId},
};
impl TypeChecker2 {
  pub fn is_error_suppressing_location_type_pack_id(
    &mut self,
    loc: Location,
    tp: TypePackId,
  ) -> bool {
    match should_suppress_errors_not_null_normalizer_type_pack_id(
      &mut self.normalizer as *mut _,
      tp,
    )
    .error_suppression_value()
    {
      Value::DoNotSuppress => false,
      Value::Suppress => true,
      Value::NormalizationFailed => {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &loc,
        );
        false
      }
    }
  }
}
