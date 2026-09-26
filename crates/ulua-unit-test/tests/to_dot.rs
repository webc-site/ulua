use core::ptr::null_mut;

use ulua_analysis::functions::to_dot_to_dot;
extern crate alloc;

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_bound() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot,
    records::{to_dot_options::ToDotOptions, type_arena::TypeArena},
    type_aliases::bound_type::BoundType,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let number_type = fixture.get_builtins().number_type;
  let mut arena = TypeArena::default();
  let ty = arena.add_type(BoundType::bound_t(number_type));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"BoundType 1\"];\nn1 -> n2;\nn2 [label=\"number\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_bound_pack() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot_type_pack_id as to_dot,
    records::{to_dot_options::ToDotOptions, type_arena::TypeArena, type_pack::TypePack},
    type_aliases::bound_type_pack::BoundTypePack,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let number_type = fixture.get_builtins().number_type;
  let mut arena = TypeArena::default();
  let pack = arena.add_type_pack_t(TypePack::new(alloc::vec![number_type], None));
  let bound = arena.add_type_pack_t(BoundTypePack::bound_t(pack));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"BoundTypePack 1\"];\nn1 -> n2;\nn2 [label=\"TypePack 2\"];\nn2 -> n3;\nn3 [label=\"number\"];\n}",
    to_dot(bound, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_bound_table() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot,
    records::{
      property_type::Property, table_type::TableType, to_dot_options::ToDotOptions,
      type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let number_type = fixture.get_builtins().number_type;
  let mut arena = TypeArena::default();

  let mut table = TableType::new();
  table
    .props
    .insert(String::from("x"), Property::rw_type_id(number_type));
  let ty = arena.add_type(table);

  let mut bound_table = TableType::new();
  bound_table.bound_to = Some(ty);
  let bound_ty = arena.add_type(bound_table);

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"TableType 1\"];\nn1 -> n2 [label=\"bound_to\"];\nn2 [label=\"TableType 2\"];\nn2 -> n3 [label=\"x\"];\nn3 [label=\"number\"];\n}",
    to_dot(bound_ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_builtintypes() {
  use ulua_analysis::{functions::to_dot_to_dot::to_dot, records::to_dot_options::ToDotOptions};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: "hi" | "\"hello\"" | true | false
    "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"UnionType 1\"];\nn1 -> n2;\nn2 [label=\"SingletonType string: hi\"];\nn1 -> n3;\nn3 [label=\"SingletonType string: \\\"hello\\\"\"];\nn1 -> n4;\nn4 [label=\"SingletonType boolean: true\"];\nn1 -> n5;\nn5 [label=\"SingletonType boolean: false\"];\n}",
    to_dot(fixture.require_type_string("x"), &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_class() {
  use ulua_analysis::{functions::to_dot_to_dot::to_dot, records::to_dot_options::ToDotOptions};
  use ulua_unit_test::records::to_dot_class_fixture::ToDotClassFixture;

  let mut fixture = ToDotClassFixture::default();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local a: ChildClass
"#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"ExternType ChildClass\"];\nn1 -> n2 [label=\"ChildField\"];\nn2 [label=\"string\"];\nn1 -> n3 [label=\"[parent]\"];\nn3 [label=\"ExternType BaseClass\"];\nn3 -> n4 [label=\"BaseField\"];\nn4 [label=\"number\"];\nn3 -> n5 [label=\"[metatable]\"];\nn5 [label=\"TableType 5\"];\n}",
    to_dot(fixture.base.require_type_string("a"), &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_error() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot,
    records::{to_dot_options::ToDotOptions, type_arena::TypeArena},
    type_aliases::error_type::ErrorType,
  };

  let mut arena = TypeArena::default();
  let ty = arena.add_type(ErrorType::new());

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"ErrorType 1\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_error_pack() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot_type_pack_id as to_dot,
    records::{to_dot_options::ToDotOptions, type_arena::TypeArena},
    type_aliases::error_type_pack::ErrorTypePack,
  };

  let mut arena = TypeArena::default();
  let pack = arena.add_type_pack_t(ErrorTypePack::new());

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"ErrorTypePack 1\"];\n}",
    to_dot(pack, &opts)
  );

  let _ = to_dot_to_dot::to_dot_default_opts_type_pack_id(pack);
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_free() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    functions::to_dot_to_dot::to_dot,
    records::{free_type::FreeType, to_dot_options::ToDotOptions, type_arena::TypeArena},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();
  let ty = arena.add_type(FreeType::free_type_scope_type_id_type_id_polarity(
    null_mut(),
    builtins.never_type,
    builtins.unknown_type,
    Polarity::Unknown,
  ));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"FreeType 1\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_free_pack() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot_type_pack_id as to_dot,
    records::{
      free_type_pack::FreeTypePack, to_dot_options::ToDotOptions, type_arena::TypeArena,
      type_level::TypeLevel,
    },
  };

  let mut arena = TypeArena::default();
  let pack = arena.add_type_pack_t(FreeTypePack::new(TypeLevel::default()));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"FreeTypePack 1\"];\n}",
    to_dot(pack, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_free_with_constraints() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    functions::to_dot_to_dot::to_dot,
    records::{free_type::FreeType, to_dot_options::ToDotOptions, type_arena::TypeArena},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();
  let ty = arena.add_type(FreeType::free_type_scope_type_id_type_id_polarity(
    null_mut(),
    builtins.number_type,
    builtins.optional_number_type,
    Polarity::Unknown,
  ));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"FreeType 1\"];\nn1 -> n2 [label=\"[lowerBound]\"];\nn2 [label=\"number\"];\nn1 -> n3 [label=\"[upperBound]\"];\nn3 [label=\"UnionType 3\"];\nn3 -> n4;\nn4 [label=\"number\"];\nn3 -> n5;\nn5 [label=\"nil\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_function() {
  use ulua_analysis::{
    functions::{to_dot_to_dot::to_dot, to_string_to_string::to_string_type_id},
    records::to_dot_options::ToDotOptions,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
local function f(a, ...: string) return a end
"#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let f_ty = fixture.require_type_string("f");
  assert_eq!("<a>(a, ...string) -> a", to_string_type_id(f_ty));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"FunctionType 1\"];\nn1 -> n2 [label=\"arg\"];\nn2 [label=\"TypePack 2\"];\nn2 -> n3;\nn3 [label=\"GenericType 3\"];\nn2 -> n4 [label=\"tail\"];\nn4 [label=\"VariadicTypePack 4\"];\nn4 -> n5;\nn5 [label=\"string\"];\nn1 -> n6 [label=\"ret\"];\nn6 [label=\"BoundTypePack 6\"];\nn6 -> n7;\nn7 [label=\"TypePack 7\"];\nn7 -> n3;\n}",
    to_dot(f_ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_generic() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    functions::to_dot_to_dot::to_dot,
    records::{
      generic_type::GenericType, to_dot_options::ToDotOptions, type_arena::TypeArena,
      type_level::TypeLevel,
    },
  };

  let mut arena = TypeArena::default();
  let ty = arena.add_type(GenericType {
    index: 0,
    level: TypeLevel::default(),
    scope: null_mut(),
    name: "T".to_string(),
    explicit_name: true,
    polarity: Polarity::Mixed,
  });

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"GenericType T\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_generic_pack() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot_type_pack_id as to_dot,
    records::{
      generic_type_pack::GenericTypePack, to_dot_options::ToDotOptions, type_arena::TypeArena,
    },
  };

  let mut arena = TypeArena::default();
  let pack1 = arena.add_type_pack_t(GenericTypePack::new());
  let pack2 = arena.add_type_pack_t(GenericTypePack::new_name("T".to_string()));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"GenericTypePack 1\"];\n}",
    to_dot(pack1, &opts)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"GenericTypePack T\"];\n}",
    to_dot(pack2, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_intersection() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot,
    records::{
      intersection_type::IntersectionType, to_dot_options::ToDotOptions, type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();
  let ty = arena.add_type(IntersectionType {
    parts: alloc::vec![builtins.string_type, builtins.number_type],
  });

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"IntersectionType 1\"];\nn1 -> n2;\nn2 [label=\"string\"];\nn1 -> n3;\nn3 [label=\"number\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_metatable() {
  use ulua_analysis::{functions::to_dot_to_dot::to_dot, records::to_dot_options::ToDotOptions};
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local a: typeof(setmetatable({}, {}))
"#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"MetatableType 1\"];\nn1 -> n2 [label=\"table\"];\nn2 [label=\"TableType 2\"];\nn1 -> n3 [label=\"metatable\"];\nn3 [label=\"TableType 3\"];\n}",
    to_dot(fixture.base.require_type_string("a"), &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_negation() {
  use ulua_analysis::{
    functions::to_dot_to_dot::to_dot,
    records::{negation_type::NegationType, to_dot_options::ToDotOptions, type_arena::TypeArena},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let string_type = fixture.get_builtins().string_type;
  let mut arena = TypeArena::default();
  let ty = arena.add_type(NegationType::new(string_type));

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"NegationType 1\"];\nn1 -> n2 [label=\"[negated]\"];\nn2 [label=\"string\"];\n}",
    to_dot(ty, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_no_duplicate_primitives() {
  use ulua_analysis::{functions::to_dot_to_dot::to_dot, records::to_dot_options::ToDotOptions};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let builtins = fixture.get_builtins();
  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: false,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"PrimitiveType number\"];\n}",
    to_dot(builtins.number_type, &opts)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"AnyType 1\"];\n}",
    to_dot(builtins.any_type, &opts)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"UnknownType 1\"];\n}",
    to_dot(builtins.unknown_type, &opts)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"NeverType 1\"];\n}",
    to_dot(builtins.never_type, &opts)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_primitive() {
  use ulua_analysis::functions::to_dot_to_dot::to_dot_default_opts_type_id as to_dot;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let builtins = fixture.get_builtins();

  assert_eq!(
    "digraph graphname {\nn1 [label=\"nil\"];\n}",
    to_dot(builtins.nil_type)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"number\"];\n}",
    to_dot(builtins.number_type)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"any\"];\n}",
    to_dot(builtins.any_type)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"unknown\"];\n}",
    to_dot(builtins.unknown_type)
  );
  assert_eq!(
    "digraph graphname {\nn1 [label=\"never\"];\n}",
    to_dot(builtins.never_type)
  );
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_table() {
  use ulua_analysis::{functions::to_dot_to_dot::to_dot, records::to_dot_options::ToDotOptions};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
type A<T, U...> = { x: T, y: (U...) -> (), [string]: any }
local a: A<number, ...string>
"#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let a_ty = fixture.require_type_string("a");
  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"TableType A\"];\nn1 -> n2 [label=\"x\"];\nn2 [label=\"number\"];\nn1 -> n3 [label=\"y\"];\nn3 [label=\"FunctionType 3\"];\nn3 -> n4 [label=\"arg\"];\nn4 [label=\"VariadicTypePack 4\"];\nn4 -> n5;\nn5 [label=\"string\"];\nn3 -> n6 [label=\"ret\"];\nn6 [label=\"TypePack 6\"];\nn1 -> n7 [label=\"[index]\"];\nn7 [label=\"string\"];\nn1 -> n8 [label=\"[value]\"];\nn8 [label=\"any\"];\nn1 -> n9 [label=\"typeParam\"];\nn9 [label=\"number\"];\nn1 -> n4 [label=\"typePackParam\"];\n}",
    to_dot(a_ty, &opts)
  );

  let _ = to_dot(a_ty, &opts);
}

// Source: `tests/ToDot.test.cpp`
#[test]
fn to_dot_union() {
  use ulua_analysis::{functions::to_dot_to_dot::to_dot, records::to_dot_options::ToDotOptions};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
local a: string | number
"#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let opts = ToDotOptions {
    show_pointers: false,
    duplicate_primitives: true,
  };

  assert_eq!(
    "digraph graphname {\nn1 [label=\"UnionType 1\"];\nn1 -> n2;\nn2 [label=\"string\"];\nn1 -> n3;\nn3 [label=\"number\"];\n}",
    to_dot(fixture.require_type_string("a"), &opts)
  );
}
