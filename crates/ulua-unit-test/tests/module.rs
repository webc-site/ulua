extern crate alloc;

mod module_any_persistance_does_not_leak {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:393:module_any_persistance_does_not_leak`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item module_any_persistance_does_not_leak

  #[cfg(test)]
  #[test]
  fn module_any_persistance_does_not_leak() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::frontend_options::FrontendOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let module_name = String::from("Module/A");

    fixture.file_resolver.source.insert(
      module_name.clone(),
      String::from(
        r#"
export type A = B
type B = A
    "#,
      ),
    );

    let opts = FrontendOptions {
      retain_full_type_graphs: false,
      ..Default::default()
    };
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_name, Some(opts));
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_name);
    let binding = module
      .exported_type_bindings
      .get("A")
      .expect("expected exported type binding A");

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("any", to_string_type_id(binding.r#type()));
    } else {
      assert_eq!("*error-type*", to_string_type_id(binding.r#type()));
    }
  }
}

mod module_builtin_types_point_into_global_types_arena {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:179:module_builtin_types_point_into_global_types_arena`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> function isInArena (tests/Fixture.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item module_builtin_types_point_into_global_types_arena

  #[cfg(test)]
  #[test]
  fn module_builtin_types_point_into_global_types_arena() {
    use ulua_analysis::{
      functions::{first::first, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::{
      functions::is_in_arena::is_in_arena, records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    let module_name = String::from("MainModule");

    fixture.base.file_resolver.source.insert(
      module_name.clone(),
      String::from(
        r#"
        return {sign=math.sign}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_name);
    let exports = first(module.return_type, true).expect("expected module return type");

    assert!(is_in_arena(exports, &module.interface_types));

    let exports_table = get_type_id::<TableType>(exports).expect("expected return table");

    let sign_type = exports_table
      .props
      .get("sign")
      .and_then(|prop| prop.read_ty)
      .expect("expected sign property read type");

    assert!(!is_in_arena(sign_type, &module.interface_types));
    assert!(is_in_arena(
      sign_type,
      fixture.get_frontend().globals.global_types_mut()
    ));
  }
}

mod module_clone_a_bound_type_to_a_persistent_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:509:module_clone_a_bound_type_to_a_persistent_type`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - translates_to -> rust_item module_clone_a_bound_type_to_a_persistent_type

  #[cfg(test)]
  #[test]
  fn module_clone_a_bound_type_to_a_persistent_type() {
    use ulua_analysis::{
      functions::{clone_clone_alt_b::clone as clone_type, follow_type::follow},
      records::{clone_state::CloneState, type_arena::TypeArena},
      type_aliases::bound_type::BoundType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let mut arena = TypeArena::default();
    let bound_to = arena.add_type(BoundType {
      bound_to: fixture.base.get_builtins().number_type,
    });

    assert!(unsafe { (*fixture.base.get_builtins().number_type).persistent });

    let mut dest = TypeArena::default();
    let mut state = CloneState::new(fixture.base.get_builtins());
    let res = unsafe { clone_type(bound_to, &mut dest, &mut state) };

    assert_eq!(res, follow(bound_to));
  }
}

mod module_clone_a_bound_typepack_to_a_persistent_typepack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:523:module_clone_a_bound_typepack_to_a_persistent_typepack`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> type_alias BoundTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - translates_to -> rust_item module_clone_a_bound_typepack_to_a_persistent_typepack

  #[cfg(test)]
  #[test]
  fn module_clone_a_bound_typepack_to_a_persistent_typepack() {
    use ulua_analysis::{
      functions::{clone_clone::clone as clone_pack, follow_type_pack::follow},
      records::{clone_state::CloneState, type_arena::TypeArena},
      type_aliases::bound_type_pack::BoundTypePack,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let mut arena = TypeArena::default();
    let bound_to = arena.add_type_pack_t(BoundTypePack {
      bound_to: fixture.base.get_builtins().never_type_pack,
    });

    assert!(unsafe { (*fixture.base.get_builtins().never_type_pack).is_persistent() });

    let mut dest = TypeArena::default();
    let mut state = CloneState::new(fixture.base.get_builtins());
    let res = unsafe { clone_pack(bound_to, &mut dest, &mut state) };

    assert_eq!(res, unsafe { follow(bound_to) });
  }
}

mod module_clone_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:233:module_clone_class`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item module_clone_class

  #[cfg(test)]
  #[test]
  fn module_clone_class() {
    use ulua_analysis::{
      functions::{clone_clone_alt_b::clone as clone_type, get_type_alt_j::get_type_id},
      records::{
        clone_state::CloneState, extern_type::ExternType, property_type::Property, r#type::Type,
        type_arena::TypeArena,
      },
      type_aliases::props_type::Props,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().any_type;
    let number_type = fixture.get_builtins().number_type;
    let string_type = fixture.get_builtins().string_type;

    let mut metaclass_props = Props::default();
    metaclass_props.insert("__add".into(), Property::readonly(any_type));
    let example_meta_class = Type::from(ExternType {
      name: "ExampleClassMeta".into(),
      props: metaclass_props,
      parent: None,
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: "Test".into(),
      definition_location: None,
      indexer: None,
      relation: None,
    });
    let example_meta_class_id = &example_meta_class as *const Type;

    let mut class_props = Props::default();
    class_props.insert("PropOne".into(), Property::readonly(number_type));
    class_props.insert("PropTwo".into(), Property::readonly(string_type));
    let example_class = Type::from(ExternType {
      name: "ExampleClass".into(),
      props: class_props,
      parent: None,
      metatable: Some(example_meta_class_id),
      tags: Default::default(),
      user_data: None,
      definition_module_name: "Test".into(),
      definition_location: None,
      indexer: None,
      relation: None,
    });
    let example_class_id = &example_class as *const Type;

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned = unsafe { clone_type(example_class_id, &mut dest, &mut clone_state) };
    let etv = get_type_id::<ExternType>(cloned).expect("expected extern type");

    let metatable_ty = etv.metatable.expect("expected cloned metatable");
    let metatable = get_type_id::<ExternType>(metatable_ty).expect("expected extern metatable");

    assert_eq!("ExampleClass", etv.name);
    assert_eq!("ExampleClassMeta", metatable.name);
  }
}

mod module_clone_cyclic_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:368:module_clone_cyclic_union`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item module_clone_cyclic_union

  #[cfg(test)]
  #[test]
  fn module_clone_cyclic_union() {
    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, get_mutable_type::get_mutable_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{clone_state::CloneState, type_arena::TypeArena, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type();
    let string_type = fixture.get_builtins().string_type();

    let mut src = TypeArena::default();
    let u = src.add_type(UnionType {
      options: alloc::vec![number_type, string_type],
    });
    let uu = get_mutable_type_id::<UnionType>(u).expect("expected source union");

    uu.options.push(u);

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned = unsafe { clone_type(u, &mut dest, &mut clone_state) };
    assert!(!cloned.is_null());

    let cloned_union = get_type_id::<UnionType>(cloned).expect("expected cloned union");
    assert_eq!(3, cloned_union.options.len());

    assert_eq!(number_type, cloned_union.options[0]);
    assert_eq!(string_type, cloned_union.options[1]);
    assert_eq!(cloned, cloned_union.options[2]);
  }
}

mod module_clone_free_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:293:module_clone_free_tables`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item module_clone_free_tables

  #[cfg(test)]
  #[test]
  fn module_clone_free_tables() {
    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        clone_clone_alt_b::clone as clone_type, get_mutable_type::get_mutable_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{
        clone_state::CloneState, table_type::TableType, r#type::Type, type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let mut table_ty = Type::from(TableType::new());
    let table_ty_id = &mut table_ty as *mut Type as *const Type;
    let ttv = get_mutable_type_id::<TableType>(table_ty_id).expect("expected source table type");
    ttv.state = TableState::Free;

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned = unsafe { clone_type(table_ty_id, &mut dest, &mut clone_state) };
    let cloned_ttv = get_type_id::<TableType>(cloned).expect("expected cloned table type");

    assert_eq!(cloned_ttv.state, TableState::Free);
  }
}

mod module_clone_free_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:276:module_clone_free_types`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item module_clone_free_types

  #[cfg(test)]
  #[test]
  fn module_clone_free_types() {
    use core::ptr::null_mut;

    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::{
        clone_clone::clone as clone_pack, clone_clone_alt_b::clone as clone_type,
        fresh_type::fresh_type, get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
      },
      records::{
        clone_state::CloneState, free_type::FreeType, free_type_pack::FreeTypePack,
        type_arena::TypeArena, type_level::TypeLevel, type_pack_var::TypePackVar,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut arena = TypeArena::default();
    let free_ty = fresh_type(
      &mut arena,
      fixture.get_builtins(),
      null_mut(),
      Polarity::Unknown,
    );
    let free_tp = TypePackVar::from(FreeTypePack::new(TypeLevel::default()));
    let free_tp_id = &free_tp as *const TypePackVar;

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned_ty = unsafe { clone_type(free_ty, &mut dest, &mut clone_state) };
    assert!(get_type_id::<FreeType>(cloned_ty).is_some());

    clone_state = CloneState::new(fixture.get_builtins());
    let cloned_tp = unsafe { clone_pack(free_tp_id, &mut dest, &mut clone_state) };
    assert!(get_type_pack_id::<FreeTypePack>(cloned_tp).is_some());
  }
}

mod module_clone_iteration_limit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:337:module_clone_iteration_limit`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias ErrorType (Analysis/include/Luau/Type.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item module_clone_iteration_limit

  #[cfg(test)]
  #[test]
  fn module_clone_iteration_limit() {
    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, get_mutable_type::get_mutable_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{
        clone_state::CloneState, property_type::Property, table_type::TableType,
        type_arena::TypeArena,
      },
      type_aliases::error_type::ErrorType,
    };
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _sfi = ScopedFastInt::new(&FInt::LuauTypeCloneIterationLimit, 2000);
    let mut fixture = Fixture::fixture_bool(false);

    let mut src = TypeArena::default();

    let table = src.add_type(TableType::new());
    let mut nested = table;

    let nesting = 2500;
    for _ in 0..nesting {
      let child = src.add_type(TableType::new());
      let ttv = get_mutable_type_id::<TableType>(nested).expect("expected nested table type");
      ttv.props.insert("a".into(), Property::rw_type_id(child));
      nested = child;
    }

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let ty = unsafe { clone_type(table, &mut dest, &mut clone_state) };
    assert!(get_type_id::<ErrorType>(ty).is_some());

    // Cloning it again is an important test.
    let ty2 = unsafe { clone_type(table, &mut dest, &mut clone_state) };
    assert!(get_type_id::<ErrorType>(ty2).is_some());
  }
}

mod module_clone_self_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:307:module_clone_self_property`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item module_clone_self_property

  #[cfg(test)]
  #[test]
  fn module_clone_self_property() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    // CLI-117082 ModuleTests.clone_self_property we don't infer self correctly,
    // instead replacing it with unknown.
    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        --!nonstrict
        local a = {}
        function a:foo(x: number)
            return -x;
        end
        return a;
    "#,
      ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_a, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        --!nonstrict
        local a = require(script.Parent.A)
        return a.foo(5)
    "#,
      ),
    );

    let module_b = String::from("Module/B");
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "This function must be called with self. Did you mean to use a colon instead of a dot?",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod module_clone_table_bound_to_table_bound_to_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:481:module_clone_table_bound_to_table_bound_to_table`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item module_clone_table_bound_to_table_bound_to_table

  #[cfg(test)]
  #[test]
  fn module_clone_table_bound_to_table_bound_to_table() {
    use core::ptr::null_mut;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        clone_clone_alt_b::clone as clone_type, get_mutable_type::get_mutable_type_id,
        get_type_alt_j::get_type_id,
      },
      records::{
        clone_state::CloneState, table_type::TableType, type_arena::TypeArena,
        type_level::TypeLevel,
      },
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let mut arena = TypeArena::default();

    let a = arena.add_type(TableType::table_type_table_state_type_level_scope(
      TableState::Free,
      TypeLevel::default(),
      null_mut(),
    ));
    get_mutable_type_id::<TableType>(a)
      .expect("expected table a")
      .name = Some("a".into());

    let b = arena.add_type(TableType::table_type_table_state_type_level_scope(
      TableState::Free,
      TypeLevel::default(),
      null_mut(),
    ));
    get_mutable_type_id::<TableType>(b)
      .expect("expected table b")
      .name = Some("b".into());

    let c = arena.add_type(TableType::table_type_table_state_type_level_scope(
      TableState::Free,
      TypeLevel::default(),
      null_mut(),
    ));
    get_mutable_type_id::<TableType>(c)
      .expect("expected table c")
      .name = Some("c".into());

    get_mutable_type_id::<TableType>(a)
      .expect("expected table a")
      .bound_to = Some(b);
    get_mutable_type_id::<TableType>(b)
      .expect("expected table b")
      .bound_to = Some(c);

    let mut dest = TypeArena::default();
    let mut state = CloneState::new(fixture.base.get_builtins());
    let res = unsafe { clone_type(a, &mut dest, &mut state) };

    assert_eq!(1, dest.types.size());

    let table_a = get_type_id::<TableType>(res).expect("expected cloned table");
    assert_eq!(Some("c"), table_a.name.as_deref());
    assert!(table_a.bound_to.is_none());
  }
}

mod module_deep_clone_cyclic_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:107:module_deep_clone_cyclic_table`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item module_deep_clone_cyclic_table

  #[cfg(test)]
  #[test]
  fn module_deep_clone_cyclic_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, first::first,
        get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{
        clone_state::CloneState, function_type::FunctionType, table_type::TableType,
        type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Cyclic = {}
        function Cyclic.get()
            return Cyclic
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("Cyclic"));

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());
    let clone_ty = unsafe { clone_type(ty, &mut dest, &mut clone_state) };

    let ttv = get_mutable_type_id::<TableType>(clone_ty).expect("expected cloned table type");

    assert_eq!(Some(String::from("Cyclic")), ttv.synthetic_name.clone());

    let method_type = ttv
      .props
      .get("get")
      .and_then(|prop| prop.read_ty)
      .expect("expected get method type");

    let ftv = get_type_id::<FunctionType>(method_type).expect("expected get method function type");

    let method_return_type = first(ftv.ret_types(), true).expect("expected method return type");

    assert!(
      method_return_type == clone_ty,
      "{} should be pointer identical to {}",
      to_string_type_id(method_type),
      to_string_type_id(clone_ty)
    );
    assert_eq!(2, dest.type_packs.size());
    assert_eq!(2, dest.types.size());
  }
}

