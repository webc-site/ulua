extern crate alloc;

mod type_infer_oop_assign_to_prop_of_intersection_of_metatables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:790:type_infer_oop_assign_to_prop_of_intersection_of_metatables`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_assign_to_prop_of_intersection_of_metatables

  #[cfg(test)]
  #[test]
  fn type_infer_oop_assign_to_prop_of_intersection_of_metatables() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fix_prop_reads = ScopedFastFlag::new(&FFlag::LuauFixPropReadsOnMetatableTypes, true);
    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local Base = {}
        Base.__index = Base

        type BaseStructure = { BaseString: string }

        export type Base = setmetatable<BaseStructure, typeof(Base)>

        function Base.new() : Base
            return nil :: any
        end

        local Sub = {}
        Sub.__index = Sub

        type SubStructure = { SubString: string }

        type Sub = setmetatable<SubStructure, typeof(Sub)> & Base

        function Sub.new() : Sub
            local self: Sub = setmetatable(Base.new(), Sub) :: any

            self.SubString = 5 -- Line 24
            self.BaseString = 5 -- Line 25

            return self
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(24, result.errors[0].location.begin.line);
    get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!(25, result.errors[1].location.begin.line);
  }
}

mod type_infer_oop_augmenting_an_unsealed_table_with_a_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:387:type_infer_oop_augmenting_an_unsealed_table_with_a_metatable`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_oop_augmenting_an_unsealed_table_with_a_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_oop_augmenting_an_unsealed_table_with_a_metatable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local A = {number = 8}

        local B = setmetatable({}, A)

        function B:method()
            return "hello!!"
        end
    "#,
      ),
      None,
    );

    let mut opts = ToStringOptions::new(true);
    let b_type = fixture.base.require_type_string(&String::from("B"));
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ @metatable { number: number }, { method: (unknown) -> string } }"
    } else {
      "{ @metatable {| number: number |}, {| method: <a>(a) -> string |} }"
    };
    assert_eq!(
      expected,
      to_string_type_id_to_string_options(b_type, &mut opts)
    );
  }
}

