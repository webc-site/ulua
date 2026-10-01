use crate::{
  functions::{follow_type, get_type},
  records::{
    generic_type::GenericType, type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};
impl TypeFunctionReductionGuesser {
  pub fn infer_type_function_substitutions(
    &mut self,
    ty: TypeId,
    instance: &TypeFunctionInstanceType,
  ) {
    let result = if self.is_numeric_binop_function(instance) {
      self.infer_numeric_binop_function(instance)
    } else if self.is_comparison_function(instance) {
      self.infer_comparison_function(instance)
    } else if self.is_or_and_function(instance) {
      self.infer_or_and_function(instance)
    } else if self.is_not_function(instance) {
      self.infer_not_function(instance)
    } else if self.is_len_function(instance) {
      self.infer_len_function(instance)
    } else if self.is_unary_minus(instance) {
      self.infer_unary_minus_function(instance)
    } else {
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![],
        function_result_inference: self.builtins.get_mut().unknown_type,
      }
    };

    let result_inference = follow_type::follow(result.function_result_inference);

    if !self.function_reduces_to.contains(&result_inference) {
      *self.function_reduces_to.get_or_insert(ty) = result_inference;
    }

    // zip 在较短一侧停摆，等价于上游的 i < operand_inference.len() 逐个 continue
    for (&arg_raw, &inf) in instance
      .type_arguments
      .iter()
      .zip(result.operand_inference.iter())
    {
      let arg = follow_type::follow(arg_raw);
      let inference = follow_type::follow(inf);

      if get_type::get::<TypeFunctionInstanceType>(arg).is_some() {
        if !self.function_reduces_to.contains(&arg) {
          *self.function_reduces_to.get_or_insert(arg) = inference;
        }
      } else if get_type::get::<GenericType>(arg).is_some() {
        *self.substitutable.get_or_insert(arg) = inference;
      }
    }
  }
}
