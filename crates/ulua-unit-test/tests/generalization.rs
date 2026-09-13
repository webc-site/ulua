extern crate alloc;

mod generalization_a_a {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:218:generalization_a_a`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_a_a

  #[cfg(test)]
  #[test]
  fn generalization_a_a() {
    use ulua_analysis::records::function_type::FunctionType;
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let free_ty = fixture.fresh_type().0;
    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[free_ty]);
    let rets = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[free_ty]);
    let fn_ty = fixture
      .arena
      .add_type(FunctionType::function_type_new(args, rets, None, false));

    fixture.generalize(fn_ty);

    assert_eq!("<a>(a) -> a", fixture.to_string_type_id(fn_ty));
  }
}

mod generalization_a_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:260:generalization_a_b`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item generalization_a_b

  #[cfg(test)]
  #[test]
  fn generalization_a_b() {
    use ulua_analysis::records::{
      function_type::FunctionType, table_indexer::TableIndexer, table_type::TableType,
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let (a_ty, a_free) = fixture.fresh_type();
    let (b_ty, _) = fixture.fresh_type();

    let mut tt = TableType::new();
    tt.indexer = Some(TableIndexer {
      index_type: fixture.builtin_types.number_type,
      index_result_type: b_ty,
      is_read_only: false,
    });

    let table_ty = fixture.arena.add_type(tt);
    unsafe {
      (*a_free).upper_bound = table_ty;
    }

    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[a_ty]);
    let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
      args,
      fixture.builtin_types.empty_type_pack,
      None,
      false,
    ));

    fixture.generalize(function_ty);

    assert_eq!("<a>({a}) -> ()", fixture.to_string_type_id(function_ty));
  }
}

mod generalization_a_number_string_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:247:generalization_a_number_string_string`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item generalization_a_number_string_string

  #[cfg(test)]
  #[test]
  fn generalization_a_number_string_string() {
    use ulua_analysis::records::{function_type::FunctionType, union_type::UnionType};
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let (a_ty, a_free) = fixture.fresh_type();

    let upper_bound = fixture.arena.add_type(UnionType {
      options: vec![
        fixture.builtin_types.number_type,
        fixture.builtin_types.string_type,
      ],
    });
    unsafe {
      (*a_free).upper_bound = upper_bound;
    }

    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[a_ty]);
    let rets = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.optional_string_type]);
    let fn_type = fixture
      .arena
      .add_type(FunctionType::function_type_new(args, rets, None, false));

    fixture.generalize(fn_type);

    assert_eq!(
      "(number | string) -> string?",
      fixture.to_string_type_id(fn_type)
    );
  }
}

mod generalization_avoid_cross_module_mutation_in_bidirectional_inference {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:423:generalization_avoid_cross_module_mutation_in_bidirectional_inference`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item generalization_avoid_cross_module_mutation_in_bidirectional_inference

  #[cfg(test)]
  #[test]
  fn generalization_avoid_cross_module_mutation_in_bidirectional_inference() {
    use std::sync::Arc;

    use ulua_analysis::{functions::freeze::freeze, records::module::Module};
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("Module/ListFns"),
      String::from(
        r#"
        local mod = {}
        function mod.findWhere(list, predicate): number?
            for i = 1, #list do
                if predicate(list[i], i) then
                    return i
                end
            end
            return nil
        end
        return mod
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        local funs = require(script.Parent.ListFns)
        local accessories = funs.findWhere(getList(), function(accessory)
            return accessory.AccessoryType ~= accessoryTypeEnum
        end)
        return {}
    "#,
      ),
    );

    let module_list_fns = String::from("Module/ListFns");
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_list_fns, None);
    let mod_list_fns = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_list_fns);

    unsafe {
      let module = Arc::as_ptr(&mod_list_fns) as *mut Module;
      freeze(&mut (*module).interface_types);
      freeze(&mut (*module).internal_types);
    }

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let module_b = String::from("Module/B");
    let _result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod generalization_b_t1_a_t1_t1_where_t1_a_t1_c {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:279:generalization_b_t1_a_t1_t1_where_t1_a_t1_c`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_b_t1_a_t1_t1_where_t1_a_t1_c

  #[cfg(test)]
  #[test]
  fn generalization_b_t1_a_t1_t1_where_t1_a_t1_c() {
    use ulua_analysis::records::{
      function_type::FunctionType, table_indexer::TableIndexer, table_type::TableType,
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let (a_ty, a_free) = fixture.fresh_type();
    let (b_ty, b_free) = fixture.fresh_type();
    let (c_ty, c_free) = fixture.fresh_type();

    unsafe {
      (*a_free).upper_bound = c_ty;
      (*c_free).lower_bound = a_ty;
    }

    let mut tt = TableType::new();
    tt.indexer = Some(TableIndexer {
      index_type: fixture.builtin_types.number_type,
      index_result_type: c_ty,
      is_read_only: false,
    });

    let table_ty = fixture.arena.add_type(tt);
    unsafe {
      (*b_free).upper_bound = table_ty;
    }

    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[b_ty, a_ty]);
    let rets = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[c_ty]);
    let function_ty = fixture
      .arena
      .add_type(FunctionType::function_type_new(args, rets, None, false));

    fixture.generalize(function_ty);

    assert_eq!("<a>({a}, a) -> a", fixture.to_string_type_id(function_ty));
  }
}

mod generalization_cache_fully_generalized_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:126:generalization_cache_fully_generalized_types`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_cache_fully_generalized_types

  #[cfg(test)]
  #[test]
  fn generalization_cache_fully_generalized_types() {
    use alloc::collections::BTreeMap;

    use ulua_analysis::{
      enums::table_state::TableState,
      records::{property_type::Property, table_type::TableType, type_level::TypeLevel},
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    assert!(fixture.generalized_types.empty());

    let mut props = BTreeMap::new();
    props.insert(
      String::from("one"),
      Property::rw_type_id(fixture.builtin_types.number_type),
    );
    props.insert(
      String::from("two"),
      Property::rw_type_id(fixture.builtin_types.string_type),
    );
    let tiny_table = fixture.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    );

    fixture.generalize(tiny_table);

    assert!(fixture.generalized_types.contains(&tiny_table));
    assert!(
      fixture
        .generalized_types
        .contains(&fixture.builtin_types.number_type)
    );
    assert!(
      fixture
        .generalized_types
        .contains(&fixture.builtin_types.string_type)
    );
  }
}

