use crate::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;

impl ExplicitFunctionAnnotationRecommended {
  #[inline]
  pub fn operator_eq(&self, rhs: &ExplicitFunctionAnnotationRecommended) -> bool {
    self.recommended_return == rhs.recommended_return
      && self.recommended_args == rhs.recommended_args
  }
}
