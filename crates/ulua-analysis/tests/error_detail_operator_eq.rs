//! 错误详情结构体（CannotAssignToNever, MissingUnionProperty, IncorrectGenericParameterCount）
//! 的派生 `PartialEq` 对等性测试。
//!
//! 注意：cpp 比较的是 `*TypeId` 指向的 `Type`（深比较），移植版比较的是
//! `TypeId` 地址本身（`Vec<TypeId>` 的 `PartialEq`），因此这里用「按整数造的
//! 假想地址」当类型标识，只钉字段集合是否齐全，不依赖任何真实 `Type` 对象。

use ulua_analysis::{
  enums::reason::Reason,
  records::{
    cannot_assign_to_never::CannotAssignToNever, generic_type_definition::GenericTypeDefinition,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    missing_union_property::MissingUnionProperty, r#type::Type, type_fun::TypeFun,
  },
  type_aliases::type_id::TypeId,
};

/// `TypeId` 是 `*const Type`，测试用假想地址充当不同的类型标识（永不解引用）。
fn ty(v: u8) -> TypeId {
  v as *const Type
}

// CannotAssignToNever 测试

#[test]
fn cannot_assign_to_never_equal_values() {
  assert_eq!(
    CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed),
    CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed),
  );
}

#[test]
fn cannot_assign_to_never_cause_difference_not_equal() {
  let base = CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed);
  // cpp：先比 cause.size()，再逐个比元素
  assert_ne!(
    base,
    CannotAssignToNever::new(ty(1), vec![ty(3)], Reason::PropertyNarrowed),
  );
  assert_ne!(
    base,
    CannotAssignToNever::new(ty(1), vec![], Reason::PropertyNarrowed),
  );
  assert_ne!(
    base,
    CannotAssignToNever::new(ty(1), vec![ty(2), ty(3)], Reason::PropertyNarrowed),
  );
}

#[test]
fn cannot_assign_to_never_rhs_type_difference_not_equal() {
  let base = CannotAssignToNever::new(ty(1), vec![ty(2)], Reason::PropertyNarrowed);
  assert_ne!(
    base,
    CannotAssignToNever::new(ty(4), vec![ty(2)], Reason::PropertyNarrowed),
  );
}

// MissingUnionProperty 测试

fn prop(ty_id: TypeId, missing: Vec<TypeId>, key: &str) -> MissingUnionProperty {
  MissingUnionProperty::new(ty_id, missing, key)
}

#[test]
fn missing_union_property_equal_props() {
  assert_eq!(
    prop(ty(1), vec![ty(2), ty(3)], "x"),
    prop(ty(1), vec![ty(2), ty(3)], "x"),
  );
}

#[test]
fn missing_union_property_different_missing_element_not_equal() {
  assert_ne!(
    prop(ty(1), vec![ty(2), ty(3)], "x"),
    prop(ty(1), vec![ty(2), ty(4)], "x"),
  );
}

#[test]
fn missing_union_property_different_missing_length_not_equal() {
  assert_ne!(
    prop(ty(1), vec![ty(2)], "x"),
    prop(ty(1), vec![ty(2), ty(3)], "x"),
  );
}

#[test]
fn missing_union_property_different_type_or_key_not_equal() {
  assert_ne!(prop(ty(1), vec![ty(2)], "x"), prop(ty(5), vec![ty(2)], "x"),);
  assert_ne!(prop(ty(1), vec![ty(2)], "x"), prop(ty(1), vec![ty(2)], "y"),);
}

// IncorrectGenericParameterCount 测试

fn type_fun(r#type: TypeId, params: &[u8]) -> TypeFun {
  TypeFun::new(
    params
      .iter()
      .map(|&p| GenericTypeDefinition::new(ty(p), None))
      .collect(),
    vec![],
    r#type,
    None,
  )
}

fn err(name: &str, r#type: TypeId, params: &[u8]) -> IncorrectGenericParameterCount {
  IncorrectGenericParameterCount::new(name, type_fun(r#type, params), params.len(), 0)
}

#[test]
fn incorrect_generic_parameter_count_equal_values() {
  assert_eq!(err("T", ty(1), &[2, 3]), err("T", ty(1), &[2, 3]),);
}

#[test]
fn incorrect_generic_parameter_count_different_param_ty_not_equal() {
  assert_ne!(err("T", ty(1), &[2, 3]), err("T", ty(1), &[2, 4]),);
}

#[test]
fn incorrect_generic_parameter_count_different_param_count_not_equal() {
  assert_ne!(err("T", ty(1), &[2]), err("T", ty(1), &[2, 3]),);
}

#[test]
fn incorrect_generic_parameter_count_different_name_or_type_not_equal() {
  assert_ne!(err("T", ty(1), &[2]), err("U", ty(1), &[2]),);
  assert_ne!(err("T", ty(1), &[2]), err("T", ty(5), &[2]),);
}
