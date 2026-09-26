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
    // Safety: `instance` 由派发链 `infer_type_function_substitutions` 在
    // `get_type::get::<TypeFunctionInstanceType>` 命中并经非空守卫后传入，指向类型
    // arena bump 分块中存活节点；此处只读其 `type_arguments` Vec 的长度做前置断言。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });

    let builtins = self.builtins.get();
    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![builtins.number_type, builtins.number_type],
      function_result_inference: builtins.number_type,
    }
  }
}
