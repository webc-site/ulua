extern crate alloc;

#[test]
fn type_path_index() {
  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string::to_string_type_id, traverse_for_type_type_path::traverse_for_type,
    },
    methods::{
      path_builder_args::PathBuilderArgs, path_builder_build::PathBuilderBuild,
      path_builder_index::PathBuilderIndex,
    },
    records::{path::Path, path_builder::PathBuilder, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = number | string | boolean
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let string_type = fixture.get_builtins().string_type;
  let builtins = fixture.get_builtins();

  let mut builder = PathBuilder::new();
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(root, &builder.index(1).build(), builtins, &mut arena),
    Some(string_type)
  );

  let mut builder = PathBuilder::new();
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(root, &builder.index(97).build(), builtins, &mut arena),
    None
  );

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (() -> ()) & ((true) -> false) & ((false) -> true)
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();

  let mut builder = PathBuilder::new();
  let mut arena = TypeArena::default();
  let result = traverse_for_type(root, &builder.index(1).build(), builtins, &mut arena);
  assert!(result.is_some());
  assert_eq!(to_string_type_id(result.unwrap()), "(true) -> false");

  let mut builder = PathBuilder::new();
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(root, &builder.index(97).build(), builtins, &mut arena),
    None
  );

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (number, string, true, false) -> ()
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let string_type = fixture.get_builtins().string_type;
  let builtins = fixture.get_builtins();

  let mut builder = PathBuilder::new();
  let path = builder.args().index(1).build();
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(root, &path, builtins, &mut arena),
    Some(string_type)
  );

  let path = Path::from_components(vec![Component::PackField(PackField::Arguments)]);
  let mut builder = PathBuilder::new();
  let path = path.append(&builder.index(72).build());
  let mut arena = TypeArena::default();
  assert_eq!(traverse_for_type(root, &path, builtins, &mut arena), None);
}

mod type_path_metatable_property {

