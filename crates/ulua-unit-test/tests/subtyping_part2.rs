//! Subtyping.test.cpp 的补充移植批次（与 `subtyping.rs` 同一 cpp 源文件）。
//!
//! `subtyping.rs` 只覆盖到 cpp 161 个 `TEST_CASE_FIXTURE` + 75 个由
//! `TEST_IS_SUBTYPE` / `TEST_IS_NOT_SUBTYPE` 宏自动展开的用例中的 22 个，
//! 本文件按 cpp 顺序补齐其余「结构清晰、只依赖既有 `SubtypeFixture` helper」
//! 的用例；依赖缺失 API（reasoning 缓存、`uniqueTypes`、TypePath 遍历等）的
//! 用例见 `subtyping_part3.rs` 顶部清单与任务报告。
//!
//! 断言方向严格照抄 cpp，不做弱化：`check_subtype!` / `check_not_subtype!`
//! 分别对应 `CHECK_IS_SUBTYPE` / `CHECK_IS_NOT_SUBTYPE`。

extern crate alloc;

use alloc::{vec, vec::Vec};

use ulua_analysis::{
  functions::to_string_to_string::to_string_type_id,
  records::{
    function_type::FunctionType, property_type::Property, table_indexer::TableIndexer,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_variant::TypePackVariant},
};
use ulua_unit_test::{
  functions::add_sealed_table_type::add_sealed_table_type, records::subtype_fixture::SubtypeFixture,
};

/// cpp `CHECK_IS_SUBTYPE(left, right)`。
macro_rules! check_subtype {
  ($fixture:ident, $sub:expr, $super_ty:expr) => {{
    let (sub_ty, super_ty) = ($sub, $super_ty);
    assert_subtype(&mut $fixture, sub_ty, super_ty);
  }};
}

/// cpp `CHECK_IS_NOT_SUBTYPE(left, right)`。
macro_rules! check_not_subtype {
  ($fixture:ident, $sub:expr, $super_ty:expr) => {{
    let (sub_ty, super_ty) = ($sub, $super_ty);
    assert_not_subtype(&mut $fixture, sub_ty, super_ty);
  }};
}

fn assert_subtype(fixture: &mut SubtypeFixture, sub_ty: TypeId, super_ty: TypeId) {
  let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
  assert!(
    result.is_subtype(),
    "expected {} <: {}, got {result:?}",
    to_string_type_id(sub_ty),
    to_string_type_id(super_ty)
  );
}

fn assert_not_subtype(fixture: &mut SubtypeFixture, sub_ty: TypeId, super_ty: TypeId) {
  let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
  assert!(
    !result.is_subtype(),
    "expected {} </: {}, got {result:?}",
    to_string_type_id(sub_ty),
    to_string_type_id(super_ty)
  );
}

/// cpp `SubtypeFixture::fn(args, rets)`。
fn fn_ty(fixture: &mut SubtypeFixture, args: &[TypeId], rets: &[TypeId]) -> TypeId {
  fixture.fn_item_initializer_list_type_id_initializer_list_type_id(args.to_vec(), rets.to_vec())
}

/// cpp `SubtypeFixture::fn(argHead, argTail, rets)`（参数表带 variadic 尾）。
fn fn_ty_tail(
  fixture: &mut SubtypeFixture,
  args: &[TypeId],
  tail: TypePackVariant,
  rets: &[TypeId],
) -> TypeId {
  fixture.fn_item_initializer_list_type_id_type_pack_variant_initializer_list_type_id(
    args.to_vec(),
    tail,
    rets.to_vec(),
  )
}

/// cpp `(...t) -> r`：仅由 variadic 尾构成的参数表。
fn variadic_fn(fixture: &mut SubtypeFixture, elem: TypeId, ret: TypeId) -> TypeId {
  fn_ty_tail(
    fixture,
    &[],
    TypePackVariant::Variadic(VariadicTypePack::new(elem)),
    &[ret],
  )
}

/// cpp `SubtypeFixture::tbl({{name, ty}, ...})`：属性全部读写（rw）。
fn rw_tbl(fixture: &mut SubtypeFixture, props: &[(&str, TypeId)]) -> TypeId {
  let owned: Vec<(&str, Property)> = props
    .iter()
    .map(|&(name, ty)| (name, Property::rw_type_id(ty)))
    .collect();
  fixture.tbl(SubtypeFixture::props(owned))
}

/// 带 read/write 修饰符的表：cpp `tbl({{"x", Property::readonly(..)}})`。
fn prop_tbl(fixture: &mut SubtypeFixture, props: &[(&str, Property)]) -> TypeId {
  fixture.tbl(SubtypeFixture::props(props.to_vec()))
}

/// cpp `SubtypeFixture::idx(keyTy, valueTy, isReadOnly)`；`idx` helper 未暴露
/// `isReadOnly`，这里按 cpp `TableType{ {}, TableIndexer{..}, Sealed }` 直接构造。
fn idx_read_only(fixture: &mut SubtypeFixture, key: TypeId, value: TypeId) -> TypeId {
  let indexer = TableIndexer {
    index_type: key,
    index_result_type: value,
    is_read_only: true,
  };
  add_sealed_table_type(&mut fixture.arena, &Default::default(), Some(indexer))
}

/// cpp `SubtypeFixture::boolSingleton(true/false)` 等等价：直接取内置单例。
fn builtin(fixture: &mut SubtypeFixture) -> BuiltinRefs {
  BuiltinRefs {
    any: fixture.builtin_types.any_type,
    unknown: fixture.builtin_types.unknown_type,
    number: fixture.builtin_types.number_type,
    optional_number: fixture.builtin_types.optional_number_type,
    string: fixture.builtin_types.string_type,
    optional_string: fixture.builtin_types.optional_string_type,
    boolean: fixture.builtin_types.boolean_type,
    boolean_true: fixture.builtin_types.true_type,
    boolean_false: fixture.builtin_types.false_type,
    table: fixture.builtin_types.table_type,
    function: fixture.builtin_types.function_type,
    thread: fixture.builtin_types.thread_type,
    buffer: fixture.builtin_types.buffer_type,
    never: fixture.builtin_types.never_type,
    extern_ty: fixture.builtin_types.extern_type,
    object: fixture.builtin_types.object_type,
    class: fixture.builtin_types.class_type,
  }
}

/// 一次性的内置类型快照，避免每个用例重复 `fixture.builtin_types.x` 借用。
#[derive(Clone, Copy)]
struct BuiltinRefs {
  any: TypeId,
  unknown: TypeId,
  number: TypeId,
  optional_number: TypeId,
  string: TypeId,
  optional_string: TypeId,
  boolean: TypeId,
  boolean_true: TypeId,
  boolean_false: TypeId,
  table: TypeId,
  function: TypeId,
  thread: TypeId,
  buffer: TypeId,
  never: TypeId,
  extern_ty: TypeId,
  object: TypeId,
  class: TypeId,
}

/// 原语、单例、union/intersection 用例。
mod primitives {
  use super::*;

