use crate::records::function_exits_without_returning::FunctionExitsWithoutReturning;

impl FunctionExitsWithoutReturning {
  #[inline]
  pub fn operator_eq(&self, rhs: &FunctionExitsWithoutReturning) -> bool {
    self.expected_return_type == rhs.expected_return_type
  }
}
