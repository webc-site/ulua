use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    generic_type::GenericType, type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};
impl TypeFunctionReductionGuesser {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn infer_type_function_substitutions(
    &mut self,
    ty: TypeId,
    instance: *const TypeFunctionInstanceType,
  ) {
    LUAU_ASSERT!(!instance.is_null());

    let result: TypeFunctionInferenceResult;

    unsafe {
      let instance_ref = &*instance;

      if self.is_numeric_binop_function(instance_ref) {
        result = self.infer_numeric_binop_function(instance);
      } else if self.is_comparison_function(instance_ref) {
        result = self.infer_comparison_function(instance);
      } else if self.is_or_and_function(instance_ref) {
        result = self.infer_or_and_function(instance);
      } else if self.is_not_function(instance_ref) {
        result = self.infer_not_function(instance);
      } else if self.is_len_function(instance_ref) {
        result = self.infer_len_function(instance);
      } else if self.is_unary_minus(instance_ref) {
        result = self.infer_unary_minus_function(instance);
      } else {
        result = TypeFunctionInferenceResult {
          operand_inference: alloc::vec![],
          function_result_inference: (*self.builtins).unknown_type,
        };
      }
    }

    let result_inference = follow_type_id(result.function_result_inference);

    if !self.function_reduces_to.contains(&result_inference) {
      *self.function_reduces_to.get_or_insert(ty) = result_inference;
    }

    unsafe {
      for i in 0..(*instance).type_arguments.len() {
        if i < result.operand_inference.len() {
          let arg = follow_type_id((&(*instance).type_arguments)[i]);
          let inference = follow_type_id(result.operand_inference[i]);

          if !get_type_id::<TypeFunctionInstanceType>(arg).is_none() {
            if !self.function_reduces_to.contains(&arg) {
              *self.function_reduces_to.get_or_insert(arg) = inference;
            }
          } else if !get_type_id::<GenericType>(arg).is_none() {
            *self.substitutable.get_or_insert(arg) = inference;
          }
        }
      }
    }
  }
}
