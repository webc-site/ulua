extern crate alloc;

mod subtyping_a_a_a_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_a_a_a() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");

    let generic_as_to_as_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);
    let nothing_to_nothing_ty =
      fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![]);

    assert!(
      fixture
        .is_subtype_type_id_type_id(generic_as_to_as_ty, nothing_to_nothing_ty)
        .is_subtype()
    );
  }
}

mod subtyping_a_a_number_number_number_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_a_a_number_number_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let generic_as = fixture.generic_pack("A");
    let number_pack = fixture.pack_initializer_list_type_id(vec![number_ty]);

    let generic_as_to_number_ty =
      fixture.generic_pack_fn(vec![generic_as], generic_as, number_pack);
    let number_to_number_ty = fixture
      .fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![number_ty]);

    assert!(
      fixture
        .is_subtype_type_id_type_id(generic_as_to_number_ty, number_to_number_ty)
        .is_subtype()
    );
  }
}

mod subtyping_a_a_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_a_a() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");

    let generic_nothing_to_as_ty = fixture.generic_pack_fn(
      vec![generic_as],
      fixture.builtin_types.empty_type_pack,
      generic_as,
    );
    let nothing_to_nothing_ty =
      fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![]);

    assert!(
      fixture
        .is_subtype_type_id_type_id(generic_nothing_to_as_ty, nothing_to_nothing_ty)
        .is_subtype()
    );
  }
}

mod subtyping_a_b_string_lower_string_subtyping_test {
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_a_b_string_lower_string() {
    let mut fixture = SubtypeFixture::default();
    let a_ty = fixture.str("a");
    let b_ty = fixture.str("b");
    let string_ty = fixture.builtin_types.string_type;

    let not_a = fixture.negate(a_ty);
    let not_b = fixture.negate(b_ty);
    let not_a_and_not_b = fixture.meet(not_a, not_b);
    let sub_ty = fixture.meet(not_a_and_not_b, string_ty);
    let table_with_lower = fixture.table_with_lower();

    assert!(
      fixture
        .is_subtype_type_id_type_id(sub_ty, table_with_lower)
        .is_subtype()
    );
  }
}

mod subtyping_child_another_child_userdata_subtyping_test {
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_child_another_child_userdata() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let extern_ty = fixture.builtin_types.extern_type;
    let left = fixture.join(hierarchy.child_class, hierarchy.another_child_class);

    assert!(
      fixture
        .is_subtype_type_id_type_id(left, extern_ty)
        .is_subtype()
    );
  }
}

mod subtyping_child_root_userdata_subtyping_test {
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_child_root_userdata() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let extern_ty = fixture.builtin_types.extern_type;
    let left = fixture.meet(hierarchy.child_class, hierarchy.root_class);

    assert!(
      fixture
        .is_subtype_type_id_type_id(left, extern_ty)
        .is_subtype()
    );
  }
}

mod subtyping_hello_world_number_subtyping_test {
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_hello_world_number() {
    let mut fixture = SubtypeFixture::default();
    let hello_ty = fixture.str("hello");
    let world_ty = fixture.str("world");
    let hello_or_world_ty = fixture.join(hello_ty, world_ty);
    let number_ty = fixture.builtin_types.number_type;

    assert!(
      !fixture
        .is_subtype_type_id_type_id(hello_or_world_ty, number_ty)
        .is_subtype()
    );
  }
}

mod subtyping_number_number_a_a_number_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_number_a_a_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let generic_as = fixture.generic_pack("A");
    let number_pack = fixture.pack_initializer_list_type_id(vec![number_ty]);

    let number_to_number_ty = fixture
      .fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![number_ty]);
    let generic_as_to_number_ty =
      fixture.generic_pack_fn(vec![generic_as], generic_as, number_pack);

    assert!(
      !fixture
        .is_subtype_type_id_type_id(number_to_number_ty, generic_as_to_number_ty)
        .is_subtype()
    );
  }
}

mod subtyping_number_number_subtyping_test {
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;

    assert!(
      fixture
        .is_subtype_type_id_type_id(number_ty, number_ty)
        .is_subtype()
    );
  }
}