mod generalization_dont_cache_types_that_arent_done_yet {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:141:generalization_dont_cache_types_that_arent_done_yet`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_dont_cache_types_that_arent_done_yet

  #[cfg(test)]
  #[test]
  fn generalization_dont_cache_types_that_arent_done_yet() {
    use alloc::collections::BTreeMap;
    use std::sync::Arc;

    use ulua_analysis::{
      enums::{polarity::Polarity, table_state::TableState},
      records::{
        free_type::FreeType, function_type::FunctionType, property_type::Property, scope::Scope,
        table_type::TableType, type_level::TypeLevel,
      },
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;
    let free_ty = fixture
      .arena
      .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
        global_scope,
        fixture.builtin_types.never_type,
        fixture.builtin_types.string_type,
        Polarity::Unknown,
      ));

    let fn_ret = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.number_type]);
    let fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
      fixture.builtin_types.empty_type_pack,
      fn_ret,
      None,
      false,
    ));

    let mut props = BTreeMap::new();
    props.insert(
      String::from("one"),
      Property::rw_type_id(fixture.builtin_types.number_type),
    );
    props.insert(String::from("two"), Property::rw_type_id(free_ty));
    props.insert(String::from("three"), Property::rw_type_id(fn_ty));
    let table_ty = fixture.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    );

    fixture.generalize(table_ty);

    assert!(fixture.generalized_types.contains(&fn_ty));
    assert!(
      fixture
        .generalized_types
        .contains(&fixture.builtin_types.number_type)
    );
    assert!(
      fixture
        .generalized_types
        .contains(&fixture.builtin_types.never_type)
    );
    assert!(
      fixture
        .generalized_types
        .contains(&fixture.builtin_types.string_type)
    );
    assert!(!fixture.generalized_types.contains(&free_ty));
    assert!(!fixture.generalized_types.contains(&table_ty));
  }
}

mod generalization_dont_traverse_into_class_types_when_generalizing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:112:generalization_dont_traverse_into_class_types_when_generalizing`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item generalization_dont_traverse_into_class_types_when_generalizing