mod module_deep_clone_cyclic_table_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:148:module_deep_clone_cyclic_table_2`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method Property::setType (Analysis/src/Type.cpp)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item module_deep_clone_cyclic_table_2

  #[cfg(test)]
  #[test]
  fn module_deep_clone_cyclic_table_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, first::first,
        get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
      },
      records::{
        clone_state::CloneState, function_type::FunctionType, table_type::TableType,
        type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut src = TypeArena::default();

    let table_ty = src.add_type(TableType::new());
    let arg_pack = src.add_type_pack_initializer_list_type_id(&[]);
    let ret_pack = src.add_type_pack_initializer_list_type_id(&[table_ty]);
    let method_ty = src.add_type(FunctionType::function_type_new(
      arg_pack, ret_pack, None, false,
    ));

    let tt = get_mutable_type_id::<TableType>(table_ty).expect("expected source table type");
    tt.props
      .entry(String::from("get"))
      .or_default()
      .set_type(method_ty);

    let mut dest = TypeArena::default();

    let mut clone_state = CloneState::new(fixture.get_builtins());
    let clone_ty = unsafe { clone_type(table_ty, &mut dest, &mut clone_state) };
    let ctt = get_mutable_type_id::<TableType>(clone_ty).expect("expected cloned table type");

    let cloned_method_type = ctt
      .props
      .get("get")
      .and_then(|prop| prop.read_ty)
      .expect("expected cloned get method type");

    let cmf = get_type_id::<FunctionType>(cloned_method_type)
      .expect("expected cloned get method function type");

    let clone_method_return_type =
      first(cmf.ret_types(), true).expect("expected cloned method return type");

    assert!(clone_method_return_type == clone_ty);
  }
}

mod module_deep_clone_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:218:module_deep_clone_intersection`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item module_deep_clone_intersection

  #[cfg(test)]
  #[test]
  fn module_deep_clone_intersection() {
    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, freeze::freeze,
        to_string_to_string_alt_c::to_string_type_id, unfreeze::unfreeze,
      },
      records::{
        clone_state::CloneState, intersection_type::IntersectionType, type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());
    let number_type = fixture.get_builtins().number_type();
    let string_type = fixture.get_builtins().string_type();

    let old_intersection = {
      let frontend = fixture.get_frontend();
      let global_types = frontend.globals.global_types_mut();
      unfreeze(global_types);
      let old_intersection = global_types.add_type(IntersectionType {
        parts: alloc::vec![number_type, string_type],
      });
      freeze(global_types);
      old_intersection
    };

    let new_intersection = unsafe { clone_type(old_intersection, &mut dest, &mut clone_state) };

    assert_ne!(new_intersection, old_intersection);
    assert_eq!("number & string", to_string_type_id(new_intersection));
    assert_eq!(1, dest.types.size());
  }
}

