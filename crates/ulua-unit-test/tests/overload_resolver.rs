//! Port of `cpp/tests/OverloadResolver.test.cpp`（12 个 TEST_CASE）。
//!
//! 被测对象：`OverloadResolver::resolve_overload`——交叉类型 `A & B` 的每个
//! 分量是一个重载候选，逐个与实参包做子类型检查后归入 `ok`/不兼容等桶。
//! 对应 C++ 实现：`Analysis/src/OverloadResolver.cpp`。

extern crate alloc;

use ulua_ast::records::location::Location;

// Source: `tests/OverloadResolver.test.cpp:107-115`
#[test]
fn new_basic_overload_selection() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  // ty: (number) -> number & (string) -> string
  // args: (number)
  let number = fixture.builtin_types.number_type;
  let args = fixture.pack_initializer_list_type_id(&[number]);
  let result = fixture.resolver.resolve_overload(
    fixture.number_to_number_and_string_to_string,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, result.ok.len());
  assert_eq!(fixture.number_to_number, result.ok[0]);
}

// Source: `tests/OverloadResolver.test.cpp:118-130`
#[test]
fn new_basic_overload_selection1() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  // ty: (number) -> number & (string) -> string
  // args: (string)
  let string = fixture.builtin_types.string_type;
  let args = fixture.pack_initializer_list_type_id(&[string]);
  let result = fixture.resolver.resolve_overload(
    fixture.number_to_number_and_string_to_string,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, result.ok.len());
  assert_eq!(fixture.string_to_string, result.ok[0]);

  assert_eq!(1, result.incompatible_overloads.len());
  assert_eq!(fixture.number_to_number, result.incompatible_overloads[0].0);
}

// Source: `tests/OverloadResolver.test.cpp:132-148`
#[test]
fn new_match_call_metamethod() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  // (unknown, number) -> number
  let unknown = fixture.builtin_types.unknown_type;
  let number = fixture.builtin_types.number_type;
  let call_mm = fixture.fn_type(&[unknown, number], &[number]);
  let tbl = fixture.table_with_call(call_mm);

  let args = fixture.pack_initializer_list_type_id(&[number]);
  let result =
    fixture
      .resolver
      .resolve_overload(tbl, args, Location::default(), fixture.empty_set, false);

  // C++ 注释指出的设计问题：命中的重载元数与实参包明显不同。
  assert_eq!(1, result.ok.len());
  assert_eq!(call_mm, result.ok[0]);
}

// Source: `tests/OverloadResolver.test.cpp:150-171`
#[test]
fn new_metamethod_could_be_overloaded() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  let unknown = fixture.builtin_types.unknown_type;
  let number = fixture.builtin_types.number_type;
  let string = fixture.builtin_types.string_type;

  // (unknown, number) -> number
  let overload1 = fixture.fn_type(&[unknown, number], &[number]);
  // (unknown, string) -> string
  let overload2 = fixture.fn_type(&[unknown, string], &[string]);
  // __call 自身可以是重载的
  let tbl = fixture.table_with_call(fixture.meet_initializer_list_type_id(&[overload1, overload2]));

  let args = fixture.pack_initializer_list_type_id(&[number]);
  let result =
    fixture
      .resolver
      .resolve_overload(tbl, args, Location::default(), fixture.empty_set, false);

  assert_eq!(1, result.ok.len());
  assert_eq!(overload1, result.ok[0]);

  assert_eq!(1, result.incompatible_overloads.len());
  assert_eq!(overload2, result.incompatible_overloads[0].0);
}

// Source: `tests/OverloadResolver.test.cpp:173-187`
#[test]
fn new_overload_group_could_include_metamethod() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  let unknown = fixture.builtin_types.unknown_type;
  let number = fixture.builtin_types.number_type;
  let string = fixture.builtin_types.string_type;
  let boolean = fixture.builtin_types.boolean_type;

  // (unknown, number) -> number
  let overload1 = fixture.fn_type(&[unknown, number], &[number]);
  // (unknown, string) -> string
  let overload2 = fixture.fn_type(&[unknown, string], &[string]);
  let tbl = fixture.table_with_call(fixture.meet_initializer_list_type_id(&[overload1, overload2]));

  // 重载组里可以混进带 __call 元表的表：{....} & (number) -> number
  let monstrosity =
    fixture.meet_initializer_list_type_id(&[tbl, fixture.fn_type(&[boolean], &[boolean])]);

  let args = fixture.pack_initializer_list_type_id(&[number]);
  let result = fixture.resolver.resolve_overload(
    monstrosity,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, result.ok.len());
  assert_eq!(overload1, result.ok[0]);
}

// Source: `tests/OverloadResolver.test.cpp:189-201`
#[test]
fn new_overloads_with_different_arities() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  // ty: (number) -> number & (number, number) -> number
  // args: (number)
  let number = fixture.builtin_types.number_type;
  let args = fixture.pack_initializer_list_type_id(&[number]);
  let result = fixture.resolver.resolve_overload(
    fixture.number_to_number_and_number_number_to_number,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, result.ok.len());
  assert_eq!(fixture.number_to_number, result.ok[0]);

  assert_eq!(1, result.arity_mismatches.len());
  assert_eq!(fixture.number_number_to_number, result.arity_mismatches[0]);
}

