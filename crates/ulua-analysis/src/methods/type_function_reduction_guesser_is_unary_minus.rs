use crate::records::{
  type_function_instance_type::TypeFunctionInstanceType,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};

impl TypeFunctionReductionGuesser {
  pub fn is_unary_minus(&self, instance: &TypeFunctionInstanceType) -> bool {
    let func = unsafe { instance.function.as_ref() };
    func.name == "unm"
  }
}