  /// cpp: `TEST_IS_SUBTYPE(numberType, anyType)` (L430)
  #[test]
  fn number_subtype_of_any() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_subtype!(fixture, b.number, b.any);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(numberType, stringType)` (L431)
  #[test]
  fn number_not_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_not_subtype!(fixture, b.number, b.string);
  }

  /// cpp: `"any <: unknown"` (L515) —— any 与 unknown  inhabitant 集合相同。
  #[test]
  fn any_subtype_of_unknown() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_subtype!(fixture, b.any, b.unknown);
  }

  /// cpp: `"number <: unknown"` (L527)
  #[test]
  fn number_subtype_of_unknown() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_subtype!(fixture, b.number, b.unknown);
  }

  /// cpp: `"number <: number?"` (L537)
  #[test]
  fn number_subtype_of_optional_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_subtype!(fixture, b.number, b.optional_number);
  }

  /// cpp: `"\"hello\" <: string"` (L542)
  #[test]
  fn string_singleton_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    check_subtype!(fixture, hello, b.string);
  }

  /// cpp: `"string <!: \"hello\""` (L547)
  #[test]
  fn string_not_subtype_of_string_singleton() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    check_not_subtype!(fixture, b.string, hello);
  }

  /// cpp: `"\"hello\" <: \"hello\""` (L552) —— 两个不同的单例 TypeId，字面量相同。
  #[test]
  fn equal_string_singletons_are_mutual_subtypes() {
    let mut fixture = SubtypeFixture::default();
    let hello = fixture.str("hello");
    let hello2 = fixture.str("hello");
    check_subtype!(fixture, hello, hello2);
    check_subtype!(fixture, hello2, hello);
  }

  /// cpp: `"true <: boolean"` (L557)
  #[test]
  fn true_subtype_of_boolean() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_subtype!(fixture, b.boolean_true, b.boolean);
  }

  /// cpp: `"true <: true | false"` (L562)
  #[test]
  fn true_subtype_of_boolean_union() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let true_or_false = fixture.join(b.boolean_true, b.boolean_false);
    check_subtype!(fixture, b.boolean_true, true_or_false);
  }

  /// cpp: `"true | false <!: true"` (L567)
  #[test]
  fn boolean_union_not_subtype_of_true() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let true_or_false = fixture.join(b.boolean_true, b.boolean_false);
    check_not_subtype!(fixture, true_or_false, b.boolean_true);
  }

  /// cpp: `"true | false <: boolean"` (L572)
  #[test]
  fn boolean_union_subtype_of_boolean() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let true_or_false = fixture.join(b.boolean_true, b.boolean_false);
    check_subtype!(fixture, true_or_false, b.boolean);
  }

  /// cpp: `"true | false <: true | false"` (L577)
  #[test]
  fn boolean_union_subtype_of_itself() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let true_or_false = fixture.join(b.boolean_true, b.boolean_false);
    check_subtype!(fixture, true_or_false, true_or_false);
  }

  /// cpp: `"\"hello\" | \"world\" <: number"` (L582, 断言为非子类型)
  #[test]
  fn string_singleton_union_not_subtype_of_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    let world = fixture.str("world");
    let hello_or_world = fixture.join(hello, world);
    check_not_subtype!(fixture, hello_or_world, b.number);
  }

  /// cpp: `"string <!: ('hello' | 'hello')"` (L587) —— 含重复选项的 union。
  #[test]
  fn string_not_subtype_of_duplicated_singleton_union() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    let hello_or_hello = fixture.arena.add_type(UnionType {
      options: vec![hello, hello],
    });
    check_not_subtype!(fixture, b.string, hello_or_hello);
  }

  /// cpp: `"true <: boolean & true"` (L592)
  #[test]
  fn true_subtype_of_boolean_and_true() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let boolean_and_true = fixture.meet(b.boolean, b.boolean_true);
    check_subtype!(fixture, b.boolean_true, boolean_and_true);
  }

  /// cpp: `"boolean & true <: true"` (L597)
  #[test]
  fn boolean_and_true_subtype_of_true() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let boolean_and_true = fixture.meet(b.boolean, b.boolean_true);
    check_subtype!(fixture, boolean_and_true, b.boolean_true);
  }

  /// cpp: `"boolean & true <: boolean & true"` (L602)
  #[test]
  fn boolean_and_true_subtype_of_itself() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let boolean_and_true = fixture.meet(b.boolean, b.boolean_true);
    check_subtype!(fixture, boolean_and_true, boolean_and_true);
  }

  /// cpp: `"\"hello\" & \"world\" <: number"` (L607) —— 不可居交集是 never。
  #[test]
  fn uninhabited_intersection_is_subtype_of_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    let world = fixture.str("world");
    let hello_and_world = fixture.meet(hello, world);
    check_subtype!(fixture, hello_and_world, b.number);
  }

  /// cpp: `"false <!: boolean & true"` (L612)
  #[test]
  fn false_not_subtype_of_boolean_and_true() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let boolean_and_true = fixture.meet(b.boolean, b.boolean_true);
    check_not_subtype!(fixture, b.boolean_false, boolean_and_true);
  }

  /// cpp: `"number <: ~~number"` (L1292)
  #[test]
  fn number_subtype_of_double_negated_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let negated = fixture.negate(b.number);
    let double_negated = fixture.negate(negated);
    check_subtype!(fixture, b.number, double_negated);
  }

  /// cpp: `"~~number <: number"` (L1297)
  #[test]
  fn double_negated_number_subtype_of_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let negated = fixture.negate(b.number);
    let double_negated = fixture.negate(negated);
    check_subtype!(fixture, double_negated, b.number);
  }

  /// cpp: `"semantic_subtyping_disj"` (L1092) —— unknown 与析取式否定。
  #[test]
  fn unknown_subtype_of_disjunction_of_negations() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let not_number = fixture.negate(b.number);
    let not_string = fixture.negate(b.string);
    let super_ty = fixture.join(not_number, not_string);
    check_subtype!(fixture, b.unknown, super_ty);
  }
}

/// 函数类型（协变返回、反变参数、arity、variadic）。
mod functions {
  use super::*;

  /// cpp: `"(unknown) -> string <: (number) -> string"` (L617)
  #[test]
  fn unknown_to_string_subtype_of_number_to_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.unknown], &[b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.string]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number) -> string <!: (unknown) -> string"` (L622)
  #[test]
  fn number_to_string_not_subtype_of_unknown_to_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number], &[b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.unknown], &[b.string]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number, number) -> string <!: (number) -> string"` (L627)
  #[test]
  fn wider_arity_not_subtype_of_narrower_arity() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number, b.number], &[b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.string]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number) -> string <!: (number, number) -> string"` (L632)
  #[test]
  fn narrower_arity_not_subtype_of_wider_arity() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number], &[b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.number, b.number], &[b.string]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number, number) -> string <!: (unknown, number) -> string"` (L637)
  #[test]
  fn mismatched_second_argument_not_subtype() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number, b.number], &[b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.unknown, b.number], &[b.string]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(unknown, number) -> string <: (number, number) -> string"` (L642)
  #[test]
  fn contravariant_first_argument_is_subtype() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.unknown, b.number], &[b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.number, b.number], &[b.string]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number) -> (string, unknown) <!: (number) -> (string, string)"` (L647)
  #[test]
  fn wider_return_tuple_not_subtype() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number], &[b.string, b.unknown]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.string, b.string]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number) -> (string, string) <: (number) -> (string, unknown)"` (L652)
  #[test]
  fn narrower_return_tuple_is_subtype() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number], &[b.string, b.string]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.string, b.unknown]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number, ...string) -> string <: (number) -> string"` (L667)
  #[test]
  fn variadic_extra_arguments_are_supertype_of_fixed_arity() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty_tail(
      &mut fixture,
      &[b.number],
      TypePackVariant::Variadic(VariadicTypePack::new(b.string)),
      &[b.string],
    );
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.string]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number) -> string <!: (number, ...string) -> string"` (L672)
  #[test]
  fn fixed_arity_not_subtype_of_variadic_extra_arguments() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number], &[b.string]);
    let super_ty = fn_ty_tail(
      &mut fixture,
      &[b.number],
      TypePackVariant::Variadic(VariadicTypePack::new(b.string)),
      &[b.string],
    );
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number, ...string) -> string <!: (number, ...string?) -> string"` (L682)
  #[test]
  fn variadic_string_not_subtype_of_optional_variadic_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty_tail(
      &mut fixture,
      &[b.number],
      TypePackVariant::Variadic(VariadicTypePack::new(b.string)),
      &[b.string],
    );
    let super_ty = fn_ty_tail(
      &mut fixture,
      &[b.number],
      TypePackVariant::Variadic(VariadicTypePack::new(b.optional_string)),
      &[b.string],
    );
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number, ...string) -> string <: (number, string) -> string"` (L687)
  #[test]
  fn variadic_string_subtype_of_single_string_argument() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty_tail(
      &mut fixture,
      &[b.number],
      TypePackVariant::Variadic(VariadicTypePack::new(b.string)),
      &[b.string],
    );
    let super_ty = fn_ty(&mut fixture, &[b.number, b.string], &[b.string]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number, string) -> string <!: (number, ...string) -> string"` (L692)
  #[test]
  fn single_string_argument_not_subtype_of_variadic_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number, b.string], &[b.string]);
    let super_ty = fn_ty_tail(
      &mut fixture,
      &[b.number],
      TypePackVariant::Variadic(VariadicTypePack::new(b.string)),
      &[b.string],
    );
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(() -> number) -> () <: (<T>() -> T) -> ()"` (L1587)
  #[test]
  fn higher_order_argument_subtype_of_generic_identity() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let nothing_to_number = fn_ty(&mut fixture, &[], &[b.number]);
    let generic_nothing_to_t = fixture.generic_fn(vec![generic_t], vec![], vec![generic_t]);
    let sub = fn_ty(&mut fixture, &[nothing_to_number], &[]);
    let super_ty = fn_ty(&mut fixture, &[generic_nothing_to_t], &[]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"((number) -> ()) -> () <: (<T>(T) -> ()) -> ()"` (L1594)
  #[test]
  fn higher_order_number_argument_subtype_of_generic_argument() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let number_to_nothing = fn_ty(&mut fixture, &[b.number], &[]);
    let generic_t_to_nothing = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let sub = fn_ty(&mut fixture, &[number_to_nothing], &[]);
    let super_ty = fn_ty(&mut fixture, &[generic_t_to_nothing], &[]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"((number) -> number) -> () <: (<T>(T) -> T) -> ()"` (L1601)
  #[test]
  fn higher_order_identity_subtype_of_generic_identity() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let number_to_number = fn_ty(&mut fixture, &[b.number], &[b.number]);
    let generic_t_to_t = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![generic_t]);
    let sub = fn_ty(&mut fixture, &[number_to_number], &[]);
    let super_ty = fn_ty(&mut fixture, &[generic_t_to_t], &[]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(...any) -> () <: <T>(T...) -> ()"` (L1303, luau#767)
  #[test]
  fn any_variadic_pack_subtype_of_generic_pack_function() {
    let mut fixture = SubtypeFixture::default();
    let empty_pack = fixture.builtin_types.empty_type_pack;
    let any_pack = fixture.builtin_types.any_type_pack;
    let anys_to_nothing = fixture.arena.add_type(FunctionType::function_type_new(
      any_pack, empty_pack, None, false,
    ));
    let generic_as = fixture.generic_pack("A");
    let generic_pack_fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
      generic_as, empty_pack, None, false,
    ));
    check_subtype!(fixture, anys_to_nothing, generic_pack_fn_ty);
  }

  /// cpp: `"(~fun & (string) -> number) <: (string) -> number"` (L1259)
  #[test]
  fn negated_function_intersection_subtype_of_function() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let number_to_string = fn_ty(&mut fixture, &[b.number], &[b.string]);
    let not_function = fixture.negate(b.function);
    let sub = fixture.meet(not_function, number_to_string);
    check_subtype!(fixture, sub, number_to_string);
  }

  /// cpp: `"(string) -> number <!: ~fun & (string) -> number"` (L1264)
  #[test]
  fn function_not_subtype_of_negated_function_intersection() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let number_to_string = fn_ty(&mut fixture, &[b.number], &[b.string]);
    let not_function = fixture.negate(b.function);
    let super_ty = fixture.meet(not_function, number_to_string);
    check_not_subtype!(fixture, number_to_string, super_ty);
  }
}

