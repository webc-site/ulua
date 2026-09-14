use ulua_ast::records::location::Location;

use crate::{
  enums::value::Value,
  functions::should_suppress_errors_type_utils::should_suppress_errors,
  records::{normalization_too_complex::NormalizationTooComplex, type_checker_2::TypeChecker2},
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_error_suppressing_location_type_id(
    &mut self,
    loc: Location,
    ty: TypeId,
  ) -> bool {
    match unsafe { should_suppress_errors(&mut self.normalizer as *mut _, ty) }
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
