use crate::records::{normalized_function_type::NormalizedFunctionType, normalizer::Normalizer};

impl Normalizer {
  pub fn intersect_functions(
    &mut self,
    heres: &mut NormalizedFunctionType,
    theres: &NormalizedFunctionType,
  ) {
    self.consume_fuel();

    if heres.is_never() {
    } else if theres.is_never() {
      heres.reset_to_never();
    } else {
      for there in theres.parts.order.iter() {
        let there = *there;
        self.intersect_functions_with_function(heres, there);
      }
    }
  }
}