  use ulua_analysis::{
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, property_type_path::Property, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  fn check_case(source: &str) {
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options(source, None);
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.base.require_type_string("x");
    let number_type = fixture.base.get_builtins().number_type;
    let builtins = fixture.base.get_builtins();
    let mut arena = TypeArena::default();
    let path = Path::from_component(Component::Property(Property::property_string_bool(
      "x", true,
    )));

    assert_eq!(
      traverse_for_type(root, &path, builtins, &mut arena),
      Some(number_type)
    );
  }

  #[test]
  fn type_path_metatable_property() {
    check_case(
      r#"
            local x = setmetatable({ x = 123 }, {})
        "#,
    );

    check_case(
      r#"
            local x = setmetatable({ x = 123 }, { __index = { x = 'foo' } })
        "#,
    );

    check_case(
      r#"
            local x = setmetatable({}, { __index = { x = 123 } })
        "#,
    );
  }
}

#[test]
fn type_path_pack_slice() {
  use ulua_analysis::{
    functions::{to_string_human::to_string_human, to_string_type_path::to_string},
    methods::path_builder_build::PathBuilderBuild,
    records::path_builder::PathBuilder,
  };

  let mut string_builder = PathBuilder::new();
  assert_eq!(
    to_string(&string_builder.pack_slice(1).build(), false),
    "[1:]"
  );

  let mut human_builder = PathBuilder::new();
  assert_eq!(
    to_string_human(&human_builder.pack_slice(1).build()),
    "the portion of the type pack starting at index 1 to the end"
  );
}

// Source: `tests/TypePath.test.cpp:43`
#[test]
fn type_path_append() {
  use alloc::vec;

  use ulua_analysis::{
    enums::{pack_field::PackField, type_field::TypeField},
    records::path::Path,
    type_aliases::component::Component,
  };

  // empty_paths
  let p = Path::default();
  assert!(p.append(&Path::default()).path_empty());

  // empty_path_with_path
  let p1 = Path::default();
  let p2 = Path::from_component(Component::TypeField(TypeField::Metatable));
  assert_eq!(
    p1.append(&p2),
    Path::from_component(Component::TypeField(TypeField::Metatable))
  );

  // two_paths
  let p1 = Path::from_component(Component::TypeField(TypeField::IndexLookup));
  let p2 = Path::from_component(Component::TypeField(TypeField::Metatable));
  assert_eq!(
    p1.append(&p2),
    Path::from_components(vec![
      Component::TypeField(TypeField::IndexLookup),
      Component::TypeField(TypeField::Metatable),
    ])
  );

  // all_components
  let p1 = Path::from_components(vec![
    Component::TypeField(TypeField::IndexLookup),
    Component::TypeField(TypeField::Metatable),
  ]);
  let p2 = Path::from_components(vec![
    Component::TypeField(TypeField::Metatable),
    Component::PackField(PackField::Arguments),
  ]);
  assert_eq!(
    p1.append(&p2),
    Path::from_components(vec![
      Component::TypeField(TypeField::IndexLookup),
      Component::TypeField(TypeField::Metatable),
      Component::TypeField(TypeField::Metatable),
      Component::PackField(PackField::Arguments),
    ])
  );

  // does_not_mutate
  let p1 = Path::from_component(Component::TypeField(TypeField::IndexLookup));
  let p2 = Path::from_component(Component::TypeField(TypeField::Metatable));
  let _ = p1.append(&p2);
  assert_eq!(
    p1,
    Path::from_component(Component::TypeField(TypeField::IndexLookup))
  );
  assert_eq!(
    p2,
    Path::from_component(Component::TypeField(TypeField::Metatable))
  );
}

// Source: `tests/TypePath.test.cpp:89`
#[test]
fn type_path_push() {
  use ulua_analysis::{
    enums::type_field::TypeField, records::path::Path, type_aliases::component::Component,
  };

  let p = Path::default();
  let result = p.push(Component::TypeField(TypeField::Metatable));

  assert!(p.path_empty());
  assert_eq!(
    result,
    Path::from_component(Component::TypeField(TypeField::Metatable))
  );
}

// Source: `tests/TypePath.test.cpp:98`
#[test]
fn type_path_pop() {
  use ulua_analysis::records::path::Path;

  // empty_path
  let p = Path::default();
  assert!(p.path_empty());
  assert!(p.pop().path_empty());
}

// Source: `tests/TypePath.test.cpp:119`
#[test]
fn type_path_empty_traversal() {
  use ulua_analysis::{
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, type_arena::TypeArena},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let number_type = fixture.get_builtins().number_type;
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  assert_eq!(
    traverse_for_type(number_type, &Path::default(), builtins, &mut arena),
    Some(number_type)
  );
}

// Source: `tests/TypePath.test.cpp:124`
#[test]
fn type_path_table_property() {
  use ulua_analysis::{
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, property_type_path::Property, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            local x = { y = 123 }
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_string("x");
  let number_type = fixture.get_builtins().number_type;
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  assert_eq!(
    traverse_for_type(
      root,
      &Path::from_component(Component::Property(Property::property_string_bool(
        "y", true
      ))),
      builtins,
      &mut arena
    ),
    Some(number_type)
  );
}

// Source: `tests/TypePath.test.cpp:133`
#[test]
fn type_path_class_property() {
  use ulua_analysis::{
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, property_type_path::Property, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

  // Force this here because vector2InstanceType won't get initialized until the frontend has been forced
  let mut fixture = ExternTypeFixture::default();
  fixture.get_frontend();
  let vector2_instance_type = fixture.vector2_instance_type;
  let number_type = fixture.base.base.get_builtins().number_type;
  let builtins = fixture.base.base.get_builtins();
  let mut arena = TypeArena::default();

  assert_eq!(
    traverse_for_type(
      vector2_instance_type,
      &Path::from_component(Component::Property(Property::property_string_bool(
        "X", true
      ))),
      builtins,
      &mut arena
    ),
    Some(number_type)
  );
}

// Source: `tests/TypePath.test.cpp:232`
#[test]
fn type_path_metatables() {
  use ulua_analysis::{
    enums::type_field::TypeField,
    functions::{
      follow_type, get_metatable_type::get_metatable_type_id_not_null_builtin_types,
      traverse_for_type_type_path::traverse_for_type,
    },
    records::{path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

  let mut fixture = ExternTypeFixture::default();
  fixture.get_frontend();
  let vector2_instance_type = fixture.vector2_instance_type;

  // string
  let string_type = fixture.base.base.get_builtins().string_type;
  let builtins = fixture.base.base.get_builtins() as *mut _;
  let mut arena = TypeArena::default();
  let string_metatable =
    get_metatable_type_id_not_null_builtin_types(string_type, unsafe { &*builtins });
  assert_eq!(
    traverse_for_type(
      string_type,
      &Path::from_component(Component::TypeField(TypeField::Metatable)),
      unsafe { &*builtins },
      &mut arena
    ),
    string_metatable
  );

  // string_singleton
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
            type T = "foo"
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);
  let root = fixture.base.base.require_type_alias("T");
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      root,
      &Path::from_component(Component::TypeField(TypeField::Metatable)),
      unsafe { &*builtins },
      &mut arena
    ),
    string_metatable
  );

  // table
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
            type Table = { foo: number }
            type Metatable = { bar: number }
            local tbl: Table = { foo = 123 }
            local mt: Metatable = { bar = 456 }
            local res = setmetatable(tbl, mt)
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);
  // Tricky test setup because 'setmetatable' mutates the argument 'tbl' type
  let res_ty = fixture.base.base.require_type_string("res");
  let expected = fixture
    .base
    .base
    .lookup_type("Table")
    .expect("Table should be resolvable");
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      res_ty,
      &Path::from_component(Component::TypeField(TypeField::Table)),
      unsafe { &*builtins },
      &mut arena
    ),
    Some(follow_type::follow(expected))
  );

  // metatable
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
            local mt = { foo = 123 }
            local tbl = setmetatable({}, mt)
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);
  let tbl_ty = fixture.base.base.require_type_string("tbl");
  let mt_ty = fixture.base.base.require_type_string("mt");
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      tbl_ty,
      &Path::from_component(Component::TypeField(TypeField::Metatable)),
      unsafe { &*builtins },
      &mut arena
    ),
    Some(mt_ty)
  );

  // class: ExternTypeFixture's Vector2 metatable is just an empty table, but it's there.
  let mut arena = TypeArena::default();
  let result = traverse_for_type(
    vector2_instance_type,
    &Path::from_component(Component::TypeField(TypeField::Metatable)),
    unsafe { &*builtins },
    &mut arena,
  );
  assert!(result.is_some());
}

