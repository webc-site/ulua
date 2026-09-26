use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type,
  records::{
    type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
};

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn infer_not_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    // Safety: `instance` 按函数级契约非 null，指向归约会话所用 type arena 中存活且对齐的
    // `TypeFunctionInstanceType` 节点；这里只读其 `type_arguments` 切片的长度。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 1 });

    // Safety: 同一 `instance` 存活前提成立，`(*instance).type_arguments` 重建共享引用有效；
    // 索引 `[0]` 为切片的边界检查访问——契约保证该实例恰有 1 个实参（上方 `LUAU_ASSERT!`），
    // 越界只会 panic 而非 UB，故非 null 前提仅由 `instance` 解引用这一处 unsafe 承担。
    let op_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[0]) };
    let op_ty = if let Some(ty) = self.try_assign_operand_type(op_ty) {
      follow_type::follow(ty)
    } else {
      op_ty
    };

    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![op_ty],
      // Safety: `self.builtins` 是 guesser 构造期接线的 `*mut BuiltinTypes`，指向比本 guesser
      // 长寿的内置类型单例、全程存活非空；此处只读取 Copy 的 `boolean_type`（`TypeId`），
      // 单线程归约内无并存可变借用。
      function_result_inference: self.builtins.get_mut().boolean_type,
    }
  }
}