// Source: `tests/OverloadResolver.test.cpp:203-216`
#[test]
fn new_overloads_with_different_arities1() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  // ty: (number) -> number & (number, number) -> number
  // args: (number, number)
  let number = fixture.builtin_types.number_type;
  let args = fixture.pack_initializer_list_type_id(&[number, number]);
  let result = fixture.resolver.resolve_overload(
    fixture.number_to_number_and_number_number_to_number,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, result.ok.len());
  assert_eq!(fixture.number_number_to_number, result.ok[0]);

  assert_eq!(1, result.arity_mismatches.len());
  assert_eq!(fixture.number_to_number, result.arity_mismatches[0]);
}

// Source: `tests/OverloadResolver.test.cpp:218-245`
#[test]
fn new_separate_non_viable_overloads_by_arity_mismatch() {
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  // ty: ((number)->number) & ((number)->string) & ((number, number)->number)
  // args: (string)
  let string = fixture.builtin_types.string_type;
  let ty = fixture.meet_initializer_list_type_id(&[
    fixture.number_to_number,
    fixture.number_to_string,
    fixture.number_number_to_number,
  ]);
  let args = fixture.pack_initializer_list_type_id(&[string]);

  let resolution =
    fixture
      .resolver
      .resolve_overload(ty, args, Location::default(), fixture.empty_set, false);

  assert!(resolution.ok.is_empty());
  assert!(resolution.non_functions.is_empty());
  assert_eq!(1, resolution.arity_mismatches.len());
  assert_eq!(
    fixture.number_number_to_number,
    resolution.arity_mismatches[0]
  );

  assert_eq!(2, resolution.incompatible_overloads.len());
  let incompatible: Vec<_> = resolution
    .incompatible_overloads
    .iter()
    .map(|(ty, _)| *ty)
    .collect();
  assert!(incompatible.contains(&fixture.number_to_number));
  assert!(incompatible.contains(&fixture.number_to_string));
}

// Source: `tests/OverloadResolver.test.cpp:247-259`
#[test]
fn new_select() {
  use alloc::string::String;

  use ulua_analysis::{
    records::{function_type::FunctionType, generic_type_pack::GenericTypePack},
    type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
  };
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  let number = fixture.builtin_types.number_type;
  let string = fixture.builtin_types.string_type;
  let any_type_pack = fixture.builtin_types.any_type_pack;

  let number_or_string = fixture.join(number, string);
  // Safety: fixture.arena 指向 fixture.arena_ 持有的 TypeArena，测试期内存活。
  let generic_as: TypePackId =
    unsafe { (*fixture.arena).add_type_pack_t(GenericTypePack::new_name(String::from("A"))) };

  // select: <A...>(number | string, ...A) -> ...any
  // Safety: fixture.arena 指向 fixture.arena_ 持有的 TypeArena，测试期内存活；
  // 独占 arena 顺序追加，无其他活动借用。
  let arg_types: TypePackId = unsafe {
    (*fixture.arena)
      .add_type_pack_vector_type_id_optional_type_pack_id(vec![number_or_string], Some(generic_as))
  };
  // Safety: 同上契约；泛型参数为本帧先前在同一 arena 构造的存活 id。
  let select_ty: TypeId = unsafe {
    (*fixture.arena).add_type(FunctionType::new_with_generics(
      Vec::new(),
      vec![generic_as],
      arg_types,
      any_type_pack,
      None,
      false,
    ))
  };

  let args: TypePackId = unsafe {
    (*fixture.arena).add_type_pack_vector_type_id_optional_type_pack_id(
      vec![number_or_string],
      Some(any_type_pack),
    )
  };

  let resolution = fixture.resolver.resolve_overload(
    select_ty,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, resolution.ok.len());
}

// Source: `tests/OverloadResolver.test.cpp:261-278`
#[test]
fn new_pass_table_with_indexer() {
  use ulua_analysis::{
    enums::table_state::TableState,
    records::{
      scope::Scope, table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel,
    },
    type_aliases::{props_type::Props, type_id::TypeId},
  };
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  let any = fixture.builtin_types.any_type;
  let number = fixture.builtin_types.number_type;
  let root_scope = &mut *fixture.root_scope as *mut Scope;

  // {[any]: number}
  let indexed_table =
    TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
      &Props::new(),
      Some(TableIndexer {
        index_type: any,
        index_result_type: number,
        is_read_only: false,
      }),
      TypeLevel::default(),
      root_scope,
      TableState::Sealed,
    );
  // Safety: fixture.arena 指向 fixture.arena_ 独占保有的 TypeArena（测试期内
  // 存活、无并发访问）；本行为独占 arena 的顺序追加，值已在安全区构造。
  let any_number_table: TypeId = unsafe { (*fixture.arena).add_type(indexed_table) };

  let table_to_table = fixture.fn_type(&[any_number_table], &[any_number_table]);
  let args = fixture.pack_initializer_list_type_id(&[any_number_table]);

  let resolution = fixture.resolver.resolve_overload(
    table_to_table,
    args,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, resolution.ok.len());
  assert_eq!(0, resolution.potential_overloads.len());
  assert_eq!(0, resolution.incompatible_overloads.len());
  assert_eq!(0, resolution.non_functions.len());
  assert_eq!(0, resolution.arity_mismatches.len());
}