/// 泛型函数与泛型 pack。
mod generics {
  use super::*;

  /// cpp: `"<T>(T) -> () <: <U>(U) -> ()"` (L719)
  #[test]
  fn alpha_renamed_generic_function_is_subtype() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let generic_u = fixture.generic("U");
    let sub = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let super_ty = fixture.generic_fn(vec![generic_u], vec![generic_u], vec![]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<T>(T) -> () <: (number) -> ()"` (L729)
  #[test]
  fn generic_function_instantiates_to_concrete_argument() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let sub = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<T>(T) -> T <: (number) -> number"` (L734)
  #[test]
  fn generic_identity_instantiates_to_number_identity() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let sub = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![generic_t]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.number]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<T>(T) -> T <!: (number) -> string"` (L739)
  #[test]
  fn generic_identity_not_subtype_of_number_to_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let sub = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![generic_t]);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.string]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<T>() -> (T, T) <!: () -> (string, number)"` (L749)
  #[test]
  fn single_generic_cannot_match_two_distinct_returns() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let sub = fixture.generic_fn(vec![generic_t], vec![], vec![generic_t, generic_t]);
    let super_ty = fn_ty(&mut fixture, &[], &[b.string, b.number]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<A...>(A...) -> A... <: (number) -> number"` (L758)
  #[test]
  fn generic_pack_function_instantiates_to_number_function() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_as = fixture.generic_pack("A");
    let sub = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    let super_ty = fn_ty(&mut fixture, &[b.number], &[b.number]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(number) -> number <!: <A...>(A...) -> A..."` (L763)
  #[test]
  fn concrete_function_not_subtype_of_generic_pack_function() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_as = fixture.generic_pack("A");
    let sub = fn_ty(&mut fixture, &[b.number], &[b.number]);
    let super_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<A...>(A...) -> A... <: <B...>(B...) -> B..."` (L768)
  #[test]
  fn alpha_renamed_generic_pack_function_is_subtype() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let generic_bs = fixture.generic_pack("B");
    let sub = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    let super_ty = fixture.generic_pack_fn(vec![generic_bs], generic_bs, generic_bs);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<B..., C...>(B...) -> C... <: <A...>(A...) -> A..."` (L773)
  #[test]
  fn split_generic_pack_subtype_of_single_generic_pack() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let generic_bs = fixture.generic_pack("B");
    let generic_cs = fixture.generic_pack("C");
    let sub = fixture.generic_pack_fn(vec![generic_bs, generic_cs], generic_bs, generic_cs);
    let super_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<A...>(A...) -> A... <!: <B..., C...>(B...) -> C..."` (L778)
  #[test]
  fn single_generic_pack_not_subtype_of_split_generic_pack() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let generic_bs = fixture.generic_pack("B");
    let generic_cs = fixture.generic_pack("C");
    let sub = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    let super_ty = fixture.generic_pack_fn(vec![generic_bs, generic_cs], generic_bs, generic_cs);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<A...>(A...) -> number <: (...number) -> number"` (L793)
  #[test]
  fn generic_pack_arguments_subtype_of_variadic_number_arguments() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_as = fixture.generic_pack("A");
    let number_pack = fixture.pack_initializer_list_type_id(vec![b.number]);
    let sub = fixture.generic_pack_fn(vec![generic_as], generic_as, number_pack);
    let super_ty = variadic_fn(&mut fixture, b.number, b.number);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"(...number) -> number <!: <A...>(A...) -> number"` (L798)
  #[test]
  fn variadic_number_arguments_not_subtype_of_generic_pack_arguments() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_as = fixture.generic_pack("A");
    let number_pack = fixture.pack_initializer_list_type_id(vec![b.number]);
    let sub = variadic_fn(&mut fixture, b.number, b.number);
    let super_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, number_pack);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"() -> () <!: <A...>() -> A..."` (L808)
  #[test]
  fn nothing_to_nothing_not_subtype_of_generic_nothing_to_pack() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let empty_pack = fixture.builtin_types.empty_type_pack;
    let sub = fn_ty(&mut fixture, &[], &[]);
    let super_ty = fixture.generic_pack_fn(vec![generic_as], empty_pack, generic_as);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"() -> () <!: <A...>(A...) -> A..."` (L818)
  #[test]
  fn nothing_to_nothing_not_subtype_of_generic_pack_to_pack() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let sub = fn_ty(&mut fixture, &[], &[]);
    let super_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<A...>(A...) -> (<A...>(A...) -> ()) <: (string) -> ((number) -> ())"` (L1632)
  #[test]
  fn generic_pack_returning_generic_pack_function() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_as = fixture.generic_pack("A");
    let empty_pack = fixture.builtin_types.empty_type_pack;
    let as_to_nothing = fixture.generic_pack_fn(vec![generic_as], generic_as, empty_pack);
    let ret_pack = fixture.pack_initializer_list_type_id(vec![as_to_nothing]);
    let sub = fixture.generic_pack_fn(vec![generic_as], generic_as, ret_pack);
    let number_to_nothing = fn_ty(&mut fixture, &[b.number], &[]);
    let super_ty = fn_ty(&mut fixture, &[b.string], &[number_to_nothing]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"<T>({ x: T }) -> T <: ({ method: <T>({ x: T }) -> T, x: number }) -> number"` (L1481)
  #[test]
  fn generic_table_reader_subtype_of_method_table_application() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let generic_t = fixture.generic("T");
    let x_of_t = rw_tbl(&mut fixture, &[("x", generic_t)]);
    let table_to_prop = fixture.generic_fn(vec![generic_t], vec![x_of_t], vec![generic_t]);
    let method_table = rw_tbl(&mut fixture, &[("method", table_to_prop), ("x", b.number)]);
    let super_ty = fn_ty(&mut fixture, &[method_table], &[b.number]);
    check_subtype!(fixture, table_to_prop, super_ty);
  }

  /// cpp: `"(...unknown) -> () <: <T>(T...) -> ()"` (L1312, luau#767)
  #[test]
  fn unknown_variadic_pack_subtype_of_generic_pack_function() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let empty_pack = fixture.builtin_types.empty_type_pack;
    let unknowns_to_nothing = fn_ty_tail(
      &mut fixture,
      &[],
      TypePackVariant::Variadic(VariadicTypePack::new(b.unknown)),
      &[],
    );
    let generic_as = fixture.generic_pack("A");
    let generic_pack_fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
      generic_as, empty_pack, None, false,
    ));
    check_subtype!(fixture, unknowns_to_nothing, generic_pack_fn_ty);
  }
}

/// 密封表、属性修饰符（read/write）与宽度子类型。
mod tables {
  use super::*;

  /// cpp: `"{} <: {}"` (L823)
  #[test]
  fn empty_table_subtype_of_empty_table() {
    let mut fixture = SubtypeFixture::default();
    let sub = rw_tbl(&mut fixture, &[]);
    let super_ty = rw_tbl(&mut fixture, &[]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{} <!: {x: number}"` (L833)
  #[test]
  fn empty_table_not_subtype_of_table_with_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[]);
    let super_ty = rw_tbl(&mut fixture, &[("x", b.number)]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{x: number} <!: {x: string}"` (L838)
  #[test]
  fn property_type_mismatch_breaks_subtyping() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.number)]);
    let super_ty = rw_tbl(&mut fixture, &[("x", b.string)]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{x: number?} <!: {x: number}"` (L848)
  #[test]
  fn optional_property_not_subtype_of_required_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.optional_number)]);
    let super_ty = rw_tbl(&mut fixture, &[("x", b.number)]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{x: <T>(T) -> ()} <: {x: <U>(U) -> ()}"` (L853)
  #[test]
  fn table_of_alpha_renamed_generic_functions_is_subtype() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let generic_u = fixture.generic("U");
    let t_to_nothing = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let u_to_nothing = fixture.generic_fn(vec![generic_u], vec![generic_u], vec![]);
    let sub = rw_tbl(&mut fixture, &[("x", t_to_nothing)]);
    let super_ty = rw_tbl(&mut fixture, &[("x", u_to_nothing)]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ x: number } <: { read x: number }"` (L858)
  #[test]
  fn read_write_property_subtype_of_read_only_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.number)]);
    let super_ty = prop_tbl(&mut fixture, &[("x", Property::readonly(b.number))]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ x: number } <: { write x: number }"` (L865)
  #[test]
  fn read_write_property_subtype_of_write_only_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.number)]);
    let super_ty = prop_tbl(&mut fixture, &[("x", Property::writeonly(b.number))]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `{ x: "hello" } <: { read x: string }` (L872)
  #[test]
  fn singleton_property_subtype_of_read_only_widened_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    let sub = rw_tbl(&mut fixture, &[("x", hello)]);
    let super_ty = prop_tbl(&mut fixture, &[("x", Property::readonly(b.string))]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ x: string } <: { write x: string }"` (L879)
  #[test]
  fn string_property_subtype_of_write_only_string_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.string)]);
    let super_ty = prop_tbl(&mut fixture, &[("x", Property::writeonly(b.string))]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(tableType, tbl({}))` (L916)
  #[test]
  fn table_subtype_of_empty_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let empty = rw_tbl(&mut fixture, &[]);
    check_subtype!(fixture, b.table, empty);
  }

  /// cpp: `TEST_IS_SUBTYPE(tbl({}), tableType)` (L917)
  #[test]
  fn empty_table_subtype_of_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let empty = rw_tbl(&mut fixture, &[]);
    check_subtype!(fixture, empty, b.table);
  }

  /// cpp: `"{x: number, y: number} <: {x: number}"` (L1437)
  #[test]
  fn wider_table_subtype_of_narrower_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.number), ("y", b.number)]);
    let super_ty = rw_tbl(&mut fixture, &[("x", b.number)]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{x: number} <!: {x: number, y: number}"` (L1438)
  #[test]
  fn narrower_table_not_subtype_of_wider_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("x", b.number)]);
    let super_ty = rw_tbl(&mut fixture, &[("x", b.number), ("y", b.number)]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `table_test` 的 `bill` 变体 (L1321)：结构相同、属性+indexer 的表互为子类型。
  #[test]
  fn structurally_identical_indexed_tables_are_mutual_subtypes() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let build = |fixture: &mut SubtypeFixture| {
      fixture.tbl_with_indexer(
        SubtypeFixture::props(vec![("a", Property::rw_type_id(b.string))]),
        b.string,
        b.number,
      )
    };
    let a = build(&mut fixture);
    let b_ty = build(&mut fixture);
    check_subtype!(fixture, a, b_ty);
    check_subtype!(fixture, b_ty, a);
  }
}

