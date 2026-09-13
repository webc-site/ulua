use crate::records::function_does_not_take_self::FunctionDoesNotTakeSelf;

impl FunctionDoesNotTakeSelf {
  #[inline]
  pub fn operator_eq(&self, _other: &FunctionDoesNotTakeSelf) -> bool {
    true
  }
}
