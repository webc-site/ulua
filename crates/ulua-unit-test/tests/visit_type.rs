extern crate alloc;

mod visit_type_can_be_configured_not_to_skip_bound_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:180:visit_type_can_be_configured_not_to_skip_bound_types`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
  //!   - translates_to -> rust_item visit_type_can_be_configured_not_to_skip_bound_types

  #[cfg(test)]
  #[test]
  fn visit_type_can_be_configured_not_to_skip_bound_types() {
    use ulua_analysis::type_aliases::bound_type::BoundType;
    use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let number_type = fixture.get_builtins().number_type;

    let a = fixture.arena.add_type(BoundType::bound_t(number_type));

    let mut vis = TracingVisitor::new(true, false);
    vis.run_type_id(a);

    assert_eq!(2, vis.trace.len());
    assert_eq!("number", vis.trace[0]);
    assert_eq!("number", vis.trace[1]);
  }
}

mod visit_type_detects_cycles {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:144:visit_type_detects_cycles`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> method Variant::emplace (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
  //!   - translates_to -> rust_item visit_type_detects_cycles

  #[cfg(test)]
  #[test]
  fn visit_type_detects_cycles() {
    use alloc::{collections::BTreeMap, string::String};

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        as_mutable_type::as_mutable_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{
        blocked_type::BlockedType, function_type::FunctionType, property_type::Property,
        table_type::TableType, type_level::TypeLevel,
      },
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let (number_type, empty_type_pack) = {
      let builtins = fixture.get_builtins();
      (builtins.number_type, builtins.empty_type_pack)
    };

    let f_type = fixture.arena.add_type(BlockedType::default());

    let mut props = BTreeMap::new();
    props.insert(String::from("method"), Property::rw_type_id(f_type));
    let t_type = fixture.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    );

    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[t_type, number_type]);
    unsafe {
      (*as_mutable_type_id(f_type)).ty = TypeVariant::Function(FunctionType::function_type_new(
        args,
        empty_type_pack,
        None,
        false,
      ));
    }

    let mut vis = TracingVisitor::new(true, true);
    vis.run_type_id(f_type);

    assert_eq!(3, vis.trace.len());
    assert_eq!("t1 where t1 = ({ method: t1 }, number) -> ()", vis.trace[0]);
    assert_eq!("t1 where t1 = { method: (t1, number) -> () }", vis.trace[1]);
    assert_eq!("number", vis.trace[2]);

    assert_eq!(1, vis.cycles.len());
    assert_eq!(
      "t1 where t1 = ({ method: t1 }, number) -> ()",
      to_string_type_id(vis.cycles[0])
    );
  }
}

mod visit_type_dont_throw_when_limit_is_high_enough {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:44:visit_type_dont_throw_when_limit_is_high_enough`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item visit_type_dont_throw_when_limit_is_high_enough

  #[cfg(test)]
  #[test]
  fn visit_type_dont_throw_when_limit_is_high_enough() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FInt::LuauVisitRecursionLimit;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _sfi = ScopedFastInt::new(&LuauVisitRecursionLimit, 8);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t : {a: {b: {c: {d: {e: boolean}}}}}
    "#,
      ),
      None,
    );
    assert!(result.errors.is_empty());

    let t_type = fixture.require_type_string(&String::from("t"));
    let _ = to_string_type_id(t_type);
  }
}

mod visit_type_skip_over_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:131:visit_type_skip_over_tables`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TableSkippingVisitor (tests/VisitType.test.cpp)
  //!   - translates_to -> rust_item visit_type_skip_over_tables

  #[cfg(test)]
  #[test]
  fn visit_type_skip_over_tables() {
    use ulua_unit_test::records::{fixture::Fixture, table_skipping_visitor::TableSkippingVisitor};

    let mut fixture = Fixture::fixture_bool(false);
    let a =
      fixture.parse_type("(number, string, {x: number, y: number}) -> {x: number, y: number}");

    let mut vis = TableSkippingVisitor::new();
    vis.run_type_id(a);

    assert_eq!(3, vis.trace.len());
    assert_eq!(
      "(number, string, { x: number, y: number }) -> { x: number, y: number }",
      vis.trace[0]
    );
    assert_eq!("number", vis.trace[1]);
    assert_eq!("string", vis.trace[2]);
  }
}

mod visit_type_skips_bound_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:169:visit_type_skips_bound_types`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
  //!   - translates_to -> rust_item visit_type_skips_bound_types

  #[cfg(test)]
  #[test]
  fn visit_type_skips_bound_types() {
    use ulua_analysis::type_aliases::bound_type::BoundType;
    use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let number_type = fixture.get_builtins().number_type;

    let a = fixture.arena.add_type(BoundType::bound_t(number_type));

    let mut vis = TracingVisitor::new(true, true);
    vis.run_type_id(a);

    assert_eq!(1, vis.trace.len());
    assert_eq!("number", vis.trace[0]);
  }
}

mod visit_type_some_free_types_do_not_have_bounds {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:57:visit_type_some_free_types_do_not_have_bounds`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - translates_to -> rust_item visit_type_some_free_types_do_not_have_bounds

  #[cfg(test)]
  #[test]
  fn visit_type_some_free_types_do_not_have_bounds() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{free_type::FreeType, r#type::Type, type_level::TypeLevel},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let builtins = fixture.get_builtins();

    let t = Type::from(FreeType {
      level: TypeLevel::default(),
      lower_bound: builtins.never_type,
      upper_bound: builtins.unknown_type,
      ..FreeType::default()
    });

    let _ = to_string_type_id(&t as *const Type);
  }
}