/// 元表类型。
mod metatables {
  use super::*;

  /// cpp: `"{ @metatable { x: number } } <: { @metatable {} }"` (L886)
  #[test]
  fn metatable_widening_is_subtype() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.meta(
      SubtypeFixture::props(vec![("x", Property::rw_type_id(b.number))]),
      SubtypeFixture::props(vec![]),
    );
    let super_ty = fixture.meta(SubtypeFixture::props(vec![]), SubtypeFixture::props(vec![]));
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ @metatable { x: number } } <!: { @metatable { x: boolean } }"` (L891)
  #[test]
  fn metatable_property_type_mismatch_breaks_subtyping() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.meta(
      SubtypeFixture::props(vec![("x", Property::rw_type_id(b.number))]),
      SubtypeFixture::props(vec![]),
    );
    let super_ty = fixture.meta(
      SubtypeFixture::props(vec![("x", Property::rw_type_id(b.boolean))]),
      SubtypeFixture::props(vec![]),
    );
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ @metatable {} } <!: { @metatable { x: boolean } }"` (L896)
  #[test]
  fn empty_metatable_not_subtype_of_populated_metatable() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.meta(SubtypeFixture::props(vec![]), SubtypeFixture::props(vec![]));
    let super_ty = fixture.meta(
      SubtypeFixture::props(vec![("x", Property::rw_type_id(b.boolean))]),
      SubtypeFixture::props(vec![]),
    );
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ @metatable { u: boolean }, x: number } <: { x: number }"` (L906)
  #[test]
  fn metatable_type_forgets_metatable_and_narrows_to_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.meta(
      SubtypeFixture::props(vec![("u", Property::rw_type_id(b.boolean))]),
      SubtypeFixture::props(vec![("x", Property::rw_type_id(b.number))]),
    );
    let super_ty = rw_tbl(&mut fixture, &[("x", b.number)]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `"{ @metatable { x: number } } <!: { x: number }"` (L911)
  #[test]
  fn metatable_type_not_subtype_of_plain_table_with_same_property() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.meta(
      SubtypeFixture::props(vec![("x", Property::rw_type_id(b.number))]),
      SubtypeFixture::props(vec![]),
    );
    let super_ty = rw_tbl(&mut fixture, &[("x", b.number)]);
    check_not_subtype!(fixture, sub, super_ty);
  }
}

