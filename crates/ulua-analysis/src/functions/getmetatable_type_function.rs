use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    getmetatable_helper::getmetatable_helper, is_pending::is_pending,
  },
  records::{
    intersection_type::IntersectionType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn getmetatable_type_function(
  _instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("getmetatable type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let location = if !ctx_ref.constraint.is_null() {
    unsafe { (*ctx_ref.constraint).location }
  } else {
    Location::new(Default::default(), Default::default())
  };

  let target_ty = follow_type_id(type_params[0]);

  if unsafe { is_pending(target_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![target_ty],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if let Some(ut) = get_type_id::<UnionType>(target_ty) {
    let mut options: Vec<TypeId> = Vec::with_capacity(ut.options.len());
    for option in &ut.options {
      let result = getmetatable_helper(*option, &location, ctx_ref);
      if result.result.is_none() {
        return result;
      }
      options.push(*result.result.as_ref().unwrap());
    }

    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.arena.as_ptr()).add_type(UnionType { options }) }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if let Some(it) = get_type_id::<IntersectionType>(target_ty) {
    let mut parts: Vec<TypeId> = Vec::with_capacity(it.parts.len());
    let mut errored_with_unknown = false;

    for part in &it.parts {
      let result = getmetatable_helper(*part, &location, ctx_ref);
      if result.result.is_none() {
        if get_type_id::<UnknownType>(follow_type_id(*part)).is_some() {
          errored_with_unknown = true;
          continue;
        } else {
          return result;
        }
      }
      parts.push(*result.result.as_ref().unwrap());
    }

    if errored_with_unknown && parts.is_empty() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      };
    }

    if parts.len() == 1 {
      return TypeFunctionReductionResult {
        result: Some(parts[0]),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      };
    }

    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.arena.as_ptr()).add_type(IntersectionType { parts }) }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  getmetatable_helper(target_ty, &location, ctx_ref)
}
