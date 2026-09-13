extern crate alloc;

mod type_path_index_type_path_test {
  use ulua_analysis::{
    enums::pack_field::PackField,
    functions::{
      to_string_to_string_alt_c::to_string_type_id, traverse_for_type_type_path::traverse_for_type,
    },
    methods::{
      path_builder_args::PathBuilderArgs, path_builder_build::PathBuilderBuild,
      path_builder_index::PathBuilderIndex,
    },
    records::{path::Path, path_builder::PathBuilder, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn type_path_index() {
    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
            type T = number | string | boolean
        "#,
      ),
      None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let string_type = fixture.get_builtins().string_type;
    let builtins = fixture.get_builtins() as *mut _;

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    assert_eq!(
      traverse_for_type(
        root,
        &builder.index(1).build(),
        unsafe { &*builtins },
        &mut arena
      ),
      Some(string_type)
    );

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    assert_eq!(
      traverse_for_type(
        root,
        &builder.index(97).build(),
        unsafe { &*builtins },
        &mut arena
      ),
      None
    );

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
            type T = (() -> ()) & ((true) -> false) & ((false) -> true)
        "#,
      ),
      None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let builtins = fixture.get_builtins() as *mut _;

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    let result = traverse_for_type(
      root,
      &builder.index(1).build(),
      unsafe { &*builtins },
      &mut arena,
    );
    assert!(result.is_some());
    assert_eq!(to_string_type_id(result.unwrap()), "(true) -> false");

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    assert_eq!(
      traverse_for_type(
        root,
        &builder.index(97).build(),
        unsafe { &*builtins },
        &mut arena
      ),
      None
    );

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
            type T = (number, string, true, false) -> ()
        "#,
      ),
      None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let string_type = fixture.get_builtins().string_type;
    let builtins = fixture.get_builtins() as *mut _;

    let mut builder = PathBuilder::new();
    let path = builder.args().index(1).build();
    let mut arena = TypeArena::default();
    assert_eq!(
      traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
      Some(string_type)
    );

    let path = Path::from_components(vec![Component::PackField(PackField::Arguments)]);
    let mut builder = PathBuilder::new();
    let path = path.append(&builder.index(72).build());
    let mut arena = TypeArena::default();
    assert_eq!(
      traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
      None
    );
  }
}

mod type_path_metatable_property {

  use ulua_analysis::{
    functions::traverse_for_type_type_path::traverse_for_type,
    records::{path::Path, property_type_path::Property, type_arena::TypeArena},
    type_aliases::component::Component,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  #[cfg(test)]
  fn check_case(source: &str) {
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options(&String::from(source), None);
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.base.require_type_string(&String::from("x"));
    let number_type = fixture.base.get_builtins().number_type;
    let builtins = fixture.base.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    let path = Path::from_component(Component::Property(Property::property_string_bool(
      "x", true,
    )));

    assert_eq!(
      traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
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

mod type_path_pack_slice_type_path_test {
  use ulua_analysis::{
    functions::{to_string_human::to_string_human, to_string_type_path::to_string},
    methods::path_builder_build::PathBuilderBuild,
    records::path_builder::PathBuilder,
  };

  #[test]
  fn type_path_pack_slice() {
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
}
