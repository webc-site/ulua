use crate::records::{type_error::TypeError, type_error_summary::TypeErrorSummary};
impl TypeError {
  pub fn summary(&self) -> TypeErrorSummary {
    TypeErrorSummary::type_error_summary_type_error_summary(
      self.location,
      self.module_name.clone(),
      self.code(),
    )
  }
}
