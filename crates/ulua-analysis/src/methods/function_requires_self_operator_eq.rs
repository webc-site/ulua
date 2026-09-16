use crate::records::function_requires_self::FunctionRequiresSelf;

impl FunctionRequiresSelf {
  #[inline]
  pub fn operator_eq(&self, _other: &FunctionRequiresSelf) -> bool {
    true
  }
}
