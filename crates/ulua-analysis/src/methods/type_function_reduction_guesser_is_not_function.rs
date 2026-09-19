use crate::records::{
  type_function_instance_type::TypeFunctionInstanceType,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};

impl TypeFunctionReductionGuesser {
  pub fn is_not_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    let function = unsafe { instance.function.as_ref() };
    function.name == "not"
  }
}