mod visit_type_some_free_types_have_bounds {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:64:visit_type_some_free_types_have_bounds`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item visit_type_some_free_types_have_bounds

  #[cfg(test)]
  #[test]
  fn visit_type_some_free_types_have_bounds() {
    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{free_type::FreeType, scope::Scope, r#type::Type},
    };
    use ulua_common::FFlag::DebugLuauForceOldSolver;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let builtins = fixture.get_builtins();
    let mut scope = Scope::scope_type_pack_id(builtins.any_type_pack);

    let t = Type::from(FreeType::free_type_scope_type_id_type_id_polarity(
      &mut scope,
      builtins.never_type,
      builtins.number_type,
      Polarity::Unknown,
    ));

    assert_eq!("('a <: number)", to_string_type_id(&t as *const Type));
  }
}

mod visit_type_throw_when_limit_is_exceeded {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:18:visit_type_throw_when_limit_is_exceeded`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record RecursionLimitException (Analysis/include/Luau/RecursionCounter.h)
  //!   - translates_to -> rust_item visit_type_throw_when_limit_is_exceeded

  #[cfg(test)]
  #[test]
  fn visit_type_throw_when_limit_is_exceeded() {
    use alloc::string::String;
    use std::panic::catch_unwind;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::{FFlag::DebugLuauForceOldSolver, FInt::LuauVisitRecursionLimit};
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let mut fixture = Fixture::fixture_bool(false);

    if !DebugLuauForceOldSolver.get() {
      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
            local t : {a: {b: {c: {d: {e: boolean}}}}}
        "#,
        ),
        None,
      );
      assert!(result.errors.is_empty());

      let _sfi = ScopedFastInt::new(&LuauVisitRecursionLimit, 3);
      let t_type = fixture.require_type_string(&String::from("t"));

      assert!(catch_unwind(|| to_string_type_id(t_type)).is_err());
    } else {
      let _sfi = ScopedFastInt::new(&LuauVisitRecursionLimit, 3);

      let result = fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
            local t : {a: {b: {c: {d: {e: boolean}}}}}
        "#,
        ),
        None,
      );
      assert!(result.errors.is_empty());

      let t_type = fixture.require_type_string(&String::from("t"));

      assert!(catch_unwind(|| to_string_type_id(t_type)).is_err());
    }
  }
}

mod visit_type_trace_a_simple_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:96:visit_type_trace_a_simple_function`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
  //!   - translates_to -> rust_item visit_type_trace_a_simple_function

  #[cfg(test)]
  #[test]
  fn visit_type_trace_a_simple_function() {
    use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

    let mut fixture = Fixture::fixture_bool(false);
    let a = fixture.parse_type("(number, string) -> boolean");

    let mut vis = TracingVisitor::new(true, true);
    vis.run_type_id(a);

    assert_eq!(4, vis.trace.len());
    assert_eq!("(number, string) -> boolean", vis.trace[0]);
    assert_eq!("number", vis.trace[1]);
    assert_eq!("string", vis.trace[2]);
    assert_eq!("boolean", vis.trace[3]);
  }
}

mod visit_type_visit_once {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:192:visit_type_visit_once`
  //! Source: `tests/VisitType.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/VisitType.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
  //! - incoming:
  //!   - declares <- source_file tests/VisitType.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
  //!   - translates_to -> rust_item visit_type_visit_once

  #[cfg(test)]
  #[test]
  fn visit_type_visit_once() {
    use alloc::{collections::BTreeMap, string::String};

    use ulua_analysis::{
      enums::table_state::TableState,
      records::{
        function_type::FunctionType, property_type::Property, table_type::TableType,
        type_level::TypeLevel,
      },
    };
    use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let number_type = fixture.get_builtins().number_type;

    let mut props = BTreeMap::new();
    props.insert(String::from("x"), Property::rw_type_id(number_type));
    let x_table = fixture.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    );

    let arg_pack = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[x_table, x_table]);
    let ret_pack = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[x_table]);
    let fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
      arg_pack, ret_pack, None, false,
    ));

    {
      let mut vis = TracingVisitor::new(true, true);
      vis.run_type_id(fn_ty);

      assert_eq!(3, vis.trace.len());
      assert_eq!(
        "({ x: number }, { x: number }) -> { x: number }",
        vis.trace[0]
      );
      assert_eq!("{ x: number }", vis.trace[1]);
      assert_eq!("number", vis.trace[2]);
      assert_eq!(0, vis.cycles.len());
    }

    {
      let mut vis = TracingVisitor::new(false, true);
      vis.run_type_id(fn_ty);

      assert_eq!(7, vis.trace.len());
      assert_eq!(
        "({ x: number }, { x: number }) -> { x: number }",
        vis.trace[0]
      );
      assert_eq!("{ x: number }", vis.trace[1]);
      assert_eq!("{ x: number }", vis.trace[2]);
      assert_eq!("{ x: number }", vis.trace[3]);
      assert_eq!("number", vis.trace[4]);
      assert_eq!("number", vis.trace[5]);
      assert_eq!("number", vis.trace[6]);
      assert_eq!(0, vis.cycles.len());
    }
  }
}
