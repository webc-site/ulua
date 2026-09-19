use core::mem::zeroed;

use crate::{
  fflag::LuauTypeFunctionRobustness,
  functions::{
    are_equal_type_function_runtime::are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type,
    are_equal_type_function_runtime_alt_g::are_equal_are_equal_state_type_function_union_type_type_function_union_type,
    are_equal_type_function_runtime_alt_h::are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type,
    are_equal_type_function_runtime_alt_i::are_equal_are_equal_state_type_function_negation_type_type_function_negation_type,
    are_equal_type_function_runtime_alt_j::are_equal_are_equal_state_type_function_table_type_type_function_table_type,
    are_equal_type_function_runtime_alt_k::are_equal_are_equal_state_type_function_function_type_type_function_function_type,
    are_equal_type_function_runtime_alt_l::are_equal_are_equal_state_type_function_extern_type_type_function_extern_type,
  },
  records::{
    are_equal_state::AreEqualState, recursion_limiter::RecursionLimiter,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
  type_aliases::type_function_type_variant::TypeFunctionTypeVariantMember,
};

/// 对照 C++ `get<T>(&tv)`（TypeFunctionRuntime.cpp:275-281）：双侧变体一次
/// 下转成引用对。直接走安全 trait `T::get_if`，tv 由 `&TypeFunctionType`
/// 引用保证非空，语义与 C++ `get<T>` 完全一致，免去 unsafe。
fn pair<'a, 'b, T: TypeFunctionTypeVariantMember>(
  lhs: &'a TypeFunctionType,
  rhs: &'b TypeFunctionType,
) -> Option<(&'a T, &'b T)> {
  Some((T::get_if(&lhs.type_variant)?, T::get_if(&rhs.type_variant)?))
}

pub fn are_equal_are_equal_state_type_function_type_type_function_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionType,
  rhs: &TypeFunctionType,
) -> bool {
  let mut _ra: Option<RecursionLimiter> = None;
  if LuauTypeFunctionRobustness.get() {
    // SAFETY: base/native_stack_guard 由紧随的 recursion_limiter_recursion_limiter
    // 全量覆写，zeroed 仅作占位（crate 统一惯用法，见 functions/has_length.rs）。
    let mut rl = RecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    rl.recursion_limiter_recursion_limiter("areEqual", &mut seen.recursion_count, 100);
    _ra = Some(rl);
  }

  if lhs.type_variant.index() != rhs.type_variant.index() {
    return false;
  }

  // 对照 C++ areEqual 主体（TypeFunctionRuntime.cpp:2404-2447）：逐变体下转，
  // 顺序与 C++ 完全一致，全部经由安全 get_if，免裸指针。
  if let Some((lp, rp)) = pair::<TypeFunctionPrimitiveType>(lhs, rhs) {
    return lp.r#type == rp.r#type;
  }

  if pair::<TypeFunctionAnyType>(lhs, rhs).is_some() {
    return true;
  }

  if pair::<TypeFunctionUnknownType>(lhs, rhs).is_some() {
    return true;
  }

  if pair::<TypeFunctionNeverType>(lhs, rhs).is_some() {
    return true;
  }

  if let Some((lf, rf)) = pair::<TypeFunctionSingletonType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionUnionType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_union_type_type_function_union_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionIntersectionType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionNegationType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
      seen, lf, rf,
    );
  }

  if let Some((lt, rt)) = pair::<TypeFunctionTableType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_table_type_type_function_table_type(
      seen, lt, rt,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionFunctionType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_function_type_type_function_function_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionExternType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_extern_type_type_function_extern_type(
      seen, lf, rf,
    );
  }

  // C++（TypeFunctionRuntime.cpp:2456-2462）：Generic 分支逐字段比较。
  if let Some((lg, rg)) = pair::<TypeFunctionGenericType>(lhs, rhs) {
    return lg.is_named == rg.is_named && lg.is_pack == rg.is_pack && lg.name == rg.name;
  }

  false
}