/// indexer（`[K]: V`）子类型：键与值都不变（invariant）。
mod indexers {
  use super::*;

  /// cpp: `TEST_IS_SUBTYPE(idx(number, number), tbl({}))` (L1398)
  #[test]
  fn indexed_table_subtype_of_empty_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.idx(b.number, b.number);
    let super_ty = rw_tbl(&mut fixture, &[]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(tbl({}), idx(number, number))` (L1399)
  #[test]
  fn empty_table_not_subtype_of_indexed_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[]);
    let super_ty = fixture.idx(b.number, b.number);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(tbl({X: number}), idx(number, number))` (L1401)
  #[test]
  fn property_table_not_subtype_of_indexed_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("X", b.number)]);
    let super_ty = fixture.idx(b.number, b.number);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(idx(number, number), tbl({X: number}))` (L1402)
  #[test]
  fn indexed_table_not_subtype_of_property_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.idx(b.number, b.number);
    let super_ty = rw_tbl(&mut fixture, &[("X", b.number)]);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: indexer 键为 union 时两个方向都不是子类型 (L1404-1411)
  #[test]
  fn indexer_key_is_invariant() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let number_or_string = fixture.join(b.number, b.string);
    let widened_key = fixture.idx(number_or_string, b.number);
    let plain_key = fixture.idx(b.number, b.number);
    check_not_subtype!(fixture, widened_key, plain_key);
    check_not_subtype!(fixture, plain_key, widened_key);
  }

  /// cpp: indexer 值为 union 时两个方向都不是子类型 (L1413-1420)
  #[test]
  fn indexer_value_is_invariant() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let number_or_string = fixture.join(b.number, b.string);
    let widened_value = fixture.idx(b.number, number_or_string);
    let plain_value = fixture.idx(b.number, b.number);
    check_not_subtype!(fixture, widened_value, plain_value);
    check_not_subtype!(fixture, plain_value, widened_value);
  }

  /// cpp: `"{ read [number] : string } <: { read [number] : string | number }"` (L1422)
  ///
  /// 注：cpp `Subtyping::isCovariantWith(TableIndexer, …)` 中只读索引器的协变规则是无条件的，
  /// Rust 端口把它放在 `fflag::LuauReadOnlyIndexers` 之后且默认关闭（旧快照残留），
  /// 故此处按 cpp 语义显式打开该 flag；默认路径的偏差已记录到任务报告。
  #[test]
  fn read_only_indexer_value_is_covariant() {
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _read_only_indexers = ScopedFastFlag::new(&fflag::LuauReadOnlyIndexers, true);
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let number_or_string = fixture.join(b.string, b.number);
    let sub = idx_read_only(&mut fixture, b.number, b.string);
    let super_ty = idx_read_only(&mut fixture, b.number, number_or_string);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(tbl({X: number}), idx(string, number))` (L1431)
  #[test]
  fn string_keyed_indexer_rejects_property_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = rw_tbl(&mut fixture, &[("X", b.number)]);
    let super_ty = fixture.idx(b.string, b.number);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(idx(string, number), tbl({X: number}))` (L1432)
  #[test]
  fn string_keyed_indexer_accepts_property_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.idx(b.string, b.number);
    let super_ty = rw_tbl(&mut fixture, &[("X", b.number)]);
    check_subtype!(fixture, sub, super_ty);
  }

  /// cpp: 可选属性两侧的 indexer 比较 (L1434-1435)
  #[test]
  fn optional_property_table_and_indexer_are_incomparable() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let optional_number = fixture.opt(b.number);
    let optional_x = rw_tbl(&mut fixture, &[("X", optional_number)]);
    let indexer = fixture.idx(b.string, b.number);
    check_not_subtype!(fixture, optional_x, indexer);
    check_not_subtype!(fixture, indexer, optional_x);
  }
}

/// 否定类型（`~T`）作为子类型 / 超类型的语义子类型用例。
mod negations {
  use super::*;

  /// cpp: `TEST_IS_NOT_SUBTYPE(negate(neverType), stringType)` (L920)
  #[test]
  fn negated_never_is_not_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.negate(b.never);
    check_not_subtype!(fixture, sub, b.string);
  }

  /// cpp: `TEST_IS_SUBTYPE(negate(unknownType), stringType)` (L921)
  #[test]
  fn negated_unknown_is_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.negate(b.unknown);
    check_subtype!(fixture, sub, b.string);
  }

  /// cpp: `TEST_IS_SUBTYPE(negate(anyType), stringType)` (L922)
  #[test]
  fn negated_any_is_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fixture.negate(b.any);
    check_subtype!(fixture, sub, b.string);
  }

  /// cpp: `TEST_IS_SUBTYPE(negate(meet(never, unknown)), stringType)` (L923)
  #[test]
  fn negated_never_and_unknown_intersection_is_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let meet = fixture.meet(b.never, b.unknown);
    let sub = fixture.negate(meet);
    check_subtype!(fixture, sub, b.string);
  }

  /// cpp: `TEST_IS_SUBTYPE(negate(join(never, unknown)), stringType)` (L924)
  #[test]
  fn negated_never_or_unknown_union_is_subtype_of_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let join = fixture.join(b.never, b.unknown);
    let sub = fixture.negate(join);
    check_subtype!(fixture, sub, b.string);
  }

  /// cpp: `TEST_IS_SUBTYPE(stringType, negate(neverType))` (L927)
  #[test]
  fn string_is_subtype_of_negated_never() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let super_ty = fixture.negate(b.never);
    check_subtype!(fixture, b.string, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(neverType, negate(unknownType))` (L928)
  #[test]
  fn never_is_subtype_of_negated_unknown() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let super_ty = fixture.negate(b.unknown);
    check_subtype!(fixture, b.never, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(stringType, negate(unknownType))` (L929)
  #[test]
  fn string_is_not_subtype_of_negated_unknown() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let super_ty = fixture.negate(b.unknown);
    check_not_subtype!(fixture, b.string, super_ty);
  }

  /// cpp: number / unknown 对 `~any` 的关系 (L930-931)
  #[test]
  fn number_and_unknown_are_subtypes_of_negated_any() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let super_ty = fixture.negate(b.any);
    check_subtype!(fixture, b.number, super_ty);
    check_subtype!(fixture, b.unknown, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(booleanType, negate(join(string, number)))` (L934)
  #[test]
  fn boolean_is_subtype_of_negated_string_or_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let join = fixture.join(b.string, b.number);
    let super_ty = fixture.negate(join);
    check_subtype!(fixture, b.boolean, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(rootClass, negate(join(childClass, number)))` (L935)
  #[test]
  fn root_class_is_subtype_of_negated_child_or_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let join = fixture.join(hierarchy.child_class, b.number);
    let super_ty = fixture.negate(join);
    check_subtype!(fixture, hierarchy.root_class, super_ty);
  }

  /// cpp: 字符串单例与 `~(number | boolean)` / `~(string | number)` (L936-937)
  #[test]
  fn string_singleton_against_negated_unions() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let foo = fixture.str("foo");
    let number_or_boolean = fixture.join(b.number, b.boolean);
    let super_ty = fixture.negate(number_or_boolean);
    check_subtype!(fixture, foo, super_ty);
    let string_or_number = fixture.join(b.string, b.number);
    let super_ty = fixture.negate(string_or_number);
    check_not_subtype!(fixture, foo, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(childClass, negate(join(rootClass, number)))` (L938)
  #[test]
  fn child_class_is_not_subtype_of_negated_root_or_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let join = fixture.join(hierarchy.root_class, b.number);
    let super_ty = fixture.negate(join);
    check_not_subtype!(fixture, hierarchy.child_class, super_ty);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(numbersToNumberType, negate(join(functionType, rootClass)))` (L939)
  #[test]
  fn variadic_number_function_is_not_subtype_of_negated_function_or_class() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let sub = variadic_fn(&mut fixture, b.number, b.number);
    let join = fixture.join(b.function, hierarchy.root_class);
    let super_ty = fixture.negate(join);
    check_not_subtype!(fixture, sub, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(booleanType, negate(meet(string, "foo")))` (L942)
  #[test]
  fn boolean_is_subtype_of_negated_string_and_foo_intersection() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let foo = fixture.str("foo");
    let meet = fixture.meet(b.string, foo);
    let super_ty = fixture.negate(meet);
    check_subtype!(fixture, b.boolean, super_ty);
  }

  /// cpp: `TEST_IS_SUBTYPE(trueType, negate(meet(boolean, number)))` (L943)
  #[test]
  fn true_is_subtype_of_negated_boolean_and_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let meet = fixture.meet(b.boolean, b.number);
    let super_ty = fixture.negate(meet);
    check_subtype!(fixture, b.boolean_true, super_ty);
  }

  /// cpp: extern 类型交集的否定 (L944-946)
  #[test]
  fn classes_and_unknown_against_negated_extern_intersections() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();

    let meet = fixture.meet(b.extern_ty, hierarchy.child_class);
    check_subtype!(fixture, hierarchy.root_class, fixture.negate(meet));

    let meet = fixture.meet(b.extern_ty, b.number);
    check_subtype!(fixture, hierarchy.child_class, fixture.negate(meet));
    check_subtype!(fixture, b.unknown, fixture.negate(meet));
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE("foo", negate(string & ~"bar"))` (L947)
  #[test]
  fn string_singleton_is_not_subtype_of_negated_refined_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let foo = fixture.str("foo");
    let bar = fixture.str("bar");
    let not_bar = fixture.negate(bar);
    let meet = fixture.meet(b.string, not_bar);
    let super_ty = fixture.negate(meet);
    check_not_subtype!(fixture, foo, super_ty);
  }

  /// cpp: 表与元表对 `~number` / `~table` 的关系 (L950-953)
  #[test]
  fn tables_and_metatables_against_negated_primitives() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let empty = rw_tbl(&mut fixture, &[]);
    let meta = fixture.meta(SubtypeFixture::props(vec![]), SubtypeFixture::props(vec![]));
    check_subtype!(fixture, empty, fixture.negate(b.number));
    check_not_subtype!(fixture, empty, fixture.negate(b.table));
    check_subtype!(fixture, meta, fixture.negate(b.number));
    check_not_subtype!(fixture, meta, fixture.negate(b.table));
  }

  /// cpp: 函数对 `~userdata` / `~fun` (L956-957)
  #[test]
  fn function_against_negated_extern_and_function() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let sub = fn_ty(&mut fixture, &[b.number], &[b.number]);
    check_subtype!(fixture, sub, fixture.negate(b.extern_ty));
    check_not_subtype!(fixture, sub, fixture.negate(b.function));
  }

  /// cpp: 原语与单例对自身的否定 (L960-961)
  #[test]
  fn primitive_is_not_subtype_of_its_own_negation() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_not_subtype!(fixture, b.string, fixture.negate(b.string));
    check_subtype!(fixture, b.string, fixture.negate(b.number));
  }

  /// cpp: `TEST_IS_SUBTYPE("foo", string & ~"bar")` (L962)
  #[test]
  fn string_singleton_is_subtype_of_refined_string() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let foo = fixture.str("foo");
    let bar = fixture.str("bar");
    let not_bar = fixture.negate(bar);
    let super_ty = fixture.meet(b.string, not_bar);
    check_subtype!(fixture, foo, super_ty);
  }

  /// cpp: 布尔单例与 `~boolean` / `~"foo"` (L963-965)
  #[test]
  fn singletons_against_negations_of_their_primitives() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let foo = fixture.str("foo");
    check_not_subtype!(fixture, b.boolean_true, fixture.negate(b.boolean));
    check_not_subtype!(fixture, foo, fixture.negate(foo));
    check_not_subtype!(fixture, foo, fixture.negate(b.string));
  }

  /// cpp: `false <: ~true` 及其与 boolean 的交集 (L966-968)
  #[test]
  fn false_is_subtype_of_negated_true() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let not_true = fixture.negate(b.boolean_true);
    check_subtype!(fixture, b.boolean_false, not_true);
    let meet = fixture.meet(b.boolean, not_true);
    check_subtype!(fixture, b.boolean_false, meet);
    check_not_subtype!(fixture, b.string, meet);
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(stringType, negate("foo"))` (L969)
  #[test]
  fn string_is_not_subtype_of_negated_singleton() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let foo = fixture.str("foo");
    check_not_subtype!(fixture, b.string, fixture.negate(foo));
  }

  /// cpp: `TEST_IS_NOT_SUBTYPE(booleanType, negate(falseType))` (L970)
  #[test]
  fn boolean_is_not_subtype_of_negated_false() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let not_false = fixture.negate(b.boolean_false);
    check_not_subtype!(fixture, b.boolean, not_false);
  }

  /// cpp: extern 类型与 `~table` / `~userdata` / `~Root` (L973-977)
  #[test]
  fn extern_classes_against_negated_supertypes() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();

    check_subtype!(fixture, hierarchy.root_class, fixture.negate(b.table));
    check_not_subtype!(fixture, hierarchy.root_class, fixture.negate(b.extern_ty));
    check_not_subtype!(
      fixture,
      hierarchy.child_class,
      fixture.negate(hierarchy.root_class)
    );

    let not_root = fixture.negate(hierarchy.root_class);
    let meet_extern_not_root = fixture.meet(b.extern_ty, not_root);
    check_not_subtype!(fixture, hierarchy.child_class, meet_extern_not_root);

    let not_child = fixture.negate(hierarchy.child_class);
    let meet_extern_not_child = fixture.meet(b.extern_ty, not_child);
    check_subtype!(
      fixture,
      hierarchy.another_child_class,
      meet_extern_not_child
    );
  }

  /// cpp: unknown 对每个原语否定的关系 (L980-984)
  #[test]
  fn unknown_is_not_subtype_of_negated_primitives() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    for primitive in [b.boolean, b.number, b.string, b.thread, b.buffer] {
      let super_ty = fixture.negate(primitive);
      check_not_subtype!(fixture, b.unknown, super_ty);
    }
  }
}