  #[cfg(test)]
  #[test]
  fn generalization_dont_traverse_into_class_types_when_generalizing() {
    use alloc::collections::BTreeMap;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id,
      records::{extern_type::ExternType, free_type::FreeType, property_type::Property},
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let (prop_ty, _) = fixture.fresh_type();

    let mut props = BTreeMap::new();
    props.insert(String::from("oh_no"), Property::readonly(prop_ty));
    let cursed_extern_type = fixture.arena.add_type(ExternType {
      name: String::from("Cursed"),
      props,
      parent: None,
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: Default::default(),
      definition_location: None,
      indexer: None,
      relation: None,
    });

    let gen_extern_type = fixture.generalize(cursed_extern_type);
    assert!(gen_extern_type.is_some());

    let gen_extern_type = gen_extern_type.unwrap();
    let extern_type = get_type_id::<ExternType>(gen_extern_type).unwrap();
    let gen_prop_ty = extern_type.props.get("oh_no").unwrap().read_ty.unwrap();
    assert!(get_type_id::<FreeType>(gen_prop_ty).is_some());
  }
}

mod generalization_functions_containing_cyclic_tables_can_be_cached {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:161:generalization_functions_containing_cyclic_tables_can_be_cached`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
  //!   - calls -> method Variant::emplace (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_functions_containing_cyclic_tables_can_be_cached

  #[cfg(test)]
  #[test]
  fn generalization_functions_containing_cyclic_tables_can_be_cached() {
    use alloc::collections::BTreeMap;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::as_mutable_type::as_mutable_type_id,
      records::{
        blocked_type::BlockedType, function_type::FunctionType, property_type::Property,
        table_type::TableType, type_level::TypeLevel,
      },
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let self_ty = fixture.arena.add_type(BlockedType::default());

    let method_args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[self_ty]);
    let method_rets = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.number_type]);
    let method_ty = fixture.arena.add_type(FunctionType::function_type_new(
      method_args,
      method_rets,
      None,
      false,
    ));

    let mut props = BTreeMap::new();
    props.insert(
      String::from("count"),
      Property::rw_type_id(fixture.builtin_types.number_type),
    );
    props.insert(String::from("method"), Property::rw_type_id(method_ty));
    unsafe {
      (*as_mutable_type_id(self_ty)).ty = TypeVariant::Table(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
          &props,
          None,
          TypeLevel::default(),
          TableState::Sealed,
        ),
      );
    }

    fixture.generalize(method_ty);

    assert!(fixture.generalized_types.contains(&method_ty));
    assert!(fixture.generalized_types.contains(&self_ty));
    assert!(
      fixture
        .generalized_types
        .contains(&fixture.builtin_types.number_type)
    );
  }
}

mod generalization_generalization_fuzzer_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:455:generalization_generalization_fuzzer_crash`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item generalization_generalization_fuzzer_crash

  #[cfg(test)]
  #[test]
  fn generalization_generalization_fuzzer_crash() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function t0<A>(l0,...):""
        type t0 = any
        do
        _()
        _ = {_=...,}
        _ = {_=rawget({_=_,l0,},_,- _),}
        end
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod generalization_generalization_should_not_leak_free_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:339:generalization_generalization_should_not_leak_free_type`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item generalization_generalization_should_not_leak_free_type

  #[cfg(test)]
  #[test]
  fn generalization_generalization_should_not_leak_free_type() {
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&FFlag::DebugLuauForbidInternalTypes, true);
    let mut fixture = BuiltinsFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo()

            local productButtonPairs = {}
            local func
            local dir = -1

            local function updateSearch()
                for product, button in pairs(productButtonPairs) do
                    -- This line may have a floating free type pack.
                    button.LayoutOrder = func(product) * dir
                end
            end

            function(mode)
                if mode == 'New'then
                    func = function(p)
                        return p.id
                    end
                elseif mode == 'Price'then
                    func = function(p)
                        return p.price
                    end
                end
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod generalization_generalization_traversal_should_re_traverse_unions_if_they_change_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:300:generalization_generalization_traversal_should_re_traverse_unions_if_they_change_type`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item generalization_generalization_traversal_should_re_traverse_unions_if_they_change_type