// Source: `tests/TypePath.test.cpp:289`
#[test]
fn type_path_bounds() {
  use ulua_analysis::{
    enums::type_field::TypeField,
    functions::{get_mutable_type, traverse_for_type_type_path::traverse_for_type},
    records::{free_type::FreeType, path::Path, scope::Scope, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let number_type = fixture.get_builtins().number_type;
  let boolean_type = fixture.get_builtins().boolean_type;
  let any_type_pack = fixture.get_builtins().any_type_pack;
  let builtins = fixture.get_builtins();

  // free_type / upper
  {
    let mut scope = Scope::scope_type_pack_id(any_type_pack);
    let mut arena = TypeArena::default();
    let ty = arena.fresh_type_not_null_builtin_types_scope(builtins, &mut scope);
    let ft = get_mutable_type::get_mutable::<FreeType>(ty).expect("fresh type is a FreeType");
    ft.upper_bound = number_type;

    assert_eq!(
      traverse_for_type(
        ty,
        &Path::from_component(Component::TypeField(TypeField::UpperBound)),
        builtins,
        &mut arena
      ),
      Some(number_type)
    );
  }

  // free_type / lower
  {
    let mut scope = Scope::scope_type_pack_id(any_type_pack);
    let mut arena = TypeArena::default();
    let ty = arena.fresh_type_not_null_builtin_types_scope(builtins, &mut scope);
    let ft = get_mutable_type::get_mutable::<FreeType>(ty).expect("fresh type is a FreeType");
    ft.lower_bound = boolean_type;

    assert_eq!(
      traverse_for_type(
        ty,
        &Path::from_component(Component::TypeField(TypeField::LowerBound)),
        builtins,
        &mut arena
      ),
      Some(boolean_type)
    );
  }

  // unbounded_type
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      number_type,
      &Path::from_component(Component::TypeField(TypeField::UpperBound)),
      builtins,
      &mut arena
    ),
    None
  );
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      number_type,
      &Path::from_component(Component::TypeField(TypeField::LowerBound)),
      builtins,
      &mut arena
    ),
    None
  );
}

