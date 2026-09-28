use ulua_common::macros::luau_assert::LUAU_ASSERT;

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
  /// # Safety
  /// `instance` 须非空、对齐，指向 guesser 会话期间存活的 arena
  /// `TypeFunctionInstanceType`（唯一调用方 `step()` 经 `get_type::get` 命中后
  /// 传入，同 cpp `TypeFunctionReductionGuesser.cpp:283` 的 NotNull 前提）；
  /// 本函数只读该节点，结果写回 guesser 自身的 `function_reduces_to`/
  /// `substitutable` 表。单线程。
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
          function_result_inference: self.builtins.get_mut().unknown_type,
        };
      }
    }

    let result_inference = follow_type::follow(result.function_result_inference);

    if !self.function_reduces_to.contains(&result_inference) {
      *self.function_reduces_to.get_or_insert(ty) = result_inference;
    }

    unsafe {
      // zip 在较短一侧停摆，等价于上游的 i < operand_inference.len() 逐个 continue
      for (&arg_raw, &inf) in (*instance)
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
}