mod type_infer_oop_check_methods_of_sealed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:245:type_infer_oop_check_methods_of_sealed`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_oop_check_methods_of_sealed

  #[cfg(test)]
  #[test]
  fn type_infer_oop_check_methods_of_sealed() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x: {prop: number} = {prop=9999}
function x:y(z: number)
    local s: string = z
end
"#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_class_decl {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:875:type_infer_oop_class_decl`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_oop_class_decl

  #[cfg(test)]
  #[test]
  fn type_infer_oop_class_decl() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::extern_type::ExternType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number
            public y: number
        end

        local p = Point.new { x = 2, y = 3 }

        local x = p.x
        local y = p.y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.require_type_alias(&String::from("Point"));
    assert_eq!("Point", to_string_type_id(t));

    assert!(
      get_type_id::<ExternType>(t).is_some(),
      "expected Point alias to have ExternType"
    );

    assert_eq!(
      "Point",
      to_string_type_id(fixture.require_type_string(&String::from("p")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_oop_class_that_shadows_a_type_alias {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1129:type_infer_oop_class_that_shadows_a_type_alias`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record DuplicateTypeDefinition (Analysis/include/Luau/Error.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method Lexer::previous_location (Ast/include/Luau/Lexer.h)
  //!   - translates_to -> rust_item type_infer_oop_class_that_shadows_a_type_alias

  #[cfg(test)]
  #[test]
  fn type_infer_oop_class_that_shadows_a_type_alias() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type AAA = { x: number }
        class AAA end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
      .expect("expected DuplicateTypeDefinition");
    assert_eq!("AAA", err.name());
    assert!(err.previous_location().is_some());
  }
}

mod type_infer_oop_classes_arent_in_old_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:836:type_infer_oop_classes_arent_in_old_solver`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::keyword (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_oop_classes_arent_in_old_solver

  #[cfg(test)]
  #[test]
  fn type_infer_oop_classes_arent_in_old_solver() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result =
      fixture.check_string_optional_frontend_options(&String::from(" class Point end "), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!("class keyword is illegal here", err.message());
  }
}

mod type_infer_oop_cross_module_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:573:type_infer_oop_cross_module_metatable`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
  //!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_oop_cross_module_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_oop_cross_module_metatable() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        local cls = {}
        cls.__index = cls
        function cls:abc() return 4 end
        return cls
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local cls = require(game.A)
        local tbl = {}
        setmetatable(tbl, cls)
    "#,
      ),
    );

    let module_b_name = String::from("game/B");
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&module_b_name);
    let module_scope = module_b.get_module_scope();
    let cls_binding = module_scope
      .linear_search_for_binding(&String::from("tbl"), false)
      .expect("expected binding for tbl");

    assert_eq!(
      "{ @metatable cls, tbl }",
      to_string_type_id(cls_binding.type_id)
    );
  }
}

mod type_infer_oop_cycle_between_object_constructor_and_alias {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:445:type_infer_oop_cycle_between_object_constructor_and_alias`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_oop_cycle_between_object_constructor_and_alias

  #[cfg(test)]
  #[test]
  fn type_infer_oop_cycle_between_object_constructor_and_alias() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::metatable_type::MetatableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        T.__index = T

        function T.new(): T
            return setmetatable({}, T)
        end

        export type T = typeof(T.new())

        return T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.base.get_main_module(false) };
    let alias_type = module
      .exported_type_bindings
      .get("T")
      .expect("expected exported type T")
      .r#type();
    let followed = follow_type_id(alias_type);
    let metatable = get_type_id::<MetatableType>(followed);
    assert!(
      metatable.is_some(),
      "expected metatable type, got {}",
      to_string_type_id(alias_type)
    );
  }
}

mod type_infer_oop_dont_bind_free_tables_to_themselves {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:352:type_infer_oop_dont_bind_free_tables_to_themselves`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_oop_dont_bind_free_tables_to_themselves

  #[cfg(test)]
  #[test]
  fn type_infer_oop_dont_bind_free_tables_to_themselves() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        local b: any

        function T:m()
            local a = b[i]
            if a then
                self:n()
                if self:p(a) then
                    self:n()
                end
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:75:type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works

  #[cfg(test)]
  #[test]
  fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {method: ((T, number) -> number) & ((number) -> number)}
        local T: T

        T.method(4)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:50:type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2

  #[cfg(test)]
  #[test]
  fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local someTable = {}

        local function abs(x: number)
            if x < 0 then
                return -x
            else
                return x
            end
        end

        someTable.Function2 = function(Arg1, Arg2)
            abs(Arg1)
            abs(Arg2)
        end

        someTable.Function2() -- Argument count mismatch
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<CountMismatch>(&result.errors[0]);
    assert!(
      err.is_some(),
      "expected CountMismatch, got {:?}",
      result.errors[0]
    );
  }
}

mod type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_not_defined_with_colon {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:26:type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_not_defined_with_colon`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_not_defined_with_colon

  #[cfg(test)]
  #[test]
  fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_not_defined_with_colon() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local someTable = {}

        local function abs(x: number)
            if x < 0 then
                return -x
            else
                return x
            end
        end

        someTable.Function1 = function(Arg1)
            abs(Arg1)
        end

        someTable.Function1() -- Argument count mismatch
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<CountMismatch>(&result.errors[0]);
    assert!(
      err.is_some(),
      "expected CountMismatch, got {:?}",
      result.errors[0]
    );
  }
}

mod type_infer_oop_empty_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:864:type_infer_oop_empty_class`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_oop_empty_class

  #[cfg(test)]
  #[test]
  fn type_infer_oop_empty_class() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result =
      fixture.check_string_optional_frontend_options(&String::from(" class Point end "), None);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_oop_export_class_isnt_in_old_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:850:type_infer_oop_export_class_isnt_in_old_solver`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::keyword (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_oop_export_class_isnt_in_old_solver

  #[cfg(test)]
  #[test]
  fn type_infer_oop_export_class_isnt_in_old_solver() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture
      .check_string_optional_frontend_options(&String::from(" export class Point end "), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!("class keyword is illegal here", err.message());
  }
}