mod module_deep_clone_non_persistent_primitive {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:90:module_deep_clone_non_persistent_primitive`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item module_deep_clone_non_persistent_primitive

  #[cfg(test)]
  #[test]
  fn module_deep_clone_non_persistent_primitive() {
    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, freeze::freeze,
        to_string_to_string_alt_c::to_string_type_id, unfreeze::unfreeze,
      },
      records::{
        clone_state::CloneState,
        primitive_type::{PrimitiveType, Type},
        type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    // Create a new number type that isn't persistent.
    let old_number = {
      let frontend = fixture.get_frontend();
      let global_types = frontend.globals.global_types_mut();
      unfreeze(global_types);
      let old_number = global_types.add_type(PrimitiveType {
        r#type: Type::Number,
        metatable: None,
      });
      freeze(global_types);
      old_number
    };

    let new_number = unsafe { clone_type(old_number, &mut dest, &mut clone_state) };

    assert_ne!(new_number, old_number);
    assert_eq!("number", to_string_type_id(old_number));
    assert_eq!("number", to_string_type_id(new_number));
    assert_eq!(1, dest.types.size());
  }
}

mod module_deep_clone_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:203:module_deep_clone_union`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item module_deep_clone_union

  #[cfg(test)]
  #[test]
  fn module_deep_clone_union() {
    use ulua_analysis::{
      functions::{
        clone_clone_alt_b::clone as clone_type, freeze::freeze,
        to_string_to_string_alt_c::to_string_type_id, unfreeze::unfreeze,
      },
      records::{clone_state::CloneState, type_arena::TypeArena, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());
    let number_type = fixture.get_builtins().number_type();
    let string_type = fixture.get_builtins().string_type();

    let old_union = {
      let frontend = fixture.get_frontend();
      let global_types = frontend.globals.global_types_mut();
      unfreeze(global_types);
      let old_union = global_types.add_type(UnionType {
        options: alloc::vec![number_type, string_type],
      });
      freeze(global_types);
      old_union
    };

    let new_union = unsafe { clone_type(old_union, &mut dest, &mut clone_state) };

    assert_ne!(new_union, old_union);
    assert_eq!("number | string", to_string_type_id(new_union));
    assert_eq!(1, dest.types.size());
  }
}

mod module_do_not_clone_reexports {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:415:module_do_not_clone_reexports`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item module_do_not_clone_reexports

  #[cfg(test)]
  #[test]
  fn module_do_not_clone_reexports() {
    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::table_type::TableType};
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        export type A = {p : number}
        return {}
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        local a = require(script.Parent.A)
        export type B = {q : a.A}
        return {}
    "#,
      ),
    );

    let module_b_name = String::from("Module/B");
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_a_name = String::from("Module/A");
    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_a_name);
    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_b_name);

    let type_a = module_a
      .exported_type_bindings
      .get("A")
      .expect("expected exported type A")
      .r#type();
    let type_b = module_b
      .exported_type_bindings
      .get("B")
      .expect("expected exported type B")
      .r#type();

    let table_b = get_type_id::<TableType>(type_b).expect("expected table type B");
    assert_eq!(
      Some(type_a),
      table_b.props.get("q").and_then(|prop| prop.read_ty)
    );
  }
}

mod module_do_not_clone_types_of_reexported_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:446:module_do_not_clone_types_of_reexported_values`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item module_do_not_clone_types_of_reexported_values