// Source: `tests/OverloadResolver.test.cpp:280-299`
#[test]
fn generic_higher_order_function_called_improperly() {
  use alloc::string::String;

  use ulua_analysis::{
    enums::polarity::Polarity,
    records::{
      function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    },
    type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
  };
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use crate::Location;

  let mut fixture = OverloadResolverFixture::new();

  let number = fixture.builtin_types.number_type;

  // apply: <A, B..., C...>((A, B...) -> C..., A) -> C...
  let (generic_a, generic_bs, generic_cs): (TypeId, TypePackId, TypePackId) = unsafe {
    let generic_a = (*fixture.arena).add_type(GenericType::generic_type_name_polarity(
      &String::from("A"),
      Polarity::Mixed,
    ));
    let generic_bs = (*fixture.arena).add_type_pack_t(GenericTypePack::new_name(String::from("B")));
    let generic_cs = (*fixture.arena).add_type_pack_t(GenericTypePack::new_name(String::from("C")));
    (generic_a, generic_bs, generic_cs)
  };

  let function_argument: TypeId = unsafe {
    let arg_types = (*fixture.arena)
      .add_type_pack_vector_type_id_optional_type_pack_id(vec![generic_a], Some(generic_bs));
    (*fixture.arena).add_type(FunctionType::function_type_new(
      arg_types, generic_cs, None, false,
    ))
  };

  let apply_args = fixture.pack_initializer_list_type_id(&[function_argument, generic_a]);

  let apply_ty: TypeId = unsafe {
    (*fixture.arena).add_type(FunctionType::new_with_generics(
      vec![generic_a],
      vec![generic_bs, generic_cs],
      apply_args,
      generic_cs,
      None,
      false,
    ))
  };

  let call_args_pack =
    fixture.pack_initializer_list_type_id(&[fixture.number_number_to_number, number]);

  let resolution = fixture.resolver.resolve_overload(
    apply_ty,
    call_args_pack,
    Location::default(),
    fixture.empty_set,
    false,
  );

  assert_eq!(1, resolution.ok.len());
}

mod debug_traceback {
  //! Source: `tests/OverloadResolver.test.cpp:301-349`

  use ulua_analysis::type_aliases::type_id::TypeId;
  use ulua_unit_test::records::overload_resolver_fixture::OverloadResolverFixture;

  use super::Location;

  /// debug.traceback:
  /// `((message: string?, level: number?) -> string) &`
  /// `((thread: thread, message: string?, level: number?) -> string)`
  ///
  /// C++ 用 doctest SUBCASE——每个子用例都会重新构造 fixture，故此处同样按子用例
  /// 新建 fixture，使 arena 与 `uniqueTypes` 集合互不干扰。
  fn resolve(subcase: &str, args: impl Fn(&OverloadResolverFixture) -> Vec<TypeId>) {
    let mut fixture = OverloadResolverFixture::new();

    let optional_string = fixture.builtin_types.optional_string_type;
    let optional_number = fixture.builtin_types.optional_number_type;
    let string = fixture.builtin_types.string_type;
    let thread = fixture.builtin_types.thread_type;

    let overload1 = fixture.fn_type(&[optional_string, optional_number], &[string]);
    let overload2 = fixture.fn_type(&[thread, optional_string, optional_number], &[string]);
    let debug_traceback = fixture.meet_initializer_list_type_id(&[overload1, overload2]);

    let arg_types = args(&fixture);
    let args = fixture.pack_initializer_list_type_id(&arg_types);
    let resolution = fixture.resolver.resolve_overload(
      debug_traceback,
      args,
      Location::default(),
      fixture.empty_set,
      false,
    );

    assert_eq!(1, resolution.ok.len(), "{subcase} 应命中唯一重载");
  }

  #[test]
  fn debug_traceback() {
    resolve("no_arguments", |_| Vec::new());
    resolve("message_only", |f| vec![f.builtin_types.string_type]);
    resolve("message_and_level", |f| {
      vec![f.builtin_types.string_type, f.builtin_types.number_type]
    });
    resolve("thread", |f| vec![f.builtin_types.thread_type]);
    resolve("thread_and_message", |f| {
      vec![f.builtin_types.thread_type, f.builtin_types.string_type]
    });
    resolve("thread_message_and_level", |f| {
      vec![
        f.builtin_types.thread_type,
        f.builtin_types.string_type,
        f.builtin_types.number_type,
      ]
    });
  }
}
