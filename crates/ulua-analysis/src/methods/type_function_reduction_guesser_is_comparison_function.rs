use crate::records::{
  type_function_instance_type::TypeFunctionInstanceType,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};

impl TypeFunctionReductionGuesser {
  pub fn is_comparison_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    let func = unsafe { &*instance.function.as_ptr() };
    func.name == "lt" || func.name == "le" || func.name == "eq"
  }
}