mod subtyping_number_string_number_string_string_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_string_number_string_string() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let string_ty = fixture.builtin_types.string_type;

    let number_to_string_ty = fixture
      .fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![string_ty]);
    let number_to_two_strings_ty = fixture
      .fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![string_ty, string_ty],
      );

    assert!(
      !fixture
        .is_subtype_type_id_type_id(number_to_string_ty, number_to_two_strings_ty)
        .is_subtype()
    );
  }
}

mod subtyping_number_string_string_number_string_string_subtyping_test {
  use alloc::vec;

  use ulua_analysis::{
    records::variadic_type_pack::VariadicTypePack, type_aliases::type_pack_variant::TypePackVariant,
  };
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_string_string_number_string_string() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let string_ty = fixture.builtin_types.string_type;
    let optional_string_ty = fixture.builtin_types.optional_string_type;

    let number_and_optional_strings_to_string_ty = fixture
      .fn_item_initializer_list_type_id_type_pack_variant_initializer_list_type_id(
        vec![number_ty],
        TypePackVariant::Variadic(VariadicTypePack::new(optional_string_ty)),
        vec![string_ty],
      );
    let number_and_strings_to_string_ty = fixture
      .fn_item_initializer_list_type_id_type_pack_variant_initializer_list_type_id(
        vec![number_ty],
        TypePackVariant::Variadic(VariadicTypePack::new(string_ty)),
        vec![string_ty],
      );

    assert!(
      fixture
        .is_subtype_type_id_type_id(
          number_and_optional_strings_to_string_ty,
          number_and_strings_to_string_ty,
        )
        .is_subtype()
    );
  }
}

mod subtyping_number_string_string_number_string_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_string_string_number_string() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let string_ty = fixture.builtin_types.string_type;

    let number_to_two_strings_ty = fixture
      .fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![string_ty, string_ty],
      );
    let number_to_string_ty = fixture
      .fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![string_ty]);

    assert!(
      !fixture
        .is_subtype_type_id_type_id(number_to_two_strings_ty, number_to_string_ty)
        .is_subtype()
    );
  }
}

mod subtyping_number_t_t_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_t_t() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let generic_t = fixture.generic("T");

    let nothing_to_number_ty =
      fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![number_ty]);
    let generic_nothing_to_t_ty = fixture.generic_fn(vec![generic_t], vec![], vec![generic_t]);

    assert!(
      !fixture
        .is_subtype_type_id_type_id(nothing_to_number_ty, generic_nothing_to_t_ty)
        .is_subtype()
    );
  }
}

mod subtyping_number_unknown_subtyping_test {
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_number_unknown() {
    let mut fixture = SubtypeFixture::default();
    let optional_number_ty = fixture.builtin_types.optional_number_type;
    let unknown_ty = fixture.builtin_types.unknown_type;

    assert!(
      fixture
        .is_subtype_type_id_type_id(optional_number_ty, unknown_ty)
        .is_subtype()
    );
  }
}

mod subtyping_string_number_a_string_string_number_a_string {

  use alloc::vec;

  use ulua_analysis::{
    records::{function_type::FunctionType, property_type::Property},
    type_aliases::type_id::TypeId,
  };
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  fn make_the_type(fixture: &mut SubtypeFixture) -> TypeId {
    let string_ty = fixture.builtin_types.string_type;
    let number_ty = fixture.builtin_types.number_type;
    let arg_type = fixture.tbl_with_indexer(
      SubtypeFixture::props(vec![("a", Property::rw_type_id(string_ty))]),
      string_ty,
      number_ty,
    );
    let arg_pack = fixture.pack_initializer_list_type_id(vec![arg_type]);
    let empty_type_pack = fixture.builtin_types.empty_type_pack;

    fixture.arena.add_type(FunctionType::function_type_new(
      arg_pack,
      empty_type_pack,
      None,
      false,
    ))
  }