  #[cfg(test)]
  #[test]
  fn module_do_not_clone_types_of_reexported_values() {
    use ulua_analysis::{
      functions::{
        first::first, get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        local exports = {a={p=5}}
        return exports
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        local a = require(script.Parent.A)
        local exports = {b=a.a}
        return exports
    "#,
      ),
    );

    let module_b_name = String::from("Module/B");
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_a_name = String::from("Module/A");
    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_a_name);
    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_b_name);

    let type_a = first(module_a.return_type, true).expect("expected Module/A return type");
    let type_b = first(module_b.return_type, true).expect("expected Module/B return type");

    let table_a = get_type_id::<TableType>(type_a)
      .unwrap_or_else(|| panic!("expected table, got {}", to_string_type_id(type_a)));
    let table_b = get_type_id::<TableType>(type_b)
      .unwrap_or_else(|| panic!("expected table, got {}", to_string_type_id(type_b)));

    let prop_a = table_a.props.get("a").expect("expected property a");
    let prop_b = table_b.props.get("b").expect("expected property b");

    assert_eq!(prop_a.read_ty, prop_b.read_ty);
    assert_eq!(prop_a.write_ty, prop_b.write_ty);
  }
}

mod module_dont_clone_persistent_primitive {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:80:module_dont_clone_persistent_primitive`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item module_dont_clone_persistent_primitive

  #[cfg(test)]
  #[test]
  fn module_dont_clone_persistent_primitive() {
    use ulua_analysis::{
      functions::clone_clone_alt_b::clone as clone_type,
      records::{clone_state::CloneState, type_arena::TypeArena},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let number_type = fixture.get_builtins().number_type();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    // number_type is persistent. We leave it as-is.
    let new_number = unsafe { clone_type(number_type, &mut dest, &mut clone_state) };
    assert_eq!(new_number, number_type);
  }
}

mod module_is_within_comment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:20:module_is_within_comment`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item module_is_within_comment