  #[cfg(test)]
  #[test]
  fn generalization_generalization_traversal_should_re_traverse_unions_if_they_change_type() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function byId(p)
 return p.id
end

function foo()

 local productButtonPairs = {}
 local func = byId
 local dir = -1

 local function updateSearch()
  for product, button in pairs(productButtonPairs) do
   button.LayoutOrder = func(product) * dir
  end
 end

  function(mode)
   if mode == 'Name'then
   else
    if mode == 'New'then
     func = function(p)
      return p.id
     end
    elseif mode == 'Price'then
     func = function(p)
      return p.price
     end
    end

   end
  end
end
"#,
      ),
      None,
    );
  }
}

mod generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:62:generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type

  #[cfg(test)]
  #[test]
  fn generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type() {
    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let (t1, ft1) = fixture.fresh_type();
    let (t2, ft2) = fixture.fresh_type();

    unsafe {
      (*ft1).lower_bound = t2;
      (*ft2).upper_bound = t1;
      (*ft2).lower_bound = fixture.builtin_types.unknown_type;
    }

    let t2_generalized = fixture.generalize(t2);
    assert!(t2_generalized.is_some());

    assert_eq!(follow_type_id(t1), follow_type_id(t2));

    let t1_generalized = fixture.generalize(t1);
    assert!(t1_generalized.is_some());

    assert_eq!(fixture.builtin_types.unknown_type, follow_type_id(t1));
    assert_eq!(fixture.builtin_types.unknown_type, follow_type_id(t2));
  }
}

mod generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:88:generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order

  #[cfg(test)]
  #[test]
  fn generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order()
   {
    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let (t1, ft1) = fixture.fresh_type();
    let (t2, ft2) = fixture.fresh_type();

    unsafe {
      (*ft1).lower_bound = t2;
      (*ft2).upper_bound = t1;
      (*ft2).lower_bound = fixture.builtin_types.unknown_type;
    }

    let t1_generalized = fixture.generalize(t1);
    assert!(t1_generalized.is_some());

    assert_eq!(follow_type_id(t1), follow_type_id(t2));

    let t2_generalized = fixture.generalize(t2);
    assert!(t2_generalized.is_some());

    assert_eq!(fixture.builtin_types.unknown_type, follow_type_id(t1));
    assert_eq!(fixture.builtin_types.unknown_type, follow_type_id(t2));
  }
}

mod generalization_generic_argument_with_singleton_oss_1808 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:411:generalization_generic_argument_with_singleton_oss_1808`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item generalization_generic_argument_with_singleton_oss_1808

  #[cfg(test)]
  #[test]
  fn generalization_generic_argument_with_singleton_oss_1808() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function test<T>(value: false | (T) -> T)
            return value
        end
        test(false)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod generalization_generics_dont_leak_into_callback {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:373:generalization_generics_dont_leak_into_callback`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item generalization_generics_dont_leak_into_callback

  #[cfg(test)]
  #[test]
  fn generalization_generics_dont_leak_into_callback() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local func: <T>(T, (T) -> ()) -> () = nil :: any
        func({}, function(obj)
            local _ = obj
        end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.require_type_at_position_position(Position {
        line: 3,
        column: 23,
      }))
    );
  }
}