/// 外部类型（userdata / object / class）与属性方差的表用例。
mod extern_types {
  use super::*;

  /// cpp: `"Root <: userdata"` (L986)
  #[test]
  fn root_class_is_subtype_of_userdata() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    check_subtype!(fixture, hierarchy.root_class, b.extern_ty);
  }

  /// cpp: `"Child | AnotherChild <: Child | AnotherChild"` (L996)
  #[test]
  fn class_union_is_subtype_of_itself() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let first = fixture.join(hierarchy.child_class, hierarchy.another_child_class);
    let second = fixture.join(hierarchy.child_class, hierarchy.another_child_class);
    check_subtype!(fixture, first, second);
  }

  /// cpp: `"Child | Root <: Root"` (L1001)
  #[test]
  fn class_union_with_parent_is_subtype_of_parent() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let sub = fixture.join(hierarchy.child_class, hierarchy.root_class);
    check_subtype!(fixture, sub, hierarchy.root_class);
  }

  /// cpp: `"Child & AnotherChild <: userdata"` (L1006)
  #[test]
  fn sibling_class_intersection_is_subtype_of_userdata() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let sub = fixture.meet(hierarchy.child_class, hierarchy.another_child_class);
    check_subtype!(fixture, sub, b.extern_ty);
  }

  /// cpp: `"Child & ~Root <: userdata"` (L1016)
  #[test]
  fn child_without_root_intersection_is_subtype_of_userdata() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let not_root = fixture.negate(hierarchy.root_class);
    let sub = fixture.meet(hierarchy.child_class, not_root);
    check_subtype!(fixture, sub, b.extern_ty);
  }

  /// cpp: `"Child & AnotherChild <: number"` (L1082) —— 不相交兄弟类的交集是 never。
  #[test]
  fn disjoint_sibling_intersection_is_subtype_of_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let sub = fixture.meet(hierarchy.child_class, hierarchy.another_child_class);
    check_subtype!(fixture, sub, b.number);
  }

  /// cpp: `"Child & ~GrandchildOne <!: number"` (L1087)
  #[test]
  fn child_without_one_grandchild_is_not_subtype_of_number() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hierarchy = fixture.class_hierarchy();
    let not_grandchild = fixture.negate(hierarchy.grandchild_one_class);
    let sub = fixture.meet(hierarchy.child_class, not_grandchild);
    check_not_subtype!(fixture, sub, b.number);
  }

  /// cpp: `"random extern type <!: object"` / `"object <!: class"` / `"class <!: object"` (L1021-1034)
  #[test]
  fn extern_object_and_class_are_mutually_incomparable() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    check_not_subtype!(fixture, b.extern_ty, b.object);
    check_not_subtype!(fixture, b.object, b.class);
    check_not_subtype!(fixture, b.class, b.object);
  }

  /// cpp: `"extern(object) <: object"` / `"extern(class) <: class"` (L1036-1046)
  #[test]
  fn user_defined_extern_types_subtype_their_root() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let my_object = fixture.obj("MyObject", None);
    check_subtype!(fixture, my_object, b.object);

    let my_class = fixture.cls_string_optional_type_id("MyClass", Some(b.class));
    check_subtype!(fixture, my_class, b.class);
  }

  /// cpp: `"multiple inheritance subclass object <: object"` (L1048)
  #[test]
  fn multiple_inheritance_objects_are_subtypes_of_object() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let base = fixture.obj("B", None);
    let derived = fixture.obj("A", Some(base));
    check_subtype!(fixture, derived, b.object);
    check_subtype!(fixture, base, b.object);
    check_not_subtype!(fixture, base, derived);
  }

  /// cpp: `"class A and B class subtypes"` (L1057)
  #[test]
  fn independent_classes_are_subtypes_of_class() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let a = fixture.cls_string_optional_type_id("A", Some(b.class));
    let c = fixture.cls_string_optional_type_id("B", Some(b.class));
    check_subtype!(fixture, a, b.class);
    check_subtype!(fixture, c, b.class);
  }

  /// cpp: `"class A and B not subtypes of each other"` (L1065)
  #[test]
  fn independent_classes_are_incomparable() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let a = fixture.cls_string_optional_type_id("A", Some(b.class));
    let c = fixture.cls_string_optional_type_id("B", Some(b.class));
    check_not_subtype!(fixture, a, c);
    check_not_subtype!(fixture, c, a);
  }

  /// cpp: `"Classes are subtypes of themselves"` (L1073)
  #[test]
  fn classes_are_subtypes_of_themselves() {
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let a = fixture.cls_string_optional_type_id("A", Some(b.class));
    let c = fixture.cls_string_optional_type_id("B", Some(b.class));
    check_subtype!(fixture, a, a);
    check_subtype!(fixture, c, c);
  }
}