mod type_infer_oop_flag_when_index_metamethod_returns_0_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:371:type_infer_oop_flag_when_index_metamethod_returns_0_values`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_oop_flag_when_index_metamethod_returns_0_values

  #[cfg(test)]
  #[test]
  fn type_infer_oop_flag_when_index_metamethod_returns_0_values() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}
        function T.__index()
        end

        local a = setmetatable({}, T)
        local p = a.prop
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_string(&String::from("p")))
    );
  }
}

mod type_infer_oop_fuzzer_duplicate_class_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:968:type_infer_oop_fuzzer_duplicate_class_definition`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_fuzzer_duplicate_class_definition

  #[cfg(test)]
  #[test]
  fn type_infer_oop_fuzzer_duplicate_class_definition() {
    use alloc::string::String;

    use ulua_analysis::records::syntax_error::SyntaxError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        class l0
        end
        class l0
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
      "A class named 'l0' has already been declared in this module",
      err.message()
    );
  }
}

mod type_infer_oop_fuzzer_self_referential_class_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1063:type_infer_oop_fuzzer_self_referential_class_definition`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_oop_fuzzer_self_referential_class_definition

  #[cfg(test)]
  #[test]
  fn type_infer_oop_fuzzer_self_referential_class_definition() {
    use alloc::string::String;

    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::extern_type::ExternType};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        class l0
            public _:typeof(l0)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let l0 = fixture.require_type_string(&String::from("l0"));
    assert!(
      get_type_id::<ExternType>(l0).is_some(),
      "expected l0 to have ExternType"
    );
  }
}

mod type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:273:type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table

  #[cfg(test)]
  #[test]
  fn type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function Base64FileReader(data)
            local reader = {}
            local index: number = 0

            function reader:PeekByte()
                return data:byte(index)
            end

            function reader:Byte()
                return data:byte(index - 1)
            end

            return reader
        end

        Base64FileReader()

        function ReadMidiEvents(data)

            local reader = Base64FileReader(data)

            while reader:HasMore() do
                (reader:Byte() % 128)
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:142:type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory

  #[cfg(test)]
  #[test]
  fn type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        ("foo")
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
    "#,
      ),
      None,
    );

    let module = fixture.get_main_module(false);
    let type_count = unsafe { (*module).internal_types.types.size() };
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(80 >= type_count, "type count was {}", type_count);
    } else {
      assert!(50 >= type_count, "type count was {}", type_count);
    }
  }
}

mod type_infer_oop_instantiate_duplicate_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1079:type_infer_oop_instantiate_duplicate_class`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_instantiate_duplicate_class

  #[cfg(test)]
  #[test]
  fn type_infer_oop_instantiate_duplicate_class() {
    use alloc::string::String;

    use ulua_analysis::records::{
      cannot_call_non_function::CannotCallNonFunction, syntax_error::SyntaxError,
      unknown_symbol::UnknownSymbol,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
class l0
end
class l0
end
_ = l0 {  }
"#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
      "A class named 'l0' has already been declared in this module",
      err.message()
    );
    type_error_data_ref::<UnknownSymbol>(&result.errors[1]).expect("expected UnknownSymbol");
    type_error_data_ref::<CannotCallNonFunction>(&result.errors[2])
      .expect("expected CannotCallNonFunction");
  }
}

mod type_infer_oop_metatable_field_allows_upcast {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:729:type_infer_oop_metatable_field_allows_upcast`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_oop_metatable_field_allows_upcast

  #[cfg(test)]
  #[test]
  fn type_infer_oop_metatable_field_allows_upcast() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Foobar = {}
        Foobar.__index = Foobar
        Foobar.const = 42

        local foobar = setmetatable({}, Foobar)

        local _: { read const: number } = foobar
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_metatable_field_disallows_invalid_upcast {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:746:type_infer_oop_metatable_field_disallows_invalid_upcast`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_metatable_field_disallows_invalid_upcast

  #[cfg(test)]
  #[test]
  fn type_infer_oop_metatable_field_disallows_invalid_upcast() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Foobar = {}
        Foobar.__index = Foobar
        Foobar.const = 42

        local foobar = setmetatable({}, Foobar)

