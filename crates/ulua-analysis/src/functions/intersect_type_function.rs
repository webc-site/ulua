use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    intersect_with_simple_discriminant::intersect_with_simple_discriminant, is_pending::is_pending,
    simplify_intersection_simplify::simplify_intersection,
  },
  records::{
    generic_type::GenericType, intersection_type::IntersectionType, never_type::NeverType,
    no_refine_type::NoRefineType, simplify_result::SimplifyResult,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn intersect_type_function(
  _instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  unsafe {
    let ctx = &mut *ctx;

    if !pack_params.is_empty() {
      (*ctx.ice.as_ptr()).ice_string("intersect type function: encountered a type function instance without the required argument structure");
      LUAU_ASSERT!(false);
    }

    // if we only have one parameter, there's nothing to do.
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

    // we need to follow all of the type parameters.
    let mut types: Vec<TypeId> = Vec::with_capacity(type_params.len());
    for ty in type_params.iter().copied() {
      types.push(follow_type_id(ty));
    }

    // if we only have two parameters and one is `*no-refine*`, we're all done.
    if types.len() == 2 {
      let t0 = types[0];
      let t1 = types[1];
      if !get_type_id::<NoRefineType>(t1).is_none() {
        return TypeFunctionReductionResult {
          result: Some(t0),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      } else if !get_type_id::<NoRefineType>(t0).is_none() {
        return TypeFunctionReductionResult {
          result: Some(t1),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      }
    }

    // check to see if the operand types are resolved enough, and wait to reduce if not
    // if any of them are `never`, the intersection will always be `never`, so we can reduce directly.
    for ty in types.iter().copied() {
      if is_pending(ty, ctx.solver) {
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::MaybeOk,
          blocked_types: vec![ty],
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      } else if !get_type_id::<NeverType>(ty).is_none() {
        return TypeFunctionReductionResult {
          result: Some((*ctx.builtins.as_ptr()).never_type),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      }
    }

    // fold over the types with `simplifyIntersection`
    let mut result_ty: TypeId = (*ctx.builtins.as_ptr()).unknown_type;

    // collect types which caused intersection to return never
    let mut unintersectable_types: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());

    for ty in types.iter().copied() {
      // skip any `*no-refine*` types.
      if !get_type_id::<NoRefineType>(ty).is_none() {
        continue;
      }

      if let Some(simple_result) =
        intersect_with_simple_discriminant(ctx.builtins.as_ptr(), ctx.arena.as_ptr(), result_ty, ty)
      {
        if !get_type_id::<NeverType>(simple_result).is_none() {
          unintersectable_types.insert(follow_type_id(ty));
        } else {
          result_ty = simple_result;
        }
        continue;
      }

      let result: SimplifyResult =
        simplify_intersection(ctx.builtins.as_ptr(), ctx.arena.as_ptr(), result_ty, ty);

      // If simplifying the intersection returned never, note the type we tried to intersect it with, and continue trying to intersect with the
      // rest
      if !get_type_id::<NeverType>(result.result).is_none() {
        unintersectable_types.insert(follow_type_id(ty));
        continue;
      }

      for blocked_type in result.blocked_types.iter() {
        let blocked_ty = *blocked_type;
        if get_type_id::<GenericType>(blocked_ty).is_none() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: result.blocked_types.iter().copied().collect(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
          };
        }
      }

      result_ty = result.result;
    }

    if !unintersectable_types.empty() {
      unintersectable_types.insert(result_ty);

      if unintersectable_types.size() > 1 {
        let mut parts: Vec<TypeId> = Vec::with_capacity(unintersectable_types.size());
        for ty in unintersectable_types.iter() {
          parts.push(*ty);
        }

        let intersection = (*ctx.arena.as_ptr()).add_type(IntersectionType { parts });
        return TypeFunctionReductionResult {
          result: Some(intersection),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      } else {
        let mut iter = unintersectable_types.iter();
        let only = *iter.next().unwrap();
        return TypeFunctionReductionResult {
          result: Some(only),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      }
    }

    // if the intersection simplifies to `never`, this gives us bad autocomplete.
    // we'll just produce the intersection plainly instead, but this might be revisitable
    // if we ever give `never` some kind of "explanation" trail.
    if !get_type_id::<NeverType>(result_ty).is_none() {
      let intersection = (*ctx.arena.as_ptr()).add_type(IntersectionType { parts: type_params });
      return TypeFunctionReductionResult {
        result: Some(intersection),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
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
