use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type,
  records::{
    type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn infer_comparison_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    // Safety: `instance` 由 step() 在 follow 后经 get_type::get::<TypeFunctionInstanceType>
    // 命中才传入（调用入口 LUAU_ASSERT 非空），指向 arena 中存活的实例节点载荷，
    // 猜测会话期间不移动；读 type_arguments.len() 有效。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });
    // Comparison functions are lt/le/eq.
    // Heuristic: these are type functions from t -> t -> bool

    // Safety: 同上——instance 指向存活节点且 len == 2 刚被断言，[0] 索引不越界；
    // 元素为 arena 存活 TypeId 句柄，follow_type_id 只读节点变体。
    let mut lhs_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[0]) };
    // Safety: 与 [0] 侧逐字同前提——存活实例节点、len == 2 断言、[1] 在界内。
    let mut rhs_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[1]) };

    // Safety: self.builtins 在 guesser 构造时由 TypeFunctionReducer 会话接线，
    // 非空、比 guesser 长寿、reduce 期间不再写入；提为一次共享借用，仅读 Copy 句柄。
    let builtins = self.builtins.get();
    let boolean_ty = builtins.boolean_type;
    let comparison_inference = |op: TypeId| -> TypeFunctionInferenceResult {
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![op, op],
        function_result_inference: boolean_ty,
      }
    };

    if let Some(ty) = self.try_assign_operand_type(lhs_ty) {
      lhs_ty = follow_type::follow(ty);
    }
    if let Some(ty) = self.try_assign_operand_type(rhs_ty) {
      rhs_ty = follow_type::follow(ty);
    }
    if self.operand_is_assignable(lhs_ty) && !self.operand_is_assignable(rhs_ty) {
      return comparison_inference(rhs_ty);
    }
    if self.operand_is_assignable(rhs_ty) && !self.operand_is_assignable(lhs_ty) {
      return comparison_inference(lhs_ty);
    }
    comparison_inference(builtins.number_type)
  }
}
