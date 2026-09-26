use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `GeneralizationResult<TypeId> generalizeType(...)`
/// (Generalization.cpp:730-837). Replace a single free type by its bounds
/// according to the polarity provided.
use crate::enums::polarity::Polarity;
use crate::{
  functions::{
    follow_type, get_mutable_type, get_type, is_known::is_known, is_positive::is_positive,
    remove_type::remove_type,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType,
    generalization_params::GeneralizationParams, generalization_result::GeneralizationResult,
    generic_type::GenericType, intersection_type::IntersectionType, never_type::NeverType,
    scope::Scope, r#type::Type, type_arena::TypeArena, unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn generalize_type(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  scope: *mut Scope,
  free_ty: TypeId,
  params: &GeneralizationParams,
) -> GeneralizationResult {
  let free_ty = follow_type::follow(free_ty);

  // DELIBERATE DEVIATION（cpp Generalization.cpp:1303 的 generalizeType 头部
  // collapseDirectBoundCycleAt 未移植）：cpp 侧该调用对 table.clone 求解器路径
  // 是空操作——cpp 的自由类型上界是带 table 分量的 IntersectionType 节点，
  // get_direct_free_neighbor（Generalization.cpp:817-822）follow 后非 Free，
  // 环不可达；而 Rust 约束求解器在同一场景把上界直接绑成裸 FreeType，逐点
  // 塌缩会把求解器仍持有依赖边的两个自由类型（'dictionary 与 clone 实例
  // 中介自由数）合并绑定，触发后续 TypeChecker2 NotATable。故此处只移植
  // cpp 的「非自由类型早退」（Generalization.cpp:1306-1309），环塌缩仅保留
  // generalize() 的批量预处理 collapseFreeTypeCycles（Generalization.cpp:
  // 1467/1549），oracle 用例 collapse_cycle_with_external_bound_in_union
  // 走的正是该批量入口。
  let ft = get_mutable_type::get_mutable::<FreeType>(free_ty);
  if ft.is_none() {
    return GeneralizationResult {
      result: Some(free_ty),
      was_replaced_by_generic: false,
      resource_limits_exceeded: false,
    };
  }
  let ft = ft.expect("上方 is_none 分支已排除");

  LUAU_ASSERT!(is_known(params.polarity));

  let has_lower_bound = get_type::get::<NeverType>(follow_type::follow(ft.lower_bound)).is_none();
  let has_upper_bound = get_type::get::<UnknownType>(follow_type::follow(ft.upper_bound)).is_none();

  let is_within_function = !params.found_outside_functions;

  if !has_lower_bound && !has_upper_bound {
    if !is_within_function {
      emplace_bound(free_ty, builtin_types.get().unknown_type);
    } else {
      emplace_generic(free_ty, scope, params.polarity);
      return result_generic(free_ty);
    }
  }
  // It is possible that this free type has other free types in its upper or
  // lower bounds. If so we must replace those references with never (lower)
  // or unknown (upper) to avoid tautological bounds like a <: a <: unknown.
  //
  // 保留 DELIBERATE DEVIATION：cpp-739 已把这两个「邻居界转发」分支删除
  // （Generalization.cpp:1357-1360 注释：由 generalizeType 头部的
  // collapseDirectBoundCycleAt 取代）。Rust 侧头部逐点塌缩未移植（见上方
  // DEVIATION 注释），求解器产生的直接自由界 2-环仍靠这两个分支兜底
  // （type_infer_oop_react_style_oo、cli_186992 依赖此行为），故按 dev 原样
  // 保留。
  else if is_positive(params.polarity) && !has_upper_bound {
    let lb = follow_type::follow(ft.lower_bound);
    if let Some(lower_free) = get_mutable_type::get_mutable::<FreeType>(lb)
      && lower_free.upper_bound == free_ty
    {
      // Generalizing 'a in:  LO <: 'b <: 'a <: UP
      // ... we can hold onto the bound UP and forward it to 'b.
      let upper_bound = follow_type::follow(ft.upper_bound);
      remove_type(arena, builtin_types, upper_bound, free_ty);
      lower_free.upper_bound = follow_type::follow(upper_bound);
    } else {
      remove_type(arena, builtin_types, lb, free_ty);
    }

    if follow_type::follow(lb) != free_ty {
      emplace_bound(free_ty, lb);
    } else if !is_within_function {
      emplace_bound(free_ty, builtin_types.get().unknown_type);
    } else {
      // if the lower bound is the type in question (eg 'a <: 'a), we
      // don't actually have a lower bound.
      emplace_generic(free_ty, scope, params.polarity);
      return result_generic(free_ty);
    }
  } else {
    let ub = follow_type::follow(ft.upper_bound);
    if let Some(upper_free) = get_mutable_type::get_mutable::<FreeType>(ub)
      && upper_free.lower_bound == free_ty
    {
      // Generalizing 'a in:  LO <: 'a <: 'b <: UP
      // ... we can hold onto the bound LO and forward it to 'b.
      let lower_bound = follow_type::follow(ft.lower_bound);
      remove_type(arena, builtin_types, lower_bound, free_ty);
      upper_free.lower_bound = follow_type::follow(lower_bound);
    } else {
      remove_type(arena, builtin_types, ub, free_ty);
    }

    if follow_type::follow(ub) != free_ty {
      emplace_bound(free_ty, ub);
    } else if !is_within_function || params.use_count == 1 {
      // For a free type  A <: 'b < C  we approximately generalize to the
      // intersection of its bounds, clipping the free type from the upper
      // and lower bounds, then cleaning the resulting intersection.
      let lower_bound = ft.lower_bound;
      remove_type(arena, builtin_types, lower_bound, free_ty);
      let cleaned_ty = arena.get_mut().add_type(IntersectionType {
        parts: alloc::vec![ft.lower_bound, ub],
      });
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

/// C++ `emplaceType<BoundType>(asMutable(ty), bound_to)`。
pub(crate) fn emplace_bound(ty: TypeId, bound_to: TypeId) {
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