// Source: `tests/TypePath.test.cpp:321`
#[test]
fn type_path_indexers() {
  use ulua_analysis::{
    enums::type_field::TypeField,
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  // lookup_indexer
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = { [string]: boolean }
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let string_type = fixture.get_builtins().string_type;
  let boolean_type = fixture.get_builtins().boolean_type;
  let builtins = fixture.get_builtins();

  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      root,
      &Path::from_component(Component::TypeField(TypeField::IndexLookup)),
      builtins,
      &mut arena
    ),
    Some(string_type)
  );
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      root,
      &Path::from_component(Component::TypeField(TypeField::IndexResult)),
      builtins,
      &mut arena
    ),
    Some(boolean_type)
  );

  // no_indexer
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = { y: number }
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();

  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      root,
      &Path::from_component(Component::TypeField(TypeField::IndexLookup)),
      builtins,
      &mut arena
    ),
    None
  );
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      root,
      &Path::from_component(Component::TypeField(TypeField::IndexResult)),
      builtins,
      &mut arena
    ),
    None
  );
}

// Source: `tests/TypePath.test.cpp:355`
#[test]
fn type_path_negated() {
  use ulua_analysis::{
    enums::type_field::TypeField,
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{negation_type::NegationType, path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let number_type = fixture.get_builtins().number_type;
  let builtins = fixture.get_builtins();

  // valid
  let mut arena = TypeArena::default();
  let negated = arena.add_type(NegationType::new(number_type));
  assert_eq!(
    traverse_for_type(
      negated,
      &Path::from_component(Component::TypeField(TypeField::Negated)),
      builtins,
      &mut arena
    ),
    Some(number_type)
  );

  // not_negation
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      number_type,
      &Path::from_component(Component::TypeField(TypeField::Negated)),
      builtins,
      &mut arena
    ),
    None
  );
}

// Source: `tests/TypePath.test.cpp:374`
// Rust 侧 traverse_for_type 没有 TypePackId 入口，valid 分支改用
// `Arguments -> Tail -> Variadic` 链覆盖 Variadic 步进。
#[test]
fn type_path_variadic() {
  use alloc::vec;

  use ulua_analysis::{
    enums::{pack_field::PackField, type_field::TypeField},
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  // valid
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (number, ...boolean) -> ()
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();
  let boolean_type = builtins.boolean_type;

  let mut arena = TypeArena::default();
  let path = Path::from_components(vec![
    Component::PackField(PackField::Arguments),
    Component::PackField(PackField::Tail),
    Component::TypeField(TypeField::Variadic),
  ]);
  assert_eq!(
    traverse_for_type(root, &path, builtins, &mut arena),
    Some(boolean_type)
  );

  // not_variadic
  let number_type = builtins.number_type;
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(
      number_type,
      &Path::from_component(Component::TypeField(TypeField::Variadic)),
      builtins,
      &mut arena
    ),
    None
  );
}

// Source: `tests/TypePath.test.cpp:393`
#[test]
fn type_path_arguments() {
  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string::to_string_type_pack_id, traverse_for_pack_type_path::traverse_for_pack,
    },
    records::{path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  // function
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            function f(x: number, y: string)
            end
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_string("f");
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  let pack = traverse_for_pack(
    root,
    &Path::from_component(Component::PackField(PackField::Arguments)),
    builtins,
    &mut arena,
  );
  assert!(pack.is_some());
  assert_eq!(
    to_string_type_pack_id(pack.expect("pack is_some above")),
    "number, string"
  );

  // not_function
  let boolean_type = builtins.boolean_type;
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_pack(
      boolean_type,
      &Path::from_component(Component::PackField(PackField::Arguments)),
      builtins,
      &mut arena
    ),
    None
  );
}

