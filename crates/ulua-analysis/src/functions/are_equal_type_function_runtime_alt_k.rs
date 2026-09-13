use core::ffi::c_void;

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
    are_equal_type_function_runtime_alt_p::are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var,
    seen_set_contains::seen_set_contains,
  },
  records::{
    are_equal_state::AreEqualState, type_function_function_type::TypeFunctionFunctionType,
  },
};
pub fn are_equal_are_equal_state_type_function_function_type_type_function_function_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionFunctionType,
  rhs: &TypeFunctionFunctionType,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionFunctionType as *const c_void,
    rhs as *const TypeFunctionFunctionType as *const c_void,
  ) {
    return true;
  }

  if lhs.generics.len() != rhs.generics.len() {
    return false;
  }

  // generics 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.generics.iter().zip(&rhs.generics) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      unsafe { &*l },
      unsafe { &*r },
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
      unsafe { &*l },
      unsafe { &*r },
    ) {
      return false;
    }
  }

  if lhs.arg_types.is_null() != rhs.arg_types.is_null() {
    return false;
  }

  if !lhs.arg_types.is_null()
    && !rhs.arg_types.is_null()
    && !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
      seen,
      unsafe { &*lhs.arg_types },
      unsafe { &*rhs.arg_types },
    )
  {
    return false;
  }

  if lhs.ret_types.is_null() != rhs.ret_types.is_null() {
    return false;
  }

  if !lhs.ret_types.is_null()
    && !rhs.ret_types.is_null()
    && !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
      seen,
      unsafe { &*lhs.ret_types },
      unsafe { &*rhs.ret_types },
    )
  {
    return false;
  }

  true
}

#[cfg(test)]
mod tests {
  // areEqual 语义对齐 C++ TypeFunctionRuntime.cpp 的
  // `areEqual(AreEqualState&, const TypeFunctionFunctionType&, ...)`：
  // generics 与 generics 对比、genericPacks 与 genericPacks 对比。
  use alloc::{vec, vec::Vec};
  use core::ptr::null;

  use super::are_equal_are_equal_state_type_function_function_type_type_function_function_type;
  use crate::{
    records::{
      are_equal_state::AreEqualState,
      type_function_boolean_singleton::TypeFunctionBooleanSingleton,
      type_function_function_type::TypeFunctionFunctionType,
      type_function_singleton_type::TypeFunctionSingletonType,
      type_function_type::TypeFunctionType,
    },
    type_aliases::{
      type_function_singleton_variant::TypeFunctionSingletonVariant,
      type_function_type_id::TypeFunctionTypeId,
      type_function_type_pack_id::TypeFunctionTypePackId,
      type_function_type_variant::TypeFunctionTypeVariant,
    },
  };

  /// bool 单例类型；Box 泄漏保持指针稳定（测试用）
  fn bool_singleton(b: bool) -> TypeFunctionTypeId {
    Box::into_raw(Box::new(TypeFunctionType {
      type_variant: TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
        variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value: b }),
      }),
      frozen: false,
    }))
  }

  /// 指定泛型/泛型包、arg/ret 为空的函数类型
  fn fn_type(
    generics: Vec<TypeFunctionTypeId>,
    packs: Vec<TypeFunctionTypePackId>,
  ) -> Box<TypeFunctionFunctionType> {
    Box::new(TypeFunctionFunctionType {
      generics,
      generic_packs: packs,
      arg_types: null(),
      ret_types: null(),
      arg_names: Vec::new(),
    })
  }

  #[test]
  fn same_generics_equal() {
    let f1 = fn_type(vec![bool_singleton(true)], vec![]);
    let f2 = fn_type(vec![bool_singleton(true)], vec![]);
    let mut seen = AreEqualState::default();
    assert!(
      are_equal_are_equal_state_type_function_function_type_type_function_function_type(
        &mut seen, &f1, &f2
      )
    );
  }

  #[test]
  fn different_singleton_values_not_equal() {
    let f1 = fn_type(vec![bool_singleton(true)], vec![]);
    let f2 = fn_type(vec![bool_singleton(false)], vec![]);
    let mut seen = AreEqualState::default();
    assert!(
      !are_equal_are_equal_state_type_function_function_type_type_function_function_type(
        &mut seen, &f1, &f2
      )
    );
  }

  #[test]
  fn generic_count_mismatch_not_equal() {
    let f1 = fn_type(vec![bool_singleton(true)], vec![]);
    let f2 = fn_type(vec![bool_singleton(true), bool_singleton(false)], vec![]);
    let mut seen = AreEqualState::default();
    assert!(
      !are_equal_are_equal_state_type_function_function_type_type_function_function_type(
        &mut seen, &f1, &f2
      )
    );
  }

  #[test]
  fn generic_pack_count_mismatch_not_equal() {
    // generics 相同、genericPacks 数不同 → false（数量检查先于解引用）
    let f1 = fn_type(vec![bool_singleton(true)], vec![]);
    let f2 = fn_type(vec![bool_singleton(true)], vec![null()]);
    let mut seen = AreEqualState::default();
    assert!(
      !are_equal_are_equal_state_type_function_function_type_type_function_function_type(
        &mut seen, &f1, &f2
      )
    );
  }

  #[test]
  fn variant_index_matches_cpp_variant_positions() {
    // TypeFunctionRuntime.h 的 Variant<...> 成员次序：Function = 8、Singleton = 4
    assert_eq!(
      TypeFunctionTypeVariant::index(&TypeFunctionTypeVariant::Singleton(
        TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value: true }),
        }
      )),
      4
    );
    let f = *fn_type(vec![], vec![]);
    assert_eq!(
      TypeFunctionTypeVariant::index(&TypeFunctionTypeVariant::Function(f)),
      8
    );
  }
}
