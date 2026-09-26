use alloc::vec::Vec;

use ulua_common::dfint;

use crate::{
  enums::reduction::Reduction,
  functions::{begin_type::begin_union_type, follow_type, get_type},
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
  ctx: &mut TypeFunctionContext,
) -> Option<TypeFunctionReductionResult>
where
  // Single shared late-bound lifetime: closures with two elided reference
  // parameters cannot be inferred as `for<'a, 'b>` (higher-ranked lifetime
  // error); the forwarding sites all hold both slices from the same borrow.
  F: for<'a> FnMut(
    TypeId,
    &'a [TypeId],
    &'a [TypePackId],
    &'a mut TypeFunctionContext,
  ) -> TypeFunctionReductionResult,
{
  let mut reduction_status = Reduction::MaybeOk;
  let mut blocked_types = Vec::new();
  let mut results = Vec::new();
  let mut cartesian_product_size = 1usize;

  let mut first_union_options: Option<Vec<TypeId>> = None;
  let mut union_index = 0usize;
  let mut arguments = type_params.to_vec();

  for (index, argument) in arguments.iter().copied().enumerate() {
    let ty = follow_type::follow(argument);
    let Some(union) = get_type::get::<UnionType>(ty) else {
      continue;
    };

    // C++ `std::distance(begin(ut), end(ut))` / `for (TypeId option :
    // firstUnion)`——UnionTypeIterator 展平嵌套 union 并 follow,裸 options
    // 会漏掉嵌套成员。
    let flattened: Vec<TypeId> = begin_union_type(union).collect();
    if first_union_options.is_none() {
      first_union_options = Some(flattened.clone());
      union_index = index;
    }

    cartesian_product_size = cartesian_product_size.saturating_mul(flattened.len());
    if (dfint::LuauTypeFamilyApplicationCartesianProductLimit.get() as usize)
      <= cartesian_product_size
    {
      return Some(TypeFunctionReductionResult::erroneous());
    }
  }

  let first_union_options = first_union_options?;

  for option in first_union_options {
    arguments[union_index] = option;

    let result = f(instance, &arguments, pack_params, &mut *ctx);
    blocked_types.extend(result.blocked_types.iter().copied());

    if result.reduction_status != Reduction::MaybeOk {
      reduction_status = result.reduction_status;
    }

    // 双写合一：复合「非 MaybeOk 或 result 为 None → break」拆为状态早断 +
    // `let Some` 绑定，push 点 Some 由链式判定蕴含。
    if reduction_status != Reduction::MaybeOk {
      break;
    }
    let Some(result_type) = result.result else {
      break;
    };

    results.push(result_type);
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
      return Some(TypeFunctionReductionResult::reduction(results[0]));
    }

    // SAFETY: 分派点物化的独占借用；arena/builtins 均由 ctx 持有。
    let result_ty = unsafe {
      let union_func = &ctx.builtins.as_ref().type_functions.union_func;
      let ty = ctx
        .arena
        .as_mut()
        .add_type(TypeFunctionInstanceType::new_with_args(union_func, results));
      ctx.fresh_instances.push(ty);
      ty
    };

    return Some(TypeFunctionReductionResult::reduction(result_ty));
  }

  None
}