mod generalization_generics_dont_leak_into_callback_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:392:generalization_generics_dont_leak_into_callback_2`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item generalization_generics_dont_leak_into_callback_2

  #[cfg(test)]
  #[test]
  fn generalization_generics_dont_leak_into_callback_2() {
    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local func: <T>(T, (T) -> ()) -> () = nil :: any
local foobar: (number) -> () = nil :: any
func({}, function(obj)
    foobar(obj)
end)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(err.wanted_type));
    assert_eq!("{  }", to_string_type_id(err.given_type));
  }
}

mod generalization_intersection_type_traversal_doesnt_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:200:generalization_intersection_type_traversal_doesnt_crash`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method Normalizer::intersectionType (Analysis/src/Normalize.cpp)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_intersection_type_traversal_doesnt_crash

  #[cfg(test)]
  #[test]
  fn generalization_intersection_type_traversal_doesnt_crash() {
    use std::sync::Arc;

    use ulua_analysis::{
      functions::get_mutable_type::get_mutable_type_id,
      records::{free_type::FreeType, intersection_type::IntersectionType, scope::Scope},
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;

    let i = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let h = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let j = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let intersection_type = fixture
      .arena
      .add_type(IntersectionType { parts: vec![h, j] });

    get_mutable_type_id::<FreeType>(h).unwrap().upper_bound = i;
    get_mutable_type_id::<FreeType>(h).unwrap().lower_bound = fixture.builtin_types.never_type;
    get_mutable_type_id::<FreeType>(i).unwrap().upper_bound = fixture.builtin_types.unknown_type;
    get_mutable_type_id::<FreeType>(i).unwrap().lower_bound = intersection_type;
    get_mutable_type_id::<FreeType>(j).unwrap().upper_bound = i;
    get_mutable_type_id::<FreeType>(j).unwrap().lower_bound = fixture.builtin_types.never_type;

    fixture.generalize(intersection_type);
  }
}

mod generalization_t1_t1_b_where_t1_a_t1_b_number_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:228:generalization_t1_t1_b_where_t1_a_t1_b_number_number`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_t1_t1_b_where_t1_a_t1_b_number_number

  #[cfg(test)]
  #[test]
  fn generalization_t1_t1_b_where_t1_a_t1_b_number_number() {
    use ulua_analysis::records::{
      function_type::FunctionType, intersection_type::IntersectionType,
      table_indexer::TableIndexer, table_type::TableType,
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let mut tt = TableType::new();
    tt.indexer = Some(TableIndexer {
      index_type: fixture.builtin_types.number_type,
      index_result_type: fixture.builtin_types.number_type,
      is_read_only: false,
    });
    let number_array = fixture.arena.add_type(tt);

    let (a_ty, a_free) = fixture.fresh_type();
    let (b_ty, b_free) = fixture.fresh_type();

    let upper_bound = fixture.arena.add_type(IntersectionType {
      parts: vec![b_ty, number_array, number_array],
    });
    unsafe {
      (*a_free).upper_bound = upper_bound;
      (*b_free).lower_bound = a_ty;
    }

    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[a_ty, b_ty]);
    let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
      args,
      fixture.builtin_types.empty_type_pack,
      None,
      false,
    ));

    fixture.generalize(function_ty);

    assert_eq!(
      "(unknown & {number}, unknown) -> ()",
      fixture.to_string_type_id(function_ty)
    );
  }
}

mod generalization_union_type_traversal_doesnt_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:183:generalization_union_type_traversal_doesnt_crash`
  //! Source: `tests/Generalization.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Generalization.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Generalization.h
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Generalization.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method Normalizer::unionType (Analysis/src/Normalize.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
  //!   - translates_to -> rust_item generalization_union_type_traversal_doesnt_crash

  #[cfg(test)]
  #[test]
  fn generalization_union_type_traversal_doesnt_crash() {
    use std::sync::Arc;

    use ulua_analysis::{
      functions::get_mutable_type::get_mutable_type_id,
      records::{free_type::FreeType, scope::Scope, union_type::UnionType},
    };
    use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

    let mut fixture = GeneralizationFixture::new();
    let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;

    let i = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let h = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let j = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let union_type = fixture.arena.add_type(UnionType {
      options: vec![h, j],
    });

    get_mutable_type_id::<FreeType>(h).unwrap().upper_bound = i;
    get_mutable_type_id::<FreeType>(h).unwrap().lower_bound = fixture.builtin_types.never_type;
    get_mutable_type_id::<FreeType>(i).unwrap().upper_bound = fixture.builtin_types.unknown_type;
    get_mutable_type_id::<FreeType>(i).unwrap().lower_bound = union_type;
    get_mutable_type_id::<FreeType>(j).unwrap().upper_bound = i;
    get_mutable_type_id::<FreeType>(j).unwrap().lower_bound = fixture.builtin_types.never_type;

    fixture.generalize(union_type);
  }
}
