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
  pub fn infer_or_and_function(
    &mut self,
    instance: &TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(instance.type_arguments.len() == 2);

    let mut lhs_ty = follow_type::follow(instance.type_arguments[0]);
    let mut rhs_ty = follow_type::follow(instance.type_arguments[1]);

    if let Some(ty) = self.try_assign_operand_type(lhs_ty) {
      lhs_ty = follow_type::follow(ty);
    }
    if let Some(ty) = self.try_assign_operand_type(rhs_ty) {
      rhs_ty = follow_type::follow(ty);
    }

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
    let lhs_truthy = lty.as_ref().is_some_and(|n| n.is_truthy());
    let rhs_truthy = rty.as_ref().is_some_and(|n| n.is_truthy());

    // If at the end, we still don't have good substitutions, return the default type
    let function_name = instance.function().name.as_str();

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