// Source: `tests/TypePath.test.cpp:415`
#[test]
fn type_path_returns() {
  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string::to_string_type_pack_id, traverse_for_pack_type_path::traverse_for_pack,
    },
    records::{path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  // function
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            function f(): (number, string)
                return 123, "foo"
            end
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_string("f");
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  let pack = traverse_for_pack(
    root,
    &Path::from_component(Component::PackField(PackField::Returns)),
    builtins,
    &mut arena,
  );
  assert!(pack.is_some());
  assert_eq!(
    to_string_type_pack_id(pack.expect("pack is_some above")),
    "number, string"
  );

  // not_function
  let boolean_type = builtins.boolean_type;
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_pack(
      boolean_type,
      &Path::from_component(Component::PackField(PackField::Returns)),
      builtins,
      &mut arena
    ),
    None
  );
}

// Source: `tests/TypePath.test.cpp:438`
#[test]
fn type_path_tail() {
  use alloc::vec;

  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string::to_string_type_pack_id, traverse_for_pack_type_path::traverse_for_pack,
    },
    records::{path::Path, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  // has_tail
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (number, string, ...boolean) -> ()
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  let path = Path::from_components(vec![
    Component::PackField(PackField::Arguments),
    Component::PackField(PackField::Tail),
  ]);
  let pack = traverse_for_pack(root, &path, builtins, &mut arena);
  assert!(pack.is_some());
  assert_eq!(
    to_string_type_pack_id(pack.expect("pack is_some above")),
    "...boolean"
  );

  // finite_pack
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (number, string) -> ()
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();
  assert_eq!(traverse_for_pack(root, &path, builtins, &mut arena), None);

  // type
  let string_type = builtins.string_type;
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_pack(string_type, &path, builtins, &mut arena),
    None
  );
}

// Source: `tests/TypePath.test.cpp:469`
#[test]
fn type_path_pack_slice_has_tail() {
  use alloc::vec;

  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string::to_string_type_pack_id, traverse_for_pack_type_path::traverse_for_pack,
    },
    methods::path_builder_build::PathBuilderBuild,
    records::{path::Path, path_builder::PathBuilder, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (number, string, ...boolean) -> ()
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  let mut builder = PathBuilder::new();
  let path = Path::from_components(vec![Component::PackField(PackField::Arguments)])
    .append(&builder.pack_slice(1).build());
  let pack = traverse_for_pack(root, &path, builtins, &mut arena);
  assert!(pack.is_some());
  assert_eq!(
    to_string_type_pack_id(pack.expect("pack is_some above")),
    "string, ...boolean"
  );
}

// Source: `tests/TypePath.test.cpp:484`
#[test]
fn type_path_pack_slice_finite_pack() {
  use alloc::vec;

  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string::to_string_type_pack_id, traverse_for_pack_type_path::traverse_for_pack,
    },
    methods::path_builder_build::PathBuilderBuild,
    records::{path::Path, path_builder::PathBuilder, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = (number, string) -> ()
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();
  let mut arena = TypeArena::default();

  let mut builder = PathBuilder::new();
  let path = Path::from_components(vec![Component::PackField(PackField::Arguments)])
    .append(&builder.pack_slice(1).build());
  let pack = traverse_for_pack(root, &path, builtins, &mut arena);
  assert!(pack.is_some());
  assert_eq!(
    to_string_type_pack_id(pack.expect("pack is_some above")),
    "string"
  );
}

// Source: `tests/TypePath.test.cpp:499`
#[test]
fn type_path_pack_slice_type() {
  use alloc::vec;

  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::traverse_for_pack_type_path::traverse_for_pack,
    methods::path_builder_build::PathBuilderBuild,
    records::{path::Path, path_builder::PathBuilder, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let string_type = fixture.get_builtins().string_type;
  let builtins = fixture.get_builtins();

  let mut builder = PathBuilder::new();
  let path = Path::from_components(vec![Component::PackField(PackField::Arguments)])
    .append(&builder.pack_slice(1).build());
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_pack(string_type, &path, builtins, &mut arena),
    None
  );
}

