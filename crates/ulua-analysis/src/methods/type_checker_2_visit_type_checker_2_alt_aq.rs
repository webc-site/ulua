use ulua_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

use crate::{
  enums::{normalization_result::NormalizationResult, value::Value, value_context::ValueContext},
  functions::should_suppress_errors_type_utils::should_suppress_errors,
  records::{
    error_suppression::ErrorSuppression, normalization_too_complex::NormalizationTooComplex,
    type_checker_2::TypeChecker2, types_are_unrelated::TypesAreUnrelated,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_type_assertion(&mut self, expr: *mut AstExprTypeAssertion) {
    unsafe {
      self.visit_ast_expr_value_context((*expr).expr, ValueContext::RValue);
      self.visit_ast_type((*expr).annotation);

      // SAFETY: annotation/expr 指向 AST arena 节点。
      let annotation_type: TypeId = self.lookup_annotation(&*(*expr).annotation);
      let computed_type: TypeId = self.lookup_type(&*(*expr).expr);

      let suppression: ErrorSuppression =
        should_suppress_errors(&mut self.normalizer as *mut _, computed_type).or_else(
          &should_suppress_errors(&mut self.normalizer as *mut _, annotation_type),
        );

      match suppression.error_suppression_value() {
        Value::Suppress => return,
        Value::NormalizationFailed => {
          self.report_error_type_error_data_location(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &(*expr).base.base.location,
          );
          return;
        }
        Value::DoNotSuppress => {}
      }

      match self.normalizer.is_inhabited_type_id(computed_type) {
        NormalizationResult::True => {}
        NormalizationResult::False => return,
        NormalizationResult::HitLimits => {
          self.report_error_type_error_data_location(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &(*expr).base.base.location,
          );
          return;
        }
      }

      match self
        .normalizer
        .is_intersection_inhabited_type_id_type_id(computed_type, annotation_type)
      {
        NormalizationResult::True => (),
        NormalizationResult::False => {
          self.report_error_type_error_data_location(
            TypeErrorData::TypesAreUnrelated(TypesAreUnrelated {
              left: computed_type,
              right: annotation_type,
            }),
            &(*expr).base.base.location,
          );
        }
        NormalizationResult::HitLimits => {
          self.report_error_type_error_data_location(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &(*expr).base.base.location,
          );
        }
      }
    }
  }
}
