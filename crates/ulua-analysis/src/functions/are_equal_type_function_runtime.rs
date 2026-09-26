#![deny(unsafe_code)]

use ulua_common::{fflag::LuauTypeFunctionRobustness, records::variant::Variant2};

use crate::{
  functions::seen_set_contains::seen_set_contains,
  records::{
    are_equal_state::AreEqualState, recursion_limiter::RecursionLimiter,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_type_id::AsTypeFunctionType,
    type_function_type_pack_id::{AsTypeFunctionTypePack, TypeFunctionTypePackId},
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// cpp `areEqual` 各变体入口共用的环检测守卫：同一对 `(lhs, rhs)` 地址若已在
/// `seen` 中比较过，[`seen_set_contains`] 判定相等并短路递归。泛型 `<T>` 把
/// `lhs/rhs as *const T as *const ()` 的裸指针身份转换样板收敛到单点
/// （此处裸指针是「按对象地址去重」的固有身份语义，非可 Rust 化的借用）。
fn already_seen<T>(seen: &mut AreEqualState, lhs: &T, rhs: &T) -> bool {
  seen_set_contains(
    seen,
    lhs as *const T as *const (),
    rhs as *const T as *const (),
  )
}

pub fn are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionSingletonType,
  rhs: &TypeFunctionSingletonType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  match (&lhs.variant, &rhs.variant) {
    (Variant2::V0(lp), Variant2::V0(rp)) => {
      return lp.value == rp.value;
    }
    (Variant2::V1(lp), Variant2::V1(rp)) => {
      return lp.value == rp.value;
    }
    _ => {}
  }

  false
}

pub fn are_equal_are_equal_state_type_function_union_type_type_function_union_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionUnionType,
  rhs: &TypeFunctionUnionType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.components.len() != rhs.components.len() {
    return false;
  }

  // components 按下标一一对应，zip 替代迭代器手动 next
  for (&l, &r) in lhs.components.iter().zip(&rhs.components) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      l.as_type(),
      r.as_type(),
    ) {
      return false;
    }
  }

  true
}

pub fn are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionIntersectionType,
  rhs: &TypeFunctionIntersectionType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.components.len() != rhs.components.len() {
    return false;
  }

  // components 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.components.iter().zip(&rhs.components) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      l.as_type(),
      r.as_type(),
    ) {
      return false;
    }
  }

  true
}

pub fn are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionNegationType,
  rhs: &TypeFunctionNegationType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  are_equal_are_equal_state_type_function_type_type_function_type(
    seen,
    lhs.type_id.as_type(),
    rhs.type_id.as_type(),
  )
}

pub fn are_equal_are_equal_state_type_function_table_type_type_function_table_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTableType,
  rhs: &TypeFunctionTableType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.props.len() != rhs.props.len() {
    return false;
  }

  match (&lhs.indexer, &rhs.indexer) {
    (Some(l_indexer), Some(r_indexer)) => {
      if !are_equal_are_equal_state_type_function_type_type_function_type(
        seen,
        l_indexer.key_type.as_type(),
        r_indexer.key_type.as_type(),
      ) {
        return false;
      }

      if !are_equal_are_equal_state_type_function_type_type_function_type(
        seen,
        l_indexer.value_type.as_type(),
        r_indexer.value_type.as_type(),
      ) {
        return false;
      }
    }
    (None, None) => {}
    _ => return false,
  }

  let mut l_iter = lhs.props.iter();
  let mut r_iter = rhs.props.iter();

  while let (Some((l_key, l_prop)), Some((r_key, r_prop))) = (l_iter.next(), r_iter.next()) {
    let _ = l_key;
    let _ = r_key;

    match (l_prop.read_ty, r_prop.read_ty) {
      (Some(l), Some(r)) => {
        if !are_equal_are_equal_state_type_function_type_type_function_type(
          seen,
          l.as_type(),
          r.as_type(),
        ) {
          return false;
        }
      }
      (None, None) => {}
      _ => return false,
    }

    match (l_prop.write_ty, r_prop.write_ty) {
      (Some(l), Some(r)) => {
        if !are_equal_are_equal_state_type_function_type_type_function_type(
          seen,
          l.as_type(),
          r.as_type(),
        ) {
          return false;
        }
      }
      (None, None) => {}
      _ => return false,
    }
  }

  true
}

