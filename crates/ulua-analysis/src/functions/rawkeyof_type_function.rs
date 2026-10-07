use crate::{
  functions::keyof_function_impl::keyof_function_impl,
  macros::check_arity,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub(crate) fn rawkeyof_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(type_params, pack_params, 1, assert_only);

  keyof_function_impl(type_params, pack_params, ctx, true)
}