  #[cfg(test)]
  #[test]
  fn subtyping_string_number_a_string_string_number_a_string() {
    let mut fixture = SubtypeFixture::default();

    let a = make_the_type(&mut fixture);
    let b = make_the_type(&mut fixture);

    assert!(fixture.is_subtype_type_id_type_id(a, b).is_subtype());
  }
}

mod subtyping_t_t_number_subtyping_test {
  use alloc::vec;

  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_t_t_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let generic_t = fixture.generic("T");

    let generic_nothing_to_t_ty = fixture.generic_fn(vec![generic_t], vec![], vec![generic_t]);
    let nothing_to_number_ty =
      fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![number_ty]);

    assert!(
      fixture
        .is_subtype_type_id_type_id(generic_nothing_to_t_ty, nothing_to_number_ty)
        .is_subtype()
    );
  }
}

mod subtyping_table_test_is_non_suppressing_if_any_mismatches_are_non_suppressing {
  //! Ported from upstream Luau doctest.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1898:subtyping_table_test_is_non_suppressing_if_any_mismatches_are_non_suppressing`
  //! Source: `tests/Subtyping.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Subtyping.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
  //!   - includes -> source_file Analysis/include/Luau/TypePath.h
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Subtyping.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/RegisterCallbacks.h
  //! - incoming:
  //!   - declares <- source_file tests/Subtyping.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record SubtypingResult (Analysis/include/Luau/Subtyping.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item subtyping_table_test_is_non_suppressing_if_any_mismatches_are_non_suppressing

  #[cfg(test)]
  #[test]
  fn subtyping_table_test_is_non_suppressing_if_any_mismatches_are_non_suppressing() {
    use alloc::sync::Arc;

    use ulua_analysis::records::scope::Scope;
    use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

    let mut fixture = SubtypeFixture::default();
    let table_one = fixture.base.parse_type("{foo: any, bar: string, baz: any}");
    let table_two = fixture
      .base
      .parse_type("{foo: number, bar: number, baz: boolaen}");
    let root_scope = Arc::as_ptr(&fixture.root_scope) as *mut Scope;

    let sr = fixture
      .subtyping
      .is_subtype_type_id_type_id_not_null_scope(table_one, table_two, root_scope);

    assert!(!sr.is_subtype());
    assert!(!sr.is_error_suppressing());
  }
}

mod subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing {
  //! Ported from upstream Luau doctest.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1887:subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing`
  //! Source: `tests/Subtyping.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Subtyping.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
  //!   - includes -> source_file Analysis/include/Luau/TypePath.h
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Subtyping.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/RegisterCallbacks.h
  //! - incoming:
  //!   - declares <- source_file tests/Subtyping.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record SubtypingResult (Analysis/include/Luau/Subtyping.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing

  #[cfg(test)]
  #[test]
  fn subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing() {
    use alloc::sync::Arc;

    use ulua_analysis::records::scope::Scope;
    use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

    let mut fixture = SubtypeFixture::default();
    let table_one = fixture.base.parse_type("{foo: any, bar: any}");
    let table_two = fixture.base.parse_type("{foo: number, bar: string}");
    let root_scope = Arc::as_ptr(&fixture.root_scope) as *mut Scope;

    let sr = fixture
      .subtyping
      .is_subtype_type_id_type_id_not_null_scope(table_one, table_two, root_scope);

    assert!(!sr.is_subtype());
    assert!(sr.is_error_suppressing());
  }
}

mod subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution {
  //! Ported from upstream Luau doctest.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1873:subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution`
  //! Source: `tests/Subtyping.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Subtyping.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
  //!   - includes -> source_file Analysis/include/Luau/TypePath.h
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Subtyping.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/RegisterCallbacks.h
  //! - incoming:
  //!   - declares <- source_file tests/Subtyping.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution

  #[cfg(test)]
  #[test]
  fn subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let res = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type ActionCallback = (string) -> ...any

function bindAction(callback: ActionCallback)
  local _ = function(...)
    callback(...)
  end
end
"#,
      ),
      None,
    );

    fixture.validate_errors(&res.errors);
    assert!(res.errors.is_empty(), "{}", fixture.get_errors(&res));
  }
}

