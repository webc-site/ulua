use crate::records::unapplied_type_function::UnappliedTypeFunction;

impl UnappliedTypeFunction {
  #[inline]
  pub fn operator_eq(&self, _rhs: &UnappliedTypeFunction) -> bool {
    true
  }
}