        local _: { const: number } = foobar
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{ const: number }", to_string_type_id(err.wanted_type));
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ @metatable t1, {  } } where t1 = { __index: t1, const: number }",
      to_string_type_id_to_string_options(err.given_type, &mut opts)
    );
  }
}

mod type_infer_oop_metatable_field_precedence_for_subtyping {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:767:type_infer_oop_metatable_field_precedence_for_subtyping`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_metatable_field_precedence_for_subtyping

  #[cfg(test)]
  #[test]
  fn type_infer_oop_metatable_field_precedence_for_subtyping() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foobar1(_: { read foo: number }) end
        local function foobar2(_: { read bar: boolean }) end
        local function foobar3(_: { read foo: string }) end

        local t = { foo = 4 }
        setmetatable(t, { __index = { foo = "heh", bar = true }})
        foobar1(t)
        foobar2(t)
        foobar3(t)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ read foo: string }",
      to_string_type_id_to_string_options(err.wanted_type, &mut opts)
    );
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ @metatable { __index: { bar: boolean, foo: string } }, { foo: number } }",
      to_string_type_id_to_string_options(err.given_type, &mut opts)
    );
  }
}

mod type_infer_oop_method_depends_on_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:87:type_infer_oop_method_depends_on_table`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function toposort (tests/TopoSort.test.cpp)
  //!   - translates_to -> rust_item type_infer_oop_method_depends_on_table

  #[cfg(test)]
  #[test]
  fn type_infer_oop_method_depends_on_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        -- This catches a bug where x:m didn't count as a use of x
        -- so toposort would happily reorder a definition of
        -- function x:m before the definition of x.
        function g() f() end
        local x = {}
        function x:m() end
        function f() x:m() end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_method_should_not_create_cyclic_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:556:type_infer_oop_method_should_not_create_cyclic_type`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias Component (Analysis/include/Luau/TypePath.h)
  //!   - translates_to -> rust_item type_infer_oop_method_should_not_create_cyclic_type

  #[cfg(test)]
  #[test]
  fn type_infer_oop_method_should_not_create_cyclic_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Component = {}

        function Component:__resolveUpdate(incomingState)
            local oldState = self.state
            incomingState = oldState
            self.state = incomingState
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_methods_are_topologically_sorted {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:102:type_infer_oop_methods_are_topologically_sorted`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_oop_methods_are_topologically_sorted

  #[cfg(test)]
  #[test]
  fn type_infer_oop_methods_are_topologically_sorted() {
    use alloc::string::String;

    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}

        function T:foo()
            return T:bar(999), T:bar("hi")
        end

        function T:bar(i)
            return i
        end

        local a, b = T:foo()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let a_type = fixture.require_type_string(&String::from("a"));
    let b_type = fixture.require_type_string(&String::from("b"));
    assert_eq!(
      Some(PrimitiveType::NUMBER),
      fixture.get_primitive_type(a_type)
    );
    assert_eq!(
      Some(PrimitiveType::STRING),
      fixture.get_primitive_type(b_type)
    );
  }
}

