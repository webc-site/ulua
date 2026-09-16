use crate::records::{
  type_function_instance_type::TypeFunctionInstanceType,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};

impl TypeFunctionReductionGuesser {
  pub fn is_numeric_binop_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    let func = unsafe { &*instance.function.as_ptr() };
    func.name == "add"
      || func.name == "sub"
      || func.name == "mul"
      || func.name == "div"
      || func.name == "idiv"
      || func.name == "pow"
      || func.name == "mod"
  }
}
