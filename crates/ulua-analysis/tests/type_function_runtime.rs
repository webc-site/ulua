//! areEqual 语义对齐 C++ TypeFunctionRuntime.cpp 的
//! `areEqual(AreEqualState&, const TypeFunctionFunctionType&, ...)`：
//! generics 与 generics 对比、genericPacks 与 genericPacks 对比。

use core::ptr::null;

use ulua_analysis::{
  functions::are_equal_type_function_runtime::are_equal_are_equal_state_type_function_function_type_type_function_function_type,
  records::{
    are_equal_state::AreEqualState, type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_singleton_type::TypeFunctionSingletonType, type_function_type::TypeFunctionType,
  },
  type_aliases::{
    type_function_singleton_variant::TypeFunctionSingletonVariant,
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};

/// bool 单例类型；Box 泄漏保持指针稳定（测试用）
fn bool_singleton(b: bool) -> TypeFunctionTypeId {
  Box::into_raw(Box::new(TypeFunctionType::new(
    TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
      variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value: b }),
    }),
  )))
}

/// 指定泛型/泛型包、arg/ret 为空的函数类型
fn fn_type(
  generics: Vec<TypeFunctionTypeId>,
  packs: Vec<TypeFunctionTypePackId>,
) -> Box<TypeFunctionFunctionType> {
  Box::new(TypeFunctionFunctionType::new(
    generics,
    packs,
    null(),
    null(),
    Vec::new(),
  ))
}

/// 用同一份 `AreEqualState` 跑一次比较（seen 集每次新建，避免缓存串味）
fn are_equal(lhs: &TypeFunctionFunctionType, rhs: &TypeFunctionFunctionType) -> bool {
  let mut seen = AreEqualState::default();
  are_equal_are_equal_state_type_function_function_type_type_function_function_type(
    &mut seen, lhs, rhs,
  )
}

#[test]
fn same_generics_equal() {
  let f1 = fn_type(vec![bool_singleton(true)], vec![]);
  let f2 = fn_type(vec![bool_singleton(true)], vec![]);
  assert!(are_equal(&f1, &f2));
}

#[test]
fn different_singleton_values_not_equal() {
  let f1 = fn_type(vec![bool_singleton(true)], vec![]);
  let f2 = fn_type(vec![bool_singleton(false)], vec![]);
  assert!(!are_equal(&f1, &f2));
}

#[test]
fn generic_count_mismatch_not_equal() {
  let f1 = fn_type(vec![bool_singleton(true)], vec![]);
  let f2 = fn_type(vec![bool_singleton(true), bool_singleton(false)], vec![]);
  assert!(!are_equal(&f1, &f2));
}

#[test]
fn generic_pack_count_mismatch_not_equal() {
  // generics 相同、genericPacks 数不同 → false（数量检查先于解引用）
  let f1 = fn_type(vec![bool_singleton(true)], vec![]);
  let f2 = fn_type(vec![bool_singleton(true)], vec![null()]);
  assert!(!are_equal(&f1, &f2));
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
