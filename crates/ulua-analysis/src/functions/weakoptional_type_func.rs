use crate::{
  enums::{normalization_result::NormalizationResult, reduction::Reduction},
  functions::{follow_type, get_type, is_pending::is_pending},
  macros::check_arity,
  records::{
    never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn weakoptional_type_func(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 1, "weakoptional");

  let target_ty = follow_type::follow(type_params[0]);

  // `is_pending` 为安全函数：solver 可空由内部判空折叠，target_ty 为存活 arena 句柄。
  if is_pending(target_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(alloc::vec![target_ty]);
  }

  if get_type::get::<NeverType>(instance).is_some() {
    return TypeFunctionReductionResult {
      result: Some(ctx.builtins().nil_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![],
      blocked_packs: alloc::vec![],
      error: None,
      messages: alloc::vec![],
    };
  }

  // C++ 中归一化失败返回空指针：不归约，对驻留性一无所知
  let Some(target_norm) = ctx.normalizer_mut().try_normalize(target_ty) else {
    return TypeFunctionReductionResult::no_reduction(alloc::vec![]);
  };

  let result = ctx
    .normalizer_mut()
    .is_inhabited_normalized_type(target_norm.as_ref());
  if result == NormalizationResult::False {
    return TypeFunctionReductionResult {
      result: Some(ctx.builtins().nil_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![],
      blocked_packs: alloc::vec![],
      error: None,
      messages: alloc::vec![],
    };
  }

  TypeFunctionReductionResult::reduction(target_ty)
}