// Source: `tests/TypePath.test.cpp:508`
// bound_cycle 子用例依赖 TypePackId 根遍历与 CHECK_THROWS 语义，暂不移植。
#[test]
fn type_path_cycles_table_contains_itself() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::{get_mutable_type, traverse_for_type_type_path::traverse_for_type},
    records::{
      path::Path, property_type::Property as TableProperty, property_type_path::Property,
      table_type::TableType, type_arena::TypeArena,
    },
    type_aliases::{component::Component, props_type::Props},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let builtins = fixture.get_builtins();

  let mut arena = TypeArena::default();
  let tbl = arena.add_type(TableType::new());
  let tbl_mut = get_mutable_type::get_mutable::<TableType>(tbl).expect("tbl is a TableType");
  let mut props = Props::new();
  props.insert(String::from("a"), TableProperty::readonly(tbl));
  tbl_mut.props = props;

  let result = traverse_for_type(
    tbl,
    &Path::from_component(Component::Property(Property::property_string_bool(
      "a", true,
    ))),
    builtins,
    &mut arena,
  );
  assert_eq!(result, Some(tbl));
}

// Source: `tests/TypePath.test.cpp:537`
#[test]
fn type_path_step_limit() {
  use ulua_analysis::{
    functions::{
      to_string_to_string::to_string_type_id, traverse_for_type_type_path::traverse_for_type,
    },
    methods::path_builder_build::PathBuilderBuild,
    records::{path_builder::PathBuilder, type_arena::TypeArena},
  };
  use ulua_common::dfint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  let _limit = ScopedFastInt::new(&dfint::LuauTypePathMaximumTraverseSteps, 2);

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
            type T = {
                x: {
                    y: {
                        z: number
                    }
                }
            }
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.require_type_alias("T");
  let builtins = fixture.get_builtins();
  let mut builder = PathBuilder::new();
  let path = builder.read_prop("x").read_prop("y").read_prop("z").build();
  let mut arena = TypeArena::default();
  let traversed = traverse_for_type(root, &path, builtins, &mut arena);
  assert!(
    traversed.is_none(),
    "traversal should hit the step limit, got {:?}",
    traversed.map(to_string_type_id)
  );
}

