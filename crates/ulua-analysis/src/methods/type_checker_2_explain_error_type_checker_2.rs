use ulua_ast::records::location::Location;

use crate::{
  enums::{context_error::Context, value::Value},
  functions::should_suppress_errors_type_utils::should_suppress_errors,
  records::{
    normalization_too_complex::NormalizationTooComplex, subtyping_result::SubtypingResult,
    type_checker_2::TypeChecker2, type_error::TypeError, type_mismatch::TypeMismatch,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn explain_error_type_id_type_id_location_subtyping_result(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    location: Location,
    result: &SubtypingResult,
  ) {
    if result.is_error_suppressing {
      return;
    }

    let suppression = unsafe { should_suppress_errors(&mut self.normalizer, sub_ty) }
      .or_else(&unsafe { should_suppress_errors(&mut self.normalizer, super_ty) });

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

    let reasonings = self.explain_reasonings_type_id_type_id_location_subtyping_result(
      sub_ty, super_ty, location, result,
    );

    if !reasonings.suppressed {
      self.report_error_type_error(TypeError::type_error_location_type_error_data(
        location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: reasonings.to_string(),
          error: None,
          context: Context::COVARIANT,
        }),
      ));
    }
  }
}
