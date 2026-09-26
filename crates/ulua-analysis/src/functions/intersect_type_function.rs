use alloc::vec::Vec;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    follow_type, get_type, intersect_with_simple_discriminant::intersect_with_simple_discriminant,
    is_pending::is_pending, simplify_intersection_simplify::simplify_intersection,
  },
  records::{
    arena_handle::Handle, generic_type::GenericType, intersection_type::IntersectionType,
    never_type::NeverType, no_refine_type::NoRefineType, simplify_result::SimplifyResult,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn intersect_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: `ctx` 是分派点自 NonNull 物化的本次调用独占借用，在整个归约期间独占存活，
  // 与单线程调用栈中的其它借用无重叠。其 `ice`/`builtins`/`arena`/`solver` 字段是构造时接线的
  // NonNull/裸指针，均非空且比 ctx 长寿：`(*ctx.ice.as_ptr()).ice_string(..)`、
  // `(*ctx.builtins.as_ptr()).never_type/unknown_type`、`(*ctx.arena.as_ptr()).add_type(..)` 因
  // 此在借用覆盖期内合法。`type_params`/`pack_params` 为调用方传入的切片借用。全程单线程串行。
  unsafe {
    if !pack_params.is_empty() {
      (*ctx.ice.as_ptr()).ice_string("intersect type function: encountered a type function instance without the required argument structure");
      LUAU_ASSERT!(false);
    }

    // if we only have one parameter, there's nothing to do.
    if type_params.len() == 1 {
      return TypeFunctionReductionResult::reduction(follow_type::follow(type_params[0]));
    }

    // we need to follow all of the type parameters.
    let mut types: Vec<TypeId> = Vec::with_capacity(type_params.len());
    for ty in type_params.iter().copied() {
      types.push(follow_type::follow(ty));
    }

    // if we only have two parameters and one is `*no-refine*`, we're all done.
    if types.len() == 2 {
      let t0 = types[0];
      let t1 = types[1];
      if get_type::get::<NoRefineType>(t1).is_some() {
        return TypeFunctionReductionResult::reduction(t0);
      } else if get_type::get::<NoRefineType>(t0).is_some() {
        return TypeFunctionReductionResult::reduction(t1);
      }
    }

    // check to see if the operand types are resolved enough, and wait to reduce if not
    // if any of them are `never`, the intersection will always be `never`, so we can reduce directly.
    for ty in types.iter().copied() {
      if is_pending(ty, ctx.solver) {
        return TypeFunctionReductionResult::no_reduction(vec![ty]);
      } else if get_type::get::<NeverType>(ty).is_some() {
        return TypeFunctionReductionResult::reduction((*ctx.builtins.as_ptr()).never_type);
      }
    }

    // fold over the types with `simplifyIntersection`
    let mut result_ty: TypeId = (*ctx.builtins.as_ptr()).unknown_type;

    // collect types which caused intersection to return never
    let mut unintersectable_types: DenseHashSet<TypeId> = DenseHashSet::default();

    for ty in types.iter().copied() {
      // skip any `*no-refine*` types.
      if get_type::get::<NoRefineType>(ty).is_some() {
        continue;
      }

      if let Some(simple_result) = intersect_with_simple_discriminant(
        Handle::from_nonnull(ctx.builtins),
        Handle::from_nonnull(ctx.arena),
        result_ty,
        ty,
      ) {
        if get_type::get::<NeverType>(simple_result).is_some() {
          unintersectable_types.insert(follow_type::follow(ty));
        } else {
          result_ty = simple_result;
        }
        continue;
      }

      let result: SimplifyResult = simplify_intersection(
        Handle::from_nonnull(ctx.builtins),
        Handle::from_nonnull(ctx.arena),
        result_ty,
        ty,
      );

      // If simplifying the intersection returned never, note the type we tried to intersect it with, and continue trying to intersect with the
      // rest
      if get_type::get::<NeverType>(result.result).is_some() {
        unintersectable_types.insert(follow_type::follow(ty));
        continue;
      }

      for blocked_type in result.blocked_types.iter() {
        let blocked_ty = *blocked_type;
        if get_type::get::<GenericType>(blocked_ty).is_none() {
          return TypeFunctionReductionResult::no_reduction(
            result.blocked_types.iter().copied().collect(),
          );
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
        return TypeFunctionReductionResult::reduction(intersection);
      } else {
        let mut iter = unintersectable_types.iter();
        // 块首已 insert(result_ty) 使基数 ≥1，且此处为 `size() > 1` 的 else 臂
        // 即 size == 1，唯一元素存在。
        let only = *iter
          .next()
          .expect("insert 后基数 ≥1 且 size()>1 之 else 臂即 size==1");
        return TypeFunctionReductionResult::reduction(only);
      }
    }

    // if the intersection simplifies to `never`, this gives us bad autocomplete.
    // we'll just produce the intersection plainly instead, but this might be revisitable
    // if we ever give `never` some kind of "explanation" trail.
    if get_type::get::<NeverType>(result_ty).is_some() {
      let intersection = (*ctx.arena.as_ptr()).add_type(IntersectionType {
        parts: type_params.to_vec(),
      });
      return TypeFunctionReductionResult::reduction(intersection);
    }

    TypeFunctionReductionResult::reduction(result_ty)
  }
}
