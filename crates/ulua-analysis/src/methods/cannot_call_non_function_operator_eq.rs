use crate::records::cannot_call_non_function::CannotCallNonFunction;

impl CannotCallNonFunction {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotCallNonFunction) -> bool {
    self.ty == rhs.ty
  }
}