mod type_infer_oop_nonstrict_self_mismatch_tail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:257:type_infer_oop_nonstrict_self_mismatch_tail`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_oop_nonstrict_self_mismatch_tail

  #[cfg(test)]
  #[test]
  fn type_infer_oop_nonstrict_self_mismatch_tail() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict
        local f = {}
        function f:foo(a: number, b: number) end

        function bar(...)
            f.foo(f, 1, ...)
        end

        bar(2)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_object_constructor_can_refer_to_method_of_self {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:203:type_infer_oop_object_constructor_can_refer_to_method_of_self`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_oop_object_constructor_can_refer_to_method_of_self

  #[cfg(test)]
  #[test]
  fn type_infer_oop_object_constructor_can_refer_to_method_of_self() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        type Foo = {
            fooConn: () -> () | nil
        }

        local Foo = {}
        Foo.__index = Foo

        function Foo.new()
            local self: Foo = {
                fooConn = nil,
            }
            setmetatable(self, Foo)

            self.fooConn = function()
                self:method() -- Key 'method' not found in table self
            end

            return self
        end

        function Foo:method()
            print("foo")
        end

        local foo = Foo.new()

        -- TODO This is the best our current refinement support can offer :(
        local bar = foo.fooConn
        if bar then bar() end

        -- foo.fooConn()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_oop_invoke_with_inferred_self_and_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:703:type_infer_oop_oop_invoke_with_inferred_self_and_property`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_oop_oop_invoke_with_inferred_self_and_property

  #[cfg(test)]
  #[test]
  fn type_infer_oop_oop_invoke_with_inferred_self_and_property() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local ItemContainer = {}
        ItemContainer.__index = ItemContainer

        function ItemContainer.new(name)
            local self = {name = name}
            setmetatable(self, ItemContainer)
            return self
        end

        function ItemContainer:removeItem(itemId, itemType)
            print(self.name)
            self:getItem(itemId, itemType)
        end

        function ItemContainer:getItem(itemId, itemType): ()
        end

        local container = ItemContainer.new("library")

        container:removeItem(0, "magic")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_oop_invoke_with_inferred_self_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:678:type_infer_oop_oop_invoke_with_inferred_self_type`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_oop_oop_invoke_with_inferred_self_type

  #[cfg(test)]
  #[test]
  fn type_infer_oop_oop_invoke_with_inferred_self_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local ItemContainer = {}
        ItemContainer.__index = ItemContainer

        function ItemContainer.new()
            local self = {}
            setmetatable(self, ItemContainer)
            return self
        end

        function ItemContainer:removeItem(itemId, itemType)
            self:getItem(itemId, itemType)
        end

        function ItemContainer:getItem(itemId, itemType): ()
        end

        local container = ItemContainer.new()

        container:removeItem(0, "magic")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_pass_too_many_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:173:type_infer_oop_pass_too_many_arguments`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_oop_pass_too_many_arguments

  #[cfg(test)]
  #[test]
  fn type_infer_oop_pass_too_many_arguments() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {
            method: (T, number) -> number
        }

        function makeT(): T
            return {
                method=function(self, number)
                    return number * 2
                end
            }
        end

        local a = makeT()
        a:method(5, 7)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let count_mismatch =
      get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(2, count_mismatch.expected());
    assert_eq!(3, count_mismatch.actual());
  }
}

mod type_infer_oop_point_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:907:type_infer_oop_point_class`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_oop_point_class

  #[cfg(test)]
  #[test]
  fn type_infer_oop_point_class() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::extern_type::ExternType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number
            public y: number

            function length(self)
                return 100
            end

            function __init(self, x: number, y: number)
                self.x = x
                self.y = y
            end
        end

        local p = Point.new(2, 3)
        local len = p:length()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let p = fixture.require_type_string(&String::from("p"));
    assert!(
      get_type_id::<ExternType>(p).is_some(),
      "expected p to have ExternType"
    );

    assert_eq!("Point", to_string_type_id(p));
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("len")))
    );
  }
}

mod type_infer_oop_promise_type_error_too_complex {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:470:type_infer_oop_promise_type_error_too_complex`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_oop_promise_type_error_too_complex

  #[cfg(test)]
  #[test]
  fn type_infer_oop_promise_type_error_too_complex() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local Promise = {}
        Promise.prototype = {}
        Promise.__index = Promise.prototype

        function Promise._new(traceback, callback, parent)
            if parent ~= nil and not Promise.is(parent)then
            end

            local self = {
                _parent = parent,
            }

            parent._consumers[self] = true
            setmetatable(self, Promise)
            self:_reject()

            return self
        end

        function Promise.resolve(...)
            return Promise._new(debug.traceback(nil, 2), function(resolve)
            end)
        end

        function Promise.reject(...)
            return Promise._new(debug.traceback(nil, 2), function(_, reject)
            end)
        end

        function Promise._try(traceback, callback, ...)
            return Promise._new(traceback, function(resolve)
            end)
        end

        function Promise.try(callback, ...)
            return Promise._try(debug.traceback(nil, 2), callback, ...)
        end

        function Promise._all(traceback, promises, amount)
            if #promises == 0 or amount == 0 then
                return Promise.resolve({})
            end
            return Promise._new(traceback, function(resolve, reject, onCancel)
            end)
        end

        function Promise.all(promises)
            return Promise._all(debug.traceback(nil, 2), promises)
        end

        function Promise.allSettled(promises)
            return Promise.resolve({})
        end

        function Promise.race(promises)
            return Promise._new(debug.traceback(nil, 2), function(resolve, reject, onCancel)
            end)
        end

        function Promise.each(list, predicate)
            return Promise._new(debug.traceback(nil, 2), function(resolve, reject, onCancel)
                local predicatePromise = Promise.resolve(predicate(value, index))
                local success, result = predicatePromise:await()
            end)
        end

        function Promise.is(object)
        end

        function Promise.prototype:_reject(...)
            self:_finalize()
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_oop_prop_with_typeof_reassigned_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1103:type_infer_oop_prop_with_typeof_reassigned_class`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item type_infer_oop_prop_with_typeof_reassigned_class