/// 结构化外部类属性（Vec2 等）与表 / 属性修饰符的方差用例。
mod class_properties {
  use super::*;

  /// cpp `vec2Class`：`{ X: number, Y: number }` 形式的 userdata。
  fn vec2(fixture: &mut SubtypeFixture) -> TypeId {
    let b = builtin(fixture);
    fixture.cls_string_extern_type_props(
      "Vec2",
      SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(b.number)),
        ("Y", Property::rw_type_id(b.number)),
      ]),
    )
  }

  /// cpp `readOnlyVec2Class`。
  fn read_only_vec2(fixture: &mut SubtypeFixture) -> TypeId {
    let b = builtin(fixture);
    fixture.cls_string_extern_type_props(
      "ReadOnlyVec2",
      SubtypeFixture::props(vec![
        ("X", Property::readonly(b.number)),
        ("Y", Property::readonly(b.number)),
      ]),
    )
  }

  /// cpp: `"Vec2 <: { X: number, Y: number }"` / `"Vec2 <: { X: number }"` (L1158-1175)
  #[test]
  fn vec2_class_is_subtype_of_matching_tables() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let vec2 = vec2(&mut fixture);
    let xy = rw_tbl(&mut fixture, &[("X", b.number), ("Y", b.number)]);
    let x = rw_tbl(&mut fixture, &[("X", b.number)]);
    check_subtype!(fixture, vec2, xy);
    check_subtype!(fixture, vec2, x);
  }

  /// cpp: 表不能反过来成为 Vec2 的子类型 (L1177-1194)
  #[test]
  fn matching_tables_are_not_subtypes_of_vec2_class() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let vec2 = vec2(&mut fixture);
    let xy = rw_tbl(&mut fixture, &[("X", b.number), ("Y", b.number)]);
    let x = rw_tbl(&mut fixture, &[("X", b.number)]);
    check_not_subtype!(fixture, xy, vec2);
    check_not_subtype!(fixture, x, vec2);
  }

  /// cpp: `table & { X, Y }` 与 Vec2 互不为子类型 (L1196-1214)
  #[test]
  fn table_intersection_with_props_is_incomparable_with_vec2_class() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let vec2 = vec2(&mut fixture);
    let xy = rw_tbl(&mut fixture, &[("X", b.number), ("Y", b.number)]);
    let meet = fixture.meet(b.table, xy);
    check_not_subtype!(fixture, meet, vec2);
    check_not_subtype!(fixture, vec2, meet);
  }

  /// cpp: ReadOnlyVec2 与读写 / 只读表 (L1216-1228)
  #[test]
  fn read_only_vec2_requires_read_only_properties() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let read_only = read_only_vec2(&mut fixture);
    let vec2 = vec2(&mut fixture);
    let rw_xy = rw_tbl(&mut fixture, &[("X", b.number), ("Y", b.number)]);
    check_not_subtype!(fixture, read_only, rw_xy);

    let read_props = SubtypeFixture::props(vec![
      ("X", Property::readonly(b.number)),
      ("Y", Property::readonly(b.number)),
    ]);
    let read_xy = fixture.tbl(read_props);
    check_subtype!(fixture, read_only, read_xy);
    check_subtype!(fixture, vec2, read_xy);
  }

  /// cpp: 表属性上 read/write 修饰符的方差 (L1230-1232)
  #[test]
  fn table_property_modifiers_are_variant_aware() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();

    let grandchild_p = rw_tbl(&mut fixture, &[("P", hierarchy.grandchild_one_class)]);
    let rw_root = rw_tbl(&mut fixture, &[("P", hierarchy.root_class)]);
    check_not_subtype!(fixture, grandchild_p, rw_root);

    let props = SubtypeFixture::props(vec![("P", Property::readonly(hierarchy.root_class))]);
    let read_root = fixture.tbl(props);
    check_subtype!(fixture, grandchild_p, read_root);

    let sub = rw_tbl(&mut fixture, &[("P", hierarchy.root_class)]);
    let props = SubtypeFixture::props(vec![(
      "P",
      Property::writeonly(hierarchy.grandchild_one_class),
    )]);
    let write_grandchild = fixture.tbl(props);
    check_subtype!(fixture, sub, write_grandchild);
  }

  /// cpp: 持有 `Child` 属性的类对 Root / GrandchildOne 的关系 (L1234-1237)
  #[test]
  fn has_child_class_against_property_modifiers() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let has_child = fixture.cls_string_extern_type_props(
      "HasChild",
      SubtypeFixture::props(vec![("P", Property::rw_type_id(hierarchy.child_class))]),
    );

    let rw_root = rw_tbl(&mut fixture, &[("P", hierarchy.root_class)]);
    check_not_subtype!(fixture, has_child, rw_root);

    let read_root = fixture.tbl(SubtypeFixture::props(vec![(
      "P",
      Property::readonly(hierarchy.root_class),
    )]));
    check_subtype!(fixture, has_child, read_root);

    let grandchild = rw_tbl(&mut fixture, &[("P", hierarchy.grandchild_one_class)]);
    check_not_subtype!(fixture, has_child, grandchild);

    let write_grandchild = fixture.tbl(SubtypeFixture::props(vec![(
      "P",
      Property::writeonly(hierarchy.grandchild_one_class),
    )]));
    check_subtype!(fixture, has_child, write_grandchild);
  }
}