  #[cfg(test)]
  #[test]
  fn module_is_within_comment() {
    use alloc::string::String;

    use ulua_analysis::functions::is_within_comment_module_alt_b::is_within_comment_source_module_position;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:
        ]] foo:bar()

        --[[]]--[[]] -- Two distinct comments that have zero characters of space between them.
    "#,
    );

    fixture.check_string_optional_frontend_options(&source, None);

    let source_module = fixture.get_main_source_module();
    let source_module = unsafe { &*source_module };

    assert_eq!(5, source_module.comment_locations.len());

    assert!(is_within_comment_source_module_position(
      source_module,
      Position::new(1, 15)
    ));
    assert!(is_within_comment_source_module_position(
      source_module,
      Position::new(6, 16)
    ));
    assert!(is_within_comment_source_module_position(
      source_module,
      Position::new(9, 13)
    ));
    assert!(is_within_comment_source_module_position(
      source_module,
      Position::new(9, 14)
    ));

    assert!(!is_within_comment_source_module_position(
      source_module,
      Position::new(2, 15)
    ));
    assert!(!is_within_comment_source_module_position(
      source_module,
      Position::new(7, 10)
    ));
    assert!(!is_within_comment_source_module_position(
      source_module,
      Position::new(7, 11)
    ));
  }
}

