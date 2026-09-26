use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, get_type, is_pending::is_pending, simplify_union::simplify_union},
  records::{
    arena_handle::Handle, type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn union_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  unsafe {
    if !pack_params.is_empty() {
      (*ctx.ice.as_ptr()).ice_string(
                "union type function: encountered a type function instance without the required argument structure",
            );
      LUAU_ASSERT!(false);
    }

    if type_params.len() == 1 {
      return TypeFunctionReductionResult::reduction(follow_type::follow(type_params[0]));
    }

    let mut options = Vec::new();
    let mut blocking_types = Vec::new();
    let mut worklist = type_params.to_vec();

    while let Some(ty) = worklist.pop() {
      let ty = follow_type::follow(ty);

      if let Some(union_ty) = get_type::get::<UnionType>(ty).as_ref() {
        worklist.extend(union_ty.options.iter().copied());
        continue;
      }

      if let Some(type_function_instance) = get_type::get::<TypeFunctionInstanceType>(ty).as_ref() {
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
      return TypeFunctionReductionResult::no_reduction(blocking_types);
    }

    let mut result_ty = ctx.builtins.as_ref().never_type;
    for ty in options {
      let simplified = simplify_union(
        Handle::from_nonnull(ctx.builtins),
        Handle::from_nonnull(ctx.arena),
        result_ty,
        ty,
      );
      if !simplified.blocked_types.empty() {
        return TypeFunctionReductionResult::no_reduction(
          simplified.blocked_types.iter().copied().collect(),
        );
      }

      result_ty = simplified.result;
    }

    TypeFunctionReductionResult::reduction(result_ty)
  }
}
