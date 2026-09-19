use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type::follow_type_id, is_blocked_or_unsolved_type::is_blocked_or_unsolved_type,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn or_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 2 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("or type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let lhs_ty = follow_type_id(type_params[0]);
  let rhs_ty = follow_type_id(type_params[1]);

  // t1 = or<lhs, t1> ~> lhs
  if follow_type_id(rhs_ty) == instance && lhs_ty != rhs_ty {
    return TypeFunctionReductionResult {
      result: Some(lhs_ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  // t1 = or<t1, rhs> ~> rhs
  if follow_type_id(lhs_ty) == instance && lhs_ty != rhs_ty {
    return TypeFunctionReductionResult {
      result: Some(rhs_ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if is_blocked_or_unsolved_type(lhs_ty) {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::from([lhs_ty]),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  } else if is_blocked_or_unsolved_type(rhs_ty) {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::from([rhs_ty]),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  let filtered_lhs = unsafe {
    simplify_intersection(
      ctx_ref.builtins.as_ptr(),
      ctx_ref.arena.as_ptr(),
      lhs_ty,
      ctx_ref.builtins.as_ref().truthy_type,
    )
  };
  let overall_result = simplify_union(
    ctx_ref.builtins.as_ptr(),
    ctx_ref.arena.as_ptr(),
    rhs_ty,
    filtered_lhs.result,
  );

  let mut blocked_types: Vec<TypeId> = Vec::new();
  for ty in filtered_lhs.blocked_types.iter() {
    blocked_types.push(*ty);
  }
  for ty in overall_result.blocked_types.iter() {
    blocked_types.push(*ty);
  }

  TypeFunctionReductionResult {
    result: Some(overall_result.result),
    reduction_status: Reduction::MaybeOk,
    blocked_types,
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  }
}