/// 字符串标量与「带方法的表」之间的语义子类型（依赖内置 string 元表）。
mod string_metatable {
  use super::*;

  /// cpp `tableWithoutScalarProp`。
  fn table_without_scalar_prop(fixture: &mut SubtypeFixture) -> TypeId {
    let nothing_to_nothing = fn_ty(fixture, &[], &[]);
    rw_tbl(fixture, &[("insaneThingNoScalarHas", nothing_to_nothing)])
  }

  /// cpp: `"hello"` / `string` 与 `{ lower }`、`{ insaneThingNoScalarHas }` (L1239-1257)
  #[test]
  fn strings_subtype_tables_matching_their_metatable_methods() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let hello = fixture.str("hello");
    let table_with_lower = fixture.table_with_lower();
    let without_scalar = table_without_scalar_prop(&mut fixture);

    check_subtype!(fixture, hello, table_with_lower);
    check_not_subtype!(fixture, hello, without_scalar);
    check_subtype!(fixture, b.string, table_with_lower);
    check_not_subtype!(fixture, b.string, without_scalar);
  }

  /// cpp: `"a" | (~"b" & string) <: { lower : (string) -> ()}` (L1274)
  #[test]
  fn string_union_with_refinement_is_subtype_of_lower_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let a = fixture.str("a");
    let b_string = fixture.str("b");
    let not_b = fixture.negate(b_string);
    let refined = fixture.meet(not_b, b.string);
    let sub = fixture.join(a, refined);
    let table_with_lower = fixture.table_with_lower();
    check_subtype!(fixture, sub, table_with_lower);
  }

  /// cpp: `(string | number) & ("a" | true) <: { lower: (string) -> string }` (L1279)
  #[test]
  fn intersection_of_union_and_singleton_union_is_subtype_of_lower_table() {
    let mut fixture = SubtypeFixture::default();
    let b = builtin(&mut fixture);
    let string_or_number = fixture.join(b.string, b.number);
    let a = fixture.str("a");
    let a_or_true = fixture.join(a, b.boolean_true);
    let base = fixture.meet(string_or_number, a_or_true);
    let table_with_lower = fixture.table_with_lower();
    check_subtype!(fixture, base, table_with_lower);
  }
}
