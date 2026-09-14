use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_common::DFInt;

use crate::{
  enums::reduction::Reduction,
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn try_distribute_type_function_app<F>(
  mut f: F,
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: *mut TypeFunctionContext,
) -> Option<TypeFunctionReductionResult>
where
  F: FnMut(
    TypeId,
    Vec<TypeId>,
    Vec<TypePackId>,
    *mut TypeFunctionContext,
  ) -> TypeFunctionReductionResult,
{
  // SAFETY: 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
  // NonNull 封装解引用。
  let ctx_ref = unsafe { NonNull::new_unchecked(ctx).as_mut() };

  let mut reduction_status = Reduction::MaybeOk;
  let mut blocked_types = Vec::new();
  let mut results = Vec::new();
  let mut cartesian_product_size = 1usize;

  let mut first_union_options: Option<Vec<TypeId>> = None;
  let mut union_index = 0usize;
  let mut arguments = type_params.to_vec();

  for (index, argument) in arguments.iter().copied().enumerate() {
    let ty = follow_type_id(argument);
    let Some(union) = get_type_id::<UnionType>(ty) else {
      continue;
    };

    if first_union_options.is_none() {
      first_union_options = Some(union.options.clone());
      union_index = index;
    }

    cartesian_product_size = cartesian_product_size.saturating_mul(union.options.len());
    if (DFInt::LuauTypeFamilyApplicationCartesianProductLimit.get() as usize)
      <= cartesian_product_size
    {
      return Some(TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      });
    }
  }

  let first_union_options = first_union_options?;

  for option in first_union_options {
    arguments[union_index] = option;

    let result = f(instance, arguments.clone(), pack_params.to_vec(), ctx);
    blocked_types.extend(result.blocked_types.iter().copied());

    if result.reduction_status != Reduction::MaybeOk {
      reduction_status = result.reduction_status;
    }

    if reduction_status != Reduction::MaybeOk || result.result.is_none() {
      break;
    }

    results.push(result.result.unwrap());
  }

  if reduction_status != Reduction::MaybeOk || !blocked_types.is_empty() {
    return Some(TypeFunctionReductionResult {
      result: None,
      reduction_status,
      blocked_types,
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    });
  }

  if !results.is_empty() {
    if results.len() == 1 {
      return Some(TypeFunctionReductionResult {
        result: Some(results[0]),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      });
    }

    // SAFETY: reducer 契约保证 ctx 非空且会话期有效；arena/builtins 均由 ctx 持有。
    let result_ty = unsafe {
      let union_func = &ctx_ref.builtins.as_ref().type_functions.union_func;
      let ty = ctx_ref
        .arena
        .as_mut()
        .add_type(TypeFunctionInstanceType::new_with_args(union_func, results));
      ctx_ref.fresh_instances.push(ty);
      ty
    };

    return Some(TypeFunctionReductionResult {
      result: Some(result_ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    });
  }

  None
}
