use ulua_ast::records::location::Location;

use crate::{
  enums::value::Value,
  functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id,
  records::{
    normalization_too_complex::NormalizationTooComplex, overload_resolver::OverloadResolver,
    subtyping_reasoning::SubtypingReasoning, type_error::TypeError,
    type_pack_mismatch::TypePackMismatch,
  },
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, type_error_data::TypeErrorData,
    type_pack_id::TypePackId,
  },
};
impl OverloadResolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
    &self,
    errors: *mut ErrorVec,
    arg_location: Location,
    _module_name: &ModuleName,
    _reason: *const SubtypingReasoning,
    wanted_tp: Option<TypePackId>,
    given_tp: Option<TypePackId>,
  ) {
    if wanted_tp.is_none() || given_tp.is_none() {
      return;
    }

    let wanted_tp = wanted_tp.unwrap();
    let given_tp = given_tp.unwrap();

    let suppression =
      should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer, wanted_tp).or_else(
        &should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer, given_tp),
      );

    match suppression.error_suppression_value() {
      Value::Suppress => {}
      Value::NormalizationFailed => unsafe {
        (*errors).push(TypeError::type_error_location_type_error_data(
          arg_location,
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None }),
        ));
      },
      _ => unsafe {
        (*errors).push(TypeError::type_error_location_type_error_data(
          arg_location,
          TypeErrorData::TypePackMismatch(TypePackMismatch {
            wanted_tp,
            given_tp,
            reason: String::new(),
          }),
        ));
      },
    }
  }
}
