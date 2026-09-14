use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  type_function_inference_result::TypeFunctionInferenceResult,
  type_function_instance_type::TypeFunctionInstanceType,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn infer_numeric_binop_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });

    unsafe {
      let builtins = self.builtins;
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![(*builtins).number_type, (*builtins).number_type,],
        function_result_inference: (*builtins).number_type,
      }
    }
  }
}