  #[cfg(test)]
  #[test]
  fn type_infer_oop_prop_with_typeof_reassigned_class() {
    use alloc::string::String;

    use ulua_analysis::records::syntax_error::SyntaxError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _export_value = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
class Animal end
Animal = nil
class l0
public _:typeof(Animal)
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
      "Variable 'Animal' is constant and may not be reassigned",
      err.message()
    );
  }
}

mod type_infer_oop_quantify_methods_defined_using_dot_syntax_and_explicit_self_parameter {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:125:type_infer_oop_quantify_methods_defined_using_dot_syntax_and_explicit_self_parameter`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_oop_quantify_methods_defined_using_dot_syntax_and_explicit_self_parameter

  #[cfg(test)]
  #[test]
  fn type_infer_oop_quantify_methods_defined_using_dot_syntax_and_explicit_self_parameter() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}

        function T.method(self)
            self:method()
        end

        function T.method2(self)
            self:method()
        end

        T:method2()
    "#,
      ),
      None,
    );
  }
}

mod type_infer_oop_react_style_oo {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:405:type_infer_oop_react_style_oo`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_oop_react_style_oo

  #[cfg(test)]
  #[test]
  fn type_infer_oop_react_style_oo() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Prototype = {}

        local ClassMetatable = {
            __index = Prototype
        }

        local BaseClass = (setmetatable({}, ClassMetatable))

        function BaseClass:extend(name)
            local class = {
                name=name
            }

            class.__index = class

            function class.ctor(props)
                return setmetatable({props=props}, class)
            end

            return setmetatable(class, getmetatable(self))
        end

        local C = BaseClass:extend('C')
        local i = C.ctor({hello='world'})

        local iName = i.name
        local cName = C.name
        local hello = i.props.hello
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("iName")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("cName")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("hello")))
    );
  }
}

mod type_infer_oop_read_unknown_property_from_class_object_or_instance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1205:type_infer_oop_read_unknown_property_from_class_object_or_instance`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_read_unknown_property_from_class_object_or_instance

  #[cfg(test)]
  #[test]
  fn type_infer_oop_read_unknown_property_from_class_object_or_instance() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_property::UnknownProperty;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let _access_violation = ScopedFastFlag::new(&FFlag::LuauTweakAccessViolationReporting, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number
            public y: number

            function zero()
                return Point.new {x=0, y=0}
            end
        end

        local p = Point.zero()
        local a = p.z
        local b = Point.z
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    let up0 =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("z", up0.key());
    let up1 =
      type_error_data_ref::<UnknownProperty>(&result.errors[1]).expect("expected UnknownProperty");
    assert_eq!("z", up1.key());
  }
}

mod type_infer_oop_repeat_class_methods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1009:type_infer_oop_repeat_class_methods`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_repeat_class_methods

  #[cfg(test)]
  #[test]
  fn type_infer_oop_repeat_class_methods() {
    use alloc::string::String;

    use ulua_analysis::records::syntax_error::SyntaxError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
class l0
    function foo()
    end
    function foo()
    end
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!("Duplicate class member 'foo'", err.message());
  }
}