/// 可空 arena pack 指针对（如函数签名 `arg_types`/`ret_types`）的相等判定：
/// 两侧同空视为相等，一空一非空视为不等，皆非空时递归 [`are_equal_
/// are_equal_state_type_function_type_pack_var_type_function_type_pack_var`]。
/// 收敛 function/variadic 等入口逐字重复的两段判空守卫。
fn are_equal_optional_pack(
  seen: &mut AreEqualState,
  lhs: TypeFunctionTypePackId,
  rhs: TypeFunctionTypePackId,
) -> bool {
  match (lhs.as_pack_opt(), rhs.as_pack_opt()) {
    (None, None) => true,
    (Some(l), Some(r)) => {
      are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
        seen, l, r,
      )
    }
    _ => false,
  }
}

pub fn are_equal_are_equal_state_type_function_function_type_type_function_function_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionFunctionType,
  rhs: &TypeFunctionFunctionType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.generics.len() != rhs.generics.len() {
    return false;
  }

  // generics 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.generics.iter().zip(&rhs.generics) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      l.as_type(),
      r.as_type(),
    ) {
      return false;
    }
  }

  if lhs.generic_packs.len() != rhs.generic_packs.len() {
    return false;
  }

  for (&l, &r) in lhs.generic_packs.iter().zip(&rhs.generic_packs) {
    if !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
      seen,
      l.as_pack(),
      r.as_pack(),
    ) {
      return false;
    }
  }

  if !are_equal_optional_pack(seen, lhs.arg_types, rhs.arg_types) {
    return false;
  }

  if !are_equal_optional_pack(seen, lhs.ret_types, rhs.ret_types) {
    return false;
  }

  true
}

pub fn are_equal_are_equal_state_type_function_extern_type_type_function_extern_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionExternType,
  rhs: &TypeFunctionExternType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  lhs.extern_ty == rhs.extern_ty
}

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
    _ra = Some(RecursionLimiter::new(
      "areEqual",
      &mut seen.recursion_count,
      100,
    ));
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

pub fn are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTypePack,
  rhs: &TypeFunctionTypePack,
) -> bool {
  if lhs.head.len() != rhs.head.len() {
    return false;
  }

  // head 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.head.iter().zip(&rhs.head) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      l.as_type(),
      r.as_type(),
    ) {
      return false;
    }
  }

  true
}

pub fn are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionVariadicTypePack,
  rhs: &TypeFunctionVariadicTypePack,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  are_equal_are_equal_state_type_function_type_type_function_type(
    seen,
    lhs.type_id.as_type(),
    rhs.type_id.as_type(),
  )
}

/// C++ `bool areEqual(AreEqualState& seen, const TypeFunctionTypePackVar& lhs,
/// const TypeFunctionTypePackVar& rhs)` — the pack-variant dispatcher.
pub fn are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTypePackVar,
  rhs: &TypeFunctionTypePackVar,
) -> bool {
  {
    let lb = TypeFunctionTypePack::get_if(&lhs.type_variant);
    let rb = TypeFunctionTypePack::get_if(&rhs.type_variant);
    if let (Some(lb), Some(rb)) = (lb, rb) {
      return are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
        seen, lb, rb,
      );
    }
  }

  {
    let lv = TypeFunctionVariadicTypePack::get_if(&lhs.type_variant);
    let rv = TypeFunctionVariadicTypePack::get_if(&rhs.type_variant);
    if let (Some(lv), Some(rv)) = (lv, rv) {
      return are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack(
        seen, lv, rv,
      );
    }
  }

  {
    let lg = TypeFunctionGenericTypePack::get_if(&lhs.type_variant);
    let rg = TypeFunctionGenericTypePack::get_if(&rhs.type_variant);
    if let (Some(lg), Some(rg)) = (lg, rg) {
      return lg.is_named == rg.is_named && lg.name == rg.name;
    }
  }

  false
}
