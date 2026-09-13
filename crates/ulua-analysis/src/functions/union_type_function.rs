use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_pending::is_pending,
    simplify_union::simplify_union,
  },
  records::{
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn union_type_function(
  _instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  unsafe {
    let ctx = &mut *ctx;

    if !pack_params.is_empty() {
      (*ctx.ice.as_ptr()).ice_string(
                "union type function: encountered a type function instance without the required argument structure",
            );
      LUAU_ASSERT!(false);
    }

    if type_params.len() == 1 {
      return TypeFunctionReductionResult {
        result: Some(follow_type_id(type_params[0])),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let mut options = Vec::new();
    let mut blocking_types = Vec::new();
    let mut worklist = type_params;

    while let Some(ty) = worklist.pop() {
      let ty = follow_type_id(ty);

      if let Some(union_ty) = get_type_id::<UnionType>(ty).as_ref() {
        worklist.extend(union_ty.options.iter().copied());
        continue;
      }

      if let Some(type_function_instance) = get_type_id::<TypeFunctionInstanceType>(ty).as_ref() {
        let function = type_function_instance.function.as_ref();
        if function.name == ctx.builtins.as_ref().type_functions.union_func.name {
          worklist.extend(type_function_instance.type_arguments.iter().copied());
          continue;
        }

        options.push(ty);
        blocking_types.push(ty);
        continue;
      }

      options.push(ty);
      if is_pending(ty, ctx.solver) {
        blocking_types.push(ty);
      }
    }

    if !blocking_types.is_empty() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: blocking_types,
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let mut result_ty = ctx.builtins.as_ref().never_type;
    for ty in options {
      let simplified = simplify_union(ctx.builtins.as_ptr(), ctx.arena.as_ptr(), result_ty, ty);
      if !simplified.blocked_types.empty() {
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::MaybeOk,
          blocked_types: simplified.blocked_types.iter().copied().collect(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      }

      result_ty = simplified.result;
    }

    TypeFunctionReductionResult {
      result: Some(result_ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