mod type_infer_oop_repeat_nameless_class_methods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1033:type_infer_oop_repeat_nameless_class_methods`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::identifier (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_infer_oop_repeat_nameless_class_methods

  #[cfg(test)]
  #[test]
  fn type_infer_oop_repeat_nameless_class_methods() {
    use alloc::string::String;

    use ulua_analysis::records::syntax_error::SyntaxError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
class l0
    function  ()
    end
    function ()
    end
end
"#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    let err1 = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
      "Expected identifier when parsing method name, got '('",
      err1.message()
    );
    let err2 = type_error_data_ref::<SyntaxError>(&result.errors[1]).expect("expected SyntaxError");
    assert_eq!(
      "Expected identifier when parsing method name, got '('",
      err2.message()
    );
    let err3 = type_error_data_ref::<SyntaxError>(&result.errors[2]).expect("expected SyntaxError");
    assert_eq!("Duplicate class member '%error-id%'", err3.message());
  }
}

mod type_infer_oop_repeat_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:988:type_infer_oop_repeat_props`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_repeat_props

  #[cfg(test)]
  #[test]
  fn type_infer_oop_repeat_props() {
    use alloc::string::String;

    use ulua_analysis::records::syntax_error::SyntaxError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
class l0
    public foo
    public foo
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!("Duplicate class member 'foo'", err.message());
  }
}

mod type_infer_oop_self_argument_has_self_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:945:type_infer_oop_self_argument_has_self_type`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_oop_self_argument_has_self_type

  #[cfg(test)]
  #[test]
  fn type_infer_oop_self_argument_has_self_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        class I
            function m(self)
                return self
            end
        end

        local i = I.new{}
        local i2 = i:m()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "I",
      to_string_type_id(fixture.require_type_string(&String::from("i2")))
    );
  }
}

mod type_infer_oop_set_prop_of_intersection_containing_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:333:type_infer_oop_set_prop_of_intersection_containing_metatable`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_oop_set_prop_of_intersection_containing_metatable

  #[cfg(test)]
  #[test]
  fn type_infer_oop_set_prop_of_intersection_containing_metatable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Set<T> = typeof(setmetatable(
            {} :: {
                add: (self: Set<T>, T) -> Set<T>,
            },
            {}
        ))

        local Set = {} :: Set<any> & {}

        function Set:add(t)
            return self
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_oop_table_oop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:304:type_infer_oop_table_oop`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_oop_table_oop

  #[cfg(test)]
  #[test]
  fn type_infer_oop_table_oop() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
   --!strict
local Class = {}
Class.__index = Class

type Class = typeof(setmetatable({} :: { x: number }, Class))

function Class.new(x: number): Class
    return setmetatable({x = x}, Class)
end

function Class.getx(self: Class)
    return self.x
end

function test()
    local c = Class.new(42)
    local n = c:getx()
    local nn = c.x

    print(string.format("%d %d", n, nn))
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_textbook_class_pattern {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:605:type_infer_oop_textbook_class_pattern`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_oop_textbook_class_pattern

  #[cfg(test)]
  #[test]
  fn type_infer_oop_textbook_class_pattern() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Account = {}
        Account.__index = Account

        type AccountData = {
            name: string,
            balance: number,
        }

        export type Account = setmetatable<AccountData, typeof(Account)>

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance

            return setmetatable(self, Account)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_textbook_class_pattern_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:633:type_infer_oop_textbook_class_pattern_2`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_textbook_class_pattern_2

  #[cfg(test)]
  #[test]
  fn type_infer_oop_textbook_class_pattern_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Account = {}
        Account.__index = Account

        type AccountData = {
            name: string,
            balance: number,
        }

        export type Account = setmetatable<AccountData, typeof(Account)>

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance

            return setmetatable(self, Account)
        end

        function Account.deposit(self: Account, credit: number)
            self.balance += credit
        end

        function Account.withdraw(self: Account, debit: number)
            self.balance -= debit
        end

        function Account.hasBalance(self: Account, amount: number): boolean
            return self.balance >= amount
        end

        local account = Account.new("Hina", 500)

        if account:hasBalance(123) then -- TypeError: Value of type 'unknown' could be nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_oop_typecheck_class_annotations {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1177:type_infer_oop_typecheck_class_annotations`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_typecheck_class_annotations

  #[cfg(test)]
  #[test]
  fn type_infer_oop_typecheck_class_annotations() {
    use alloc::string::String;

    use ulua_analysis::records::{
      type_mismatch::TypeMismatch, type_pack_mismatch::TypePackMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number
            public y: number
            public name: string
            function magnitude(self): string
                -- self.name is not a number
                self.name = self.x

                -- This function is declared to return string.
                return math.sqrt(self.x * self.x + self.y * self.y)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(
      result
        .errors
        .iter()
        .any(|err| type_error_data_ref::<TypeMismatch>(err).is_some())
    );
    assert!(
      result
        .errors
        .iter()
        .any(|err| type_error_data_ref::<TypePackMismatch>(err).is_some())
    );
  }
}

