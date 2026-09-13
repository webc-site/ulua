use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;

impl TypeFunctionReductionGuesser {
  pub fn infer(&mut self) {
    while !self.done() {
      self.step();
    }
  }
}
