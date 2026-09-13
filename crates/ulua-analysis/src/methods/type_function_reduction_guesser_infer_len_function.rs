use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow_type_id,
  records::{
    type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
};

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn infer_len_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    unsafe {
      LUAU_ASSERT!((*instance).type_arguments.len() == 1);
      let mut op_ty = follow_type_id((&(*instance).type_arguments)[0]);
      if let Some(ty) = self.try_assign_operand_type(op_ty) {
        op_ty = follow_type_id(ty);
      }
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![op_ty],
        function_result_inference: (*self.builtins).number_type,
      }
    }
  }
}
