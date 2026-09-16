use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `GeneralizationResult<TypeId> generalizeType(...)`
/// (Generalization.cpp:730-837). Replace a single free type by its bounds
/// according to the polarity provided.
use crate::enums::polarity::Polarity;
use crate::{
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, is_known::is_known, is_positive::is_positive,
    remove_type::remove_type,
  },
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, generalization_params::GeneralizationParams,
    generalization_result::GeneralizationResult, generic_type::GenericType,
    intersection_type::IntersectionType, never_type::NeverType, scope::Scope, r#type::Type,
    type_arena::TypeArena, unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn generalize_type(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  scope: *mut Scope,
  free_ty: TypeId,
  params: &GeneralizationParams,
) -> GeneralizationResult {
  let free_ty = follow_type_id(free_ty);

  let ft = get_mutable_type_id::<FreeType>(free_ty);
  LUAU_ASSERT!(ft.is_some());
  let ft = ft.unwrap();

  LUAU_ASSERT!(is_known(params.polarity));

  let has_lower_bound = get_type_id::<NeverType>(follow_type_id(ft.lower_bound)).is_none();
  let has_upper_bound = get_type_id::<UnknownType>(follow_type_id(ft.upper_bound)).is_none();

  let is_within_function = !params.found_outside_functions;

  if !has_lower_bound && !has_upper_bound {
    if !is_within_function {
      // SAFETY: builtin_types 指向常驻的 BuiltinTypes
      emplace_bound(free_ty, unsafe { (*builtin_types).unknown_type });
    } else {
      emplace_generic(free_ty, scope, params.polarity);
      return result_generic(free_ty);
    }
  }
  // It is possible that this free type has other free types in its upper or
  // lower bounds. If so we must replace those references with never (lower)
  // or unknown (upper) to avoid tautological bounds like a <: a <: unknown.
  else if is_positive(params.polarity) && !has_upper_bound {
    let lb = follow_type_id(ft.lower_bound);
    if let Some(lower_free) = get_mutable_type_id::<FreeType>(lb)
      && lower_free.upper_bound == free_ty
    {
      // Generalizing 'a in:  LO <: 'b <: 'a <: UP
      // ... we can hold onto the bound UP and forward it to 'b.
      let upper_bound = follow_type_id(ft.upper_bound);
      remove_type(arena, builtin_types, upper_bound, free_ty);
      lower_free.upper_bound = follow_type_id(upper_bound);
    } else {
      remove_type(arena, builtin_types, lb, free_ty);
    }

    if follow_type_id(lb) != free_ty {
      emplace_bound(free_ty, lb);
    } else if !is_within_function {
      // SAFETY: builtin_types 指向常驻的 BuiltinTypes
      emplace_bound(free_ty, unsafe { (*builtin_types).unknown_type });
    } else {
      // if the lower bound is the type in question (eg 'a <: 'a), we
      // don't actually have a lower bound.
      emplace_generic(free_ty, scope, params.polarity);
      return result_generic(free_ty);
    }
  } else {
    let ub = follow_type_id(ft.upper_bound);
    if let Some(upper_free) = get_mutable_type_id::<FreeType>(ub)
      && upper_free.lower_bound == free_ty
    {
      // Generalizing 'a in:  LO <: 'a <: 'b <: UP
      // ... we can hold onto the bound LO and forward it to 'b.
      let lower_bound = follow_type_id(ft.lower_bound);
      remove_type(arena, builtin_types, lower_bound, free_ty);
      upper_free.lower_bound = follow_type_id(lower_bound);
    } else {
      remove_type(arena, builtin_types, ub, free_ty);
    }

    if follow_type_id(ub) != free_ty {
      emplace_bound(free_ty, ub);
    } else if !is_within_function || params.use_count == 1 {
      // For a free type  A <: 'b < C  we approximately generalize to the
      // intersection of its bounds, clipping the free type from the upper
      // and lower bounds, then cleaning the resulting intersection.
      let lower_bound = ft.lower_bound;
      remove_type(arena, builtin_types, lower_bound, free_ty);
      // SAFETY: arena 指向调用方保证有效的 TypeArena
      let cleaned_ty = unsafe {
        (*arena).add_type(IntersectionType {
          parts: alloc::vec![ft.lower_bound, ub],
        })
      };
      remove_type(arena, builtin_types, cleaned_ty, free_ty);
      emplace_bound(free_ty, cleaned_ty);
    } else {
      // if the upper bound is the type in question, we don't actually
      // have an upper bound.
      emplace_generic(free_ty, scope, params.polarity);
      return result_generic(free_ty);
    }
  }

  GeneralizationResult {
    result: Some(free_ty),
    was_replaced_by_generic: false,
    resource_limits_exceeded: false,
  }
}

/// C++ `emplaceType<BoundType>(asMutable(ty), bound_to)`.
fn emplace_bound(ty: TypeId, bound_to: TypeId) {
  // SAFETY: ty 指向类型 arena 中的 Type，原地改写 variant
  unsafe {
    (*(ty as *mut Type)).ty = TypeVariant::Bound(bound_to);
  }
}

/// C++ `emplaceType<GenericType>(asMutable(ty), scope, polarity)`.
fn emplace_generic(ty: TypeId, scope: *mut Scope, polarity: Polarity) {
  // SAFETY: ty 指向类型 arena 中的 Type，原地改写 variant
  unsafe {
    (*(ty as *mut Type)).ty =
      TypeVariant::Generic(GenericType::generic_type_scope_polarity(scope, polarity));
  }
}

#[inline]
fn result_generic(free_ty: TypeId) -> GeneralizationResult {
  GeneralizationResult {
    result: Some(free_ty),
    was_replaced_by_generic: true,
    resource_limits_exceeded: false,
  }
}
