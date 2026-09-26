use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;

impl TypeFunctionReductionGuesser {
  pub fn done(&self) -> bool {
    self.to_infer.empty()
  }
}
