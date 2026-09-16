use crate::records::instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction;

impl InstantiateGenericsOnNonFunction {
  #[inline]
  pub fn operator_eq(&self, rhs: &InstantiateGenericsOnNonFunction) -> bool {
    self.interesting_edge_case == rhs.interesting_edge_case
  }
}