mod module_is_within_comment_parse_result {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:48:module_is_within_comment_parse_result`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
  //!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
  //!   - type_ref -> record ParseOptions (Ast/include/Luau/ParseOptions.h)
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - type_ref -> record Parser (Ast/include/Luau/Parser.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item module_is_within_comment_parse_result

  #[cfg(test)]
  #[test]
  fn module_is_within_comment_parse_result() {
    use alloc::string::String;

    use ulua_analysis::functions::is_within_comment_module_alt_c::is_within_comment_parse_result_position;
    use ulua_ast::records::{
      allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
      parser::Parser, position::Position,
    };

    let src = String::from(
      r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:
        ]] foo:bar()

        --[[]]--[[]] -- Two distinct comments that have zero characters of space between them.
    "#,
    );

    let mut alloc = Allocator::new();
    let mut names = AstNameTable::new(&mut alloc);
    let parse_options = ParseOptions {
      capture_comments: true,
      ..Default::default()
    };
    let parse_result = Parser::parse(&src, src.len(), &mut names, &mut alloc, parse_options);

    assert_eq!(5, parse_result.comment_locations.len());

    assert!(is_within_comment_parse_result_position(
      &parse_result,
      Position::new(1, 15)
    ));
    assert!(is_within_comment_parse_result_position(
      &parse_result,
      Position::new(6, 16)
    ));
    assert!(is_within_comment_parse_result_position(
      &parse_result,
      Position::new(9, 13)
    ));
    assert!(is_within_comment_parse_result_position(
      &parse_result,
      Position::new(9, 14)
    ));

    assert!(!is_within_comment_parse_result_position(
      &parse_result,
      Position::new(2, 15)
    ));
    assert!(!is_within_comment_parse_result_position(
      &parse_result,
      Position::new(7, 10)
    ));
    assert!(!is_within_comment_parse_result_position(
      &parse_result,
      Position::new(7, 11)
    ));
  }
}

mod module_old_solver_correctly_populates_child_scopes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:537:module_old_solver_correctly_populates_child_scopes`
  //! Source: `tests/Module.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Module.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Clone.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Module.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Module.test.cpp
  //! - outgoing:
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
  //!   - translates_to -> rust_item module_old_solver_correctly_populates_child_scopes

  #[cfg(test)]
  #[test]
  fn module_old_solver_correctly_populates_child_scopes() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
if true then
end

if false then
end

if true then
else
end

local x = {}
for i,v in x do
end
"#,
      ),
      None,
    );

    let module_name = String::from("MainModule");
    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_name);
    assert_eq!(7, module.get_module_scope().children.len());
  }
}
