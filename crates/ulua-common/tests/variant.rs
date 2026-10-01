//! `Variant1..Variant7` 对 cpp `Common/include/Luau/Variant.h` 的行为对齐，
//! 测试原型为 `cpp/tests/Variant.test.cpp`
//! （DefaultCtor / Create / Emplace / NonPOD copy / Equality / Visit）。

use std::string::{String, ToString};

use ulua_common::records::variant::{Variant1, Variant2, Variant3};

#[test]
fn variant_behavior() {
  // DefaultCtor: first alternative, default value.
  let v: Variant2<i32, String> = Variant2::default();
  assert_eq!(v.get_if_0(), Some(&0));
  assert!(v.get_if_1().is_none());
  assert_eq!(v.index(), 0);
  assert!(!v.valueless_by_exception());

  // Create + get_if by position.
  let v1: Variant2<i32, String> = Variant2::V1("hi".to_string());
  assert_eq!(v1.get_if_1().map(String::as_str), Some("hi"));
  assert_eq!(v1.index(), 1);

  // Emplace == reassign; NonPOD copy via Clone.
  let mut m: Variant2<i32, String> = Variant2::V0(5);
  assert_eq!(m.index(), 0);
  m = Variant2::V1("x".to_string());
  let mc = m.clone();
  assert_eq!(m, mc);

  // Equality: same variant+value; default == V0(0).
  let a: Variant2<i32, String> = Variant2::V0(0);
  assert_eq!(a, Variant2::<i32, String>::default());
  assert_ne!(v1, Variant2::V1("me".to_string()));
  assert_ne!(v1, Variant2::V0(1));

  // Visit -> match (arity 3).
  let t: Variant3<i32, bool, String> = Variant3::V2("z".to_string());
  let rendered = match &t {
    Variant3::V0(n) => n.to_string(),
    Variant3::V1(b) => b.to_string(),
    Variant3::V2(s) => s.clone(),
  };
  assert_eq!(rendered, "z");
}

/// 按类型查找的两个方法（cpp `Variant.h:23-36` `getTypeId<T>`、`:119-136`
/// `get_if<T>`）——本 crate 变体的 `get_type_id`/`get_if` 由 7 元数共用宏生成。
/// cpp 对"不在选项内的 T"是编译期 `static_assert(tid >= 0)`，Rust 无稳定特化，
/// 落成 `get_type_id == -1` 与 `get_if == None`（见 `records/variant.rs` 的
/// DELIBERATE DEVIATION 锚）。
#[test]
fn type_based_lookup_matches_variant_header() {
  type V = Variant3<i32, bool, String>;

  // getTypeId：返回值即选项声明序的下标。
  assert_eq!(V::get_type_id::<i32>(), 0);
  assert_eq!(V::get_type_id::<bool>(), 1);
  assert_eq!(V::get_type_id::<String>(), 2);
  assert_eq!(
    V::get_type_id::<f64>(),
    -1,
    "非选项类型必须报 -1（cpp 为编译错误）"
  );

  // get_if：仅当活跃选项即该类型时命中。
  let active: V = V::V1(true);
  assert_eq!(active.get_if::<bool>(), Some(&true));
  assert_eq!(active.get_if::<i32>(), None, "类型在选项内但非活跃 -> None");
  assert_eq!(active.get_if::<f64>(), None);

  let text: V = V::V2("z".to_string());
  assert_eq!(text.get_if::<String>().map(String::as_str), Some("z"));
  assert_eq!(text.get_if::<bool>(), None);

  // 重复类型选项：cpp `getTypeId` 的循环首个命中胜出（`Variant.h:28-32`），Rust 同序。
  assert_eq!(Variant2::<i32, i32>::get_type_id::<i32>(), 0);

  // 一元变体：唯一选项即下标 0。
  assert_eq!(Variant1::<i32>::get_type_id::<i32>(), 0);
  assert_eq!(Variant1::<i32>::get_type_id::<bool>(), -1);
  assert_eq!(Variant1::V0(9).get_if::<i32>(), Some(&9));
}
