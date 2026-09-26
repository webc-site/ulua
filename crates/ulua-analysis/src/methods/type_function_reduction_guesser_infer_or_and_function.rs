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
  pub unsafe fn infer_or_and_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    // Safety: `instance` 由 step() 在 follow 后经 get_type::get::<TypeFunctionInstanceType>
    // 命中才传入（调用入口 LUAU_ASSERT 非空），指向 arena 中存活的实例节点载荷；
    // 猜测会话期间该节点不被移动，读 type_arguments.len() 有效。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });

    // Safety: 同上——instance 指向存活节点；len == 2 刚由上行断言保证，[0] 索引
    // 不越界，元素是 arena 存活 TypeId 句柄（follow 只读节点变体）。
    let mut lhs_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[0]) };
    // Safety: 与 [0] 侧逐字同前提——存活实例节点、len == 2 断言、[1] 在界内。
    let mut rhs_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[1]) };

    if let Some(ty) = self.try_assign_operand_type(lhs_ty) {
      lhs_ty = follow_type::follow(ty);
    }
    if let Some(ty) = self.try_assign_operand_type(rhs_ty) {
      rhs_ty = follow_type::follow(ty);
    }

    // Safety: self.builtins 在 guesser 构造时由 TypeFunctionReducer 会话接线，
    // 非空、比 guesser 长寿、reduce 期间不再写入；提为一次共享借用仅读 Copy 句柄。
    let builtins = self.builtins.get();
    let unknown_ty = builtins.unknown_type;
    let boolean_ty = builtins.boolean_type;
    let default_and_or_inference = TypeFunctionInferenceResult {
      operand_inference: alloc::vec![unknown_ty, unknown_ty],
      function_result_inference: boolean_ty,
    };

    // C++ 上游即对 lhsTy 归一化两次（忠实保留）；归一化失败视为不真
    let lty = self.normalize(lhs_ty);
    let rty = self.normalize(lhs_ty);
    let lhs_truthy = lty.as_ref().map(|n| n.is_truthy()).unwrap_or(false);
    let rhs_truthy = rty.as_ref().map(|n| n.is_truthy()).unwrap_or(false);

    // If at the end, we still don't have good substitutions, return the default type
    // Safety: instance 存活前提同上（step 命中后传入、会话内节点不移动）；
    // function 字段是实例构造时写入的 NonNull<TypeFunction>，指向类型函数注册表
    // 条目——注册表比 arena 与本次猜测会话长寿且其 name 不再改写，解引用读 &str 有效。
    let function_name = unsafe { (*(*instance).function.as_ptr()).name.as_str() };

    if function_name == "or" {
      if self.operand_is_assignable(lhs_ty) && self.operand_is_assignable(rhs_ty) {
        return default_and_or_inference;
      }
      if self.operand_is_assignable(lhs_ty) {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![unknown_ty, rhs_ty],
          function_result_inference: rhs_ty,
        };
      }
      if self.operand_is_assignable(rhs_ty) {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![lhs_ty, unknown_ty],
          function_result_inference: lhs_ty,
        };
      }
      if lhs_truthy {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![lhs_ty, rhs_ty],
          function_result_inference: lhs_ty,
        };
      }
      if rhs_truthy {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![unknown_ty, rhs_ty],
          function_result_inference: rhs_ty,
        };
      }
    }

    if function_name == "and" {
      // (mirrors C++ `instance->function->name == "and"`)
      if self.operand_is_assignable(lhs_ty) && self.operand_is_assignable(rhs_ty) {
        return default_and_or_inference;
      }
      if self.operand_is_assignable(lhs_ty) {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![],
          function_result_inference: rhs_ty,
        };
      }
      if self.operand_is_assignable(rhs_ty) {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![],
          function_result_inference: lhs_ty,
        };
      }
      if lhs_truthy {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![lhs_ty, rhs_ty],
          function_result_inference: rhs_ty,
        };
      } else {
        return TypeFunctionInferenceResult {
          operand_inference: alloc::vec![lhs_ty, rhs_ty],
          function_result_inference: lhs_ty,
        };
      }
    }

    default_and_or_inference
  }
}