mod type_infer_oop_typecheck_class_method_field_access {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1149:type_infer_oop_typecheck_class_method_field_access`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_typecheck_class_method_field_access

  #[cfg(test)]
  #[test]
  fn type_infer_oop_typecheck_class_method_field_access() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::uninhabited_type_function::UninhabitedTypeFunction,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number?
            public y: number?
            function magnitude(self)
                return math.sqrt(self.x * self.x + self.y * self.y)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    for err in &result.errors {
      let utf = type_error_data_ref::<UninhabitedTypeFunction>(err)
        .expect("expected UninhabitedTypeFunction");
      assert_eq!("mul<number?, number?>", to_string_type_id(utf.ty()));
    }
  }
}

mod type_infer_oop_writes_to_class_object_properties_are_forbidden {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1240:type_infer_oop_writes_to_class_object_properties_are_forbidden`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_writes_to_class_object_properties_are_forbidden

  #[cfg(test)]
  #[test]
  fn type_infer_oop_writes_to_class_object_properties_are_forbidden() {
    use alloc::string::String;

    use ulua_analysis::records::property_access_violation::{
      PropertyAccessViolation, PropertyAccessViolation_Context,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let _access_violation = ScopedFastFlag::new(&FFlag::LuauTweakAccessViolationReporting, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number
            public y: number

            function zero()
                return Point.new {x=0, y=0}
            end

            function magnitude(self): number
                return 5 -- stochastic approximation for performance
            end
        end

        Point.magnitude = function(p: Point) return 3 end
        Point.zero = function() return Point.new { x = 1, y = 1 } end
        Point.one = function() return Point.new { x = 1, y = 1 } end
    "#,
      ),
      None,
    );

    let expected = ["magnitude", "zero", "one"];
    assert_eq!(expected.len(), result.errors.len(), "{:?}", result.errors);
    for (err, key) in result.errors.iter().zip(expected) {
      let pav = type_error_data_ref::<PropertyAccessViolation>(err)
        .expect("expected PropertyAccessViolation");
      assert_eq!(key, pav.key());
      assert_eq!(PropertyAccessViolation_Context::CannotWrite, pav.context());
    }
  }
}

mod type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1299:type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden`
  //! Source: `tests/TypeInfer.oop.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.oop.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.oop.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden

  #[cfg(test)]
  #[test]
  fn type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden() {
    use alloc::string::String;

    use ulua_analysis::records::property_access_violation::{
      PropertyAccessViolation, PropertyAccessViolation_Context,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let _access_violation = ScopedFastFlag::new(&FFlag::LuauTweakAccessViolationReporting, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        class Point
            public x: number
            public y: number

            function zero()
                return Point.new {x=0, y=0}
            end

            function magnitude(self): number
                return 5 -- stochastic approximation for performance
            end
        end

        local p = Point.zero()

        p.magnitude = function(p: Point) return 3 end
        p.zero = function() return Point.new { x = 1, y = 1 } end
        p.one = function() return Point.new { x = 1, y = 1 } end

        p.__index = {}
    "#,
      ),
      None,
    );

    let expected = ["magnitude", "zero", "one", "__index"];
    assert_eq!(expected.len(), result.errors.len(), "{:?}", result.errors);
    for (err, key) in result.errors.iter().zip(expected) {
      let pav = type_error_data_ref::<PropertyAccessViolation>(err)
        .expect("expected PropertyAccessViolation");
      assert_eq!(key, pav.key());
      assert_eq!(PropertyAccessViolation_Context::CannotWrite, pav.context());
    }
  }
}