// Source: `tests/TypePath.test.cpp:557`
#[test]
fn type_path_complex_chains() {
  use ulua_analysis::{
    functions::{
      to_string_to_string::to_string_type_id, traverse_for_type_type_path::traverse_for_type,
    },
    methods::{
      path_builder_args::PathBuilderArgs, path_builder_build::PathBuilderBuild,
      path_builder_index::PathBuilderIndex, path_builder_mt::PathBuilderMt,
      path_builder_rets::PathBuilderRets,
    },
    records::{path_builder::PathBuilder, type_arena::TypeArena},
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  // add_metamethod_return_type
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            type Meta = {
                __add: (Tab, Tab) -> number,
            }

            type Tab = typeof(setmetatable({}, {} :: Meta))
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.base.require_type_alias("Tab");
  let number_type = fixture.base.get_builtins().number_type;
  let builtins = fixture.base.get_builtins();

  let mut builder = PathBuilder::new();
  let path = builder.mt().read_prop("__add").rets().index(0).build();
  let mut arena = TypeArena::default();
  assert_eq!(
    traverse_for_type(root, &path, builtins, &mut arena),
    Some(number_type)
  );

  // overloaded_fn_overload_one_argument_two
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            type Obj = {
                method: ((true, false) -> string) & ((string) -> number)
            }
        "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let root = fixture.base.require_type_alias("Obj");
  let false_type = fixture.base.get_builtins().false_type;
  let builtins = fixture.base.get_builtins();

  let mut builder = PathBuilder::new();
  let path = builder.read_prop("method").index(0).args().index(1).build();
  let mut arena = TypeArena::default();
  let traversed = traverse_for_type(root, &path, builtins, &mut arena);
  assert_eq!(traversed, Some(false_type));
  // cpp 仅断言 `*result == falseType`；上一 assert_eq 已覆盖，此处顺带校验字符串化
  assert_eq!(
    to_string_type_id(traversed.expect("traversed is_some above")),
    "false"
  );
}

// Source: `tests/TypePath.test.cpp:594`
#[test]
fn type_path_to_string_field() {
  use ulua_analysis::{
    functions::to_string_type_path::to_string, methods::path_builder_build::PathBuilderBuild,
    records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  assert_eq!(
    to_string(&builder.prop("foo").build(), false),
    "[read \"foo\"]"
  );
}

// Source: `tests/TypePath.test.cpp:599`
#[test]
fn type_path_to_string_index() {
  use ulua_analysis::{
    functions::to_string_type_path::to_string,
    methods::{path_builder_build::PathBuilderBuild, path_builder_index::PathBuilderIndex},
    records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  assert_eq!(to_string(&builder.index(0).build(), false), "[0]");
}

// Source: `tests/TypePath.test.cpp:604`
#[test]
fn type_path_to_string_chain() {
  use ulua_analysis::{
    functions::to_string_type_path::to_string,
    methods::{
      path_builder_build::PathBuilderBuild, path_builder_index::PathBuilderIndex,
      path_builder_mt::PathBuilderMt,
    },
    records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  assert_eq!(
    to_string(&builder.index(0).mt().build(), false),
    "[0].metatable()"
  );
}

// Source: `tests/TypePath.test.cpp:609`
// Rust 侧只移植了旧版 `toStringHuman_DEPRECATED` 消息分支。
#[test]
fn type_path_human_property_then_metatable_portion() {
  use ulua_analysis::{
    functions::to_string_human::to_string_human,
    methods::{path_builder_build::PathBuilderBuild, path_builder_mt::PathBuilderMt},
    records::path_builder::PathBuilder,
  };

  let mut read_builder = PathBuilder::new();
  assert_eq!(
    to_string_human(&read_builder.read_prop("a").mt().build()),
    "accessing `a` has the metatable portion as "
  );

  let mut write_builder = PathBuilder::new();
  assert_eq!(
    to_string_human(&write_builder.write_prop("a").mt().build()),
    "writing to `a` has the metatable portion as "
  );
}

// Source: `tests/TypePath.test.cpp:935`
#[test]
fn type_path_builder_empty_path() {
  use ulua_analysis::{
    methods::path_builder_build::PathBuilderBuild, records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  assert!(builder.build().path_empty());
}

// Source: `tests/TypePath.test.cpp:941`
#[test]
fn type_path_builder_prop() {
  use ulua_analysis::{
    methods::path_builder_build::PathBuilderBuild,
    records::{path::Path, path_builder::PathBuilder, property_type_path::Property},
    type_aliases::component::Component,
  };

  let mut builder = PathBuilder::new();
  let p = builder.prop("foo").build();
  assert_eq!(
    p,
    Path::from_component(Component::Property(Property::property_string_bool(
      "foo", true
    )))
  );
}

// Source: `tests/TypePath.test.cpp:947`
#[test]
fn type_path_builder_read_prop() {
  use ulua_analysis::{
    methods::path_builder_build::PathBuilderBuild,
    records::{path::Path, path_builder::PathBuilder, property_type_path::Property},
    type_aliases::component::Component,
  };

  let mut builder = PathBuilder::new();
  let p = builder.read_prop("foo").build();
  assert_eq!(
    p,
    Path::from_component(Component::Property(Property::property_string_bool(
      "foo", true
    )))
  );
}

// Source: `tests/TypePath.test.cpp:953`
#[test]
fn type_path_builder_write_prop() {
  use ulua_analysis::{
    methods::path_builder_build::PathBuilderBuild,
    records::{path::Path, path_builder::PathBuilder, property_type_path::Property},
    type_aliases::component::Component,
  };

  let mut builder = PathBuilder::new();
  let p = builder.write_prop("foo").build();
  assert_eq!(
    p,
    Path::from_component(Component::Property(Property::property_string_bool(
      "foo", false
    )))
  );
}

// Source: `tests/TypePath.test.cpp:959`
#[test]
fn type_path_builder_index() {
  use ulua_analysis::{
    functions::to_string_type_path::to_string,
    methods::{path_builder_build::PathBuilderBuild, path_builder_index::PathBuilderIndex},
    records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  let p = builder.index(0).build();
  assert_eq!(p.components.len(), 1);
  assert_eq!(to_string(&p, false), "[0]");
}

// Source: `tests/TypePath.test.cpp:965`
#[test]
fn type_path_builder_fields() {
  use ulua_analysis::{
    enums::{pack_field::PackField, type_field::TypeField},
    methods::{
      path_builder_args::PathBuilderArgs, path_builder_build::PathBuilderBuild,
      path_builder_lb::PathBuilderLb, path_builder_mt::PathBuilderMt,
      path_builder_negated::PathBuilderNegated, path_builder_rets::PathBuilderRets,
      path_builder_tail::PathBuilderTail, path_builder_ub::PathBuilderUb,
      path_builder_variadic::PathBuilderVariadic,
    },
    records::{path::Path, path_builder::PathBuilder},
    type_aliases::component::Component,
  };

  fn check<F: FnOnce(&mut PathBuilder) -> Path>(render: F, expected: Path) {
    let mut builder = PathBuilder::new();
    assert_eq!(render(&mut builder), expected);
  }

  check(
    |b| b.mt().build(),
    Path::from_component(Component::TypeField(TypeField::Metatable)),
  );
  check(
    |b| b.lb().build(),
    Path::from_component(Component::TypeField(TypeField::LowerBound)),
  );
  check(
    |b| b.ub().build(),
    Path::from_component(Component::TypeField(TypeField::UpperBound)),
  );
  check(
    |b| b.index_key().build(),
    Path::from_component(Component::TypeField(TypeField::IndexLookup)),
  );
  check(
    |b| b.index_value().build(),
    Path::from_component(Component::TypeField(TypeField::IndexResult)),
  );
  check(
    |b| b.negated().build(),
    Path::from_component(Component::TypeField(TypeField::Negated)),
  );
  check(
    |b| b.variadic().build(),
    Path::from_component(Component::TypeField(TypeField::Variadic)),
  );
  check(
    |b| b.args().build(),
    Path::from_component(Component::PackField(PackField::Arguments)),
  );
  check(
    |b| b.rets().build(),
    Path::from_component(Component::PackField(PackField::Returns)),
  );
  check(
    |b| b.tail().build(),
    Path::from_component(Component::PackField(PackField::Tail)),
  );
}

// Source: `tests/TypePath.test.cpp:979`
#[test]
fn type_path_builder_chained() {
  use ulua_analysis::{
    functions::to_string_type_path::to_string,
    methods::{
      path_builder_args::PathBuilderArgs, path_builder_build::PathBuilderBuild,
      path_builder_index::PathBuilderIndex, path_builder_mt::PathBuilderMt,
    },
    records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  let path = builder
    .index(0)
    .read_prop("foo")
    .mt()
    .read_prop("bar")
    .args()
    .index(1)
    .build();

  assert_eq!(path.components.len(), 6);
  assert_eq!(
    to_string(&path, false),
    "[0][read \"foo\"].metatable()[read \"bar\"].arguments()[1]"
  );
}

// Source: `tests/TypePath.test.cpp:988`
#[test]
fn type_path_builder_pack_slice() {
  use ulua_analysis::{
    functions::to_string_type_path::to_string, methods::path_builder_build::PathBuilderBuild,
    records::path_builder::PathBuilder,
  };

  let mut builder = PathBuilder::new();
  let p = builder.pack_slice(3).build();
  assert_eq!(p.components.len(), 1);
  assert_eq!(to_string(&p, false), "[3:]");
}

// 缺口（未移植，对照 `tests/TypePath.test.cpp`，共 8 例）：
// - human_paths_through_properties（:626）、human_single_components（:718）、
//   human_paths_through_type_components（:745）、human_paths_through_function_packs
//   （:802）——依赖 `renderTypePathPrefix` + FFlag `LuauNewTypePathErrorMessages`
//   新报错渲染系统，本移植未实现（仅有 to_string_human/DEPRECATED 旧口）。
// - structured_human_paths（:658）、render_options_handle_cyclic_return_packs
//   （:702）、human_reduction_and_mapped_pack_paths（:884）——依赖
//   `RenderedTypePath`/`TypePathRenderMetadata`/`deriveRenderMetadata`，同上未实现。
// - cycles（:508）bound_cycle 子用例——cpp CHECK_THROWS 期望遍历抛
//   InternalTypeErrorException；Rust traverse 对环以步限返回 None（见本文件
//   :1013 既有注记），faithful 断言不可表达；table_contains_itself 子用例已移植。