mod subtyping_weird_cyclic_instantiation {
  //! Ported from upstream Luau doctest.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1909:subtyping_weird_cyclic_instantiation`
  //! Source: `tests/Subtyping.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Subtyping.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
  //!   - includes -> source_file Analysis/include/Luau/TypePath.h
  //!   - includes -> source_file Analysis/include/Luau/Normalize.h
  //!   - includes -> source_file Analysis/include/Luau/Subtyping.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/RegisterCallbacks.h
  //! - incoming:
  //!   - declares <- source_file tests/Subtyping.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
  //!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item subtyping_weird_cyclic_instantiation

  #[cfg(test)]
  #[test]
  fn subtyping_weird_cyclic_instantiation() {
    use alloc::{string::String, vec};
    use core::ptr::null;

    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::{
        get_mutable_type::get_mutable_type_id, instantiate_2_instantiation_2::instantiate_2,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{
        free_type::FreeType, function_type::FunctionType, generic_type::GenericType, scope::Scope,
        type_arena::TypeArena,
      },
      type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
    };
    use ulua_common::records::dense_hash_map::DenseHashMap;
    use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

    let mut fixture = SubtypeFixture::default();
    let mut arena = TypeArena::default();
    let mut scope = Scope::scope_type_pack_id(fixture.builtin_types.any_type_pack);

    let generic_t = arena.add_type(GenericType::generic_type_name_polarity(
      &String::from("T"),
      Polarity::Mixed,
    ));
    let id_arg_types = arena.add_type_pack_initializer_list_type_id(&[generic_t]);
    let id_ret_types = arena.add_type_pack_initializer_list_type_id(&[generic_t]);
    let id_ty = arena.add_type(FunctionType::new_with_generics(
      vec![generic_t],
      vec![],
      id_arg_types,
      id_ret_types,
      None,
      false,
    ));

    let mut generic_substitutions: DenseHashMap<TypeId, TypeId> = DenseHashMap::new(null());
    let generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId> =
      DenseHashMap::new(null());

    let free_ty = arena.fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, &mut scope);
    let ft = get_mutable_type_id::<FreeType>(free_ty);
    let ft = ft.expect("fresh type should be a FreeType");
    ft.lower_bound = id_ty;
    ft.upper_bound = fixture.builtin_types.unknown_type;

    *generic_substitutions.get_or_insert(generic_t) = free_ty;

    assert_eq!("<T>(T) -> T", to_string_type_id(id_ty));

    let res = instantiate_2(
      &mut arena,
      generic_substitutions,
      generic_pack_substitutions,
      &mut *fixture.subtyping,
      &mut scope,
      id_ty,
    );

    assert_eq!("<T>(T) -> T", to_string_type_id(id_ty));

    let res = res.expect("instantiate2 should return a type");
    assert_eq!("<T>(T) -> T", to_string_type_id(res));
  }
}

mod subtyping_x_number_subtyping_test {
  use alloc::vec;

  use ulua_analysis::records::property_type::Property;
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_x_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let left = fixture.tbl(SubtypeFixture::props(vec![(
      "x",
      Property::rw_type_id(number_ty),
    )]));
    let right = fixture.tbl(SubtypeFixture::props(vec![]));

    assert!(fixture.is_subtype_type_id_type_id(left, right).is_subtype());
  }
}

mod subtyping_x_number_x_number_subtyping_test {
  use alloc::vec;

  use ulua_analysis::records::property_type::Property;
  use ulua_unit_test::records::subtype_fixture::SubtypeFixture;

  #[cfg(test)]
  #[test]
  fn subtyping_x_number_x_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.number_type;
    let optional_number_ty = fixture.builtin_types.optional_number_type;
    let left = fixture.tbl(SubtypeFixture::props(vec![(
      "x",
      Property::rw_type_id(number_ty),
    )]));
    let right = fixture.tbl(SubtypeFixture::props(vec![(
      "x",
      Property::rw_type_id(optional_number_ty),
    )]));

    assert!(!fixture.is_subtype_type_id_type_id(left, right).is_subtype());
  }
}
