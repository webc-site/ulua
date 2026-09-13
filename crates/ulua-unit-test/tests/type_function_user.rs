extern crate alloc;

mod type_function_user_blocking_nested_pending_expansions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2261:type_function_user_blocking_nested_pending_expansions`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_blocking_nested_pending_expansions

  #[cfg(test)]
  #[test]
  fn type_function_user_blocking_nested_pending_expansions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
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
type function func(t)
    return t
end

type test<T> = { x: T, y: T? }
type wrap<T> = { a: func<(string, keyof<test<T>>) -> number>, b: T }
local x: wrap<string>
local y: keyof<typeof(x)>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      r#"{ a: (string, "x" | "y") -> number, b: string }"#,
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      r#""a" | "b""#,
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("y")),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_blocking_nested_pending_expansions_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2283:type_function_user_blocking_nested_pending_expansions_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_blocking_nested_pending_expansions_2

  #[cfg(test)]
  #[test]
  fn type_function_user_blocking_nested_pending_expansions_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type function foo(t)
    return types.unionof(t, types.singleton(nil))
end

local x: foo<{a: foo<string>, b: foo<number>}> = nil
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: string?, b: number? }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut opts
      )
    );
  }
}

mod type_function_user_explicit_export {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1555:type_function_user_explicit_export`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_user_explicit_export

  #[cfg(test)]
  #[test]
  fn type_function_user_explicit_export() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type function concat(a: type, b: type)
    local as = a:value()
    local bs = b:value()
    assert(typeof(as) == "string")
    assert(typeof(bs) == "string")
    return types.singleton(as .. bs)
end
local a: concat<'first', 'second'>
return {}
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(a_result.errors.is_empty(), "{:?}", a_result.errors);
    assert_eq!(
      "\"firstsecond\"",
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("game/A", &String::from("a"))
      )
    );

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Test = require(game.A);
local b: Test.concat<'third', 'fourth'>
    "#,
      ),
      None,
    );
    assert!(b_result.errors.is_empty(), "{:?}", b_result.errors);
    assert_eq!(
      "\"thirdfourth\"",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_function_user_externs_are_extern {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2814:type_function_user_externs_are_extern`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_user_externs_are_extern

  #[cfg(test)]
  #[test]
  fn type_function_user_externs_are_extern() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Bar with
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function foo(t: type)
            assert(t.tag == "extern")
            return t
        end

        type T = foo<Bar>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_implicit_export {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1501:type_function_user_implicit_export`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_user_implicit_export

  #[cfg(test)]
  #[test]
  fn type_function_user_implicit_export() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
type function concat(a: type, b: type)
    local as = a:value()
    local bs = b:value()
    assert(typeof(as) == "string")
    assert(typeof(bs) == "string")
    return types.singleton(as .. bs)
end
export type Concat<T, U> = concat<T, U>
local a: concat<'first', 'second'>
return {}
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(a_result.errors.is_empty(), "{:?}", a_result.errors);
    assert_eq!(
      "\"firstsecond\"",
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("game/A", &String::from("a"))
      )
    );

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Test = require(game.A);
local b: Test.Concat<'third', 'fourth'>
    "#,
      ),
      None,
    );
    assert!(b_result.errors.is_empty(), "{:?}", b_result.errors);
    assert_eq!(
      "\"thirdfourth\"",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_function_user_inner_generics_reducible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2242:type_function_user_inner_generics_reducible`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_inner_generics_reducible

  #[cfg(test)]
  #[test]
  fn type_function_user_inner_generics_reducible() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type function func(t)
    return t
end

type wrap<T> = { a: func<<T>(T) -> number>, b: T }

local x: wrap<string> = nil :: any
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: <T>(T) -> number, b: string }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut opts
      )
    );
  }
}

mod type_function_user_irreducible_pending_expansions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2300:type_function_user_irreducible_pending_expansions`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_irreducible_pending_expansions

  #[cfg(test)]
  #[test]
  fn type_function_user_irreducible_pending_expansions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
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
type function foo(t)
    return types.unionof(t, types.singleton(nil))
end

type table<T> = { a: index<T, "a"> }
type wrap<T> = foo<table<T>>

local x: wrap<{a: number}> = { a = 2 }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: number }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut opts
      )
    );
  }
}

mod type_function_user_issubtypeof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3180:type_function_user_issubtypeof`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_user_issubtypeof

  #[cfg(test)]
  #[test]
  fn type_function_user_issubtypeof() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }
    let _sff = ScopedFastFlag::new(&FFlag::LuauUdtfTypeIsSubtypeOf, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function checksubtype(a, b)
            if not a:issubtypeof(b) then
                error("Not a subtype!")
            end
            return a
        end

        local x: checksubtype<nil, nil>                          -- S
        local y: checksubtype<nil, string?>                      -- S
        local z: checksubtype<"Hello", string>                   -- S
        local w: checksubtype<string | vector | number, number>  -- F
        local a: checksubtype<boolean, number>                   -- F
        local b: checksubtype<false, nil>                        -- F
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    for error in &result.errors {
      assert!(
        matches!(&error.data, TypeErrorData::UserDefinedTypeFunctionError(_)),
        "expected UserDefinedTypeFunctionError, got {:?}",
        error
      );
    }
    assert_eq!(11, result.errors[0].location.begin.line);
    assert_eq!(12, result.errors[1].location.begin.line);
    assert_eq!(13, result.errors[2].location.begin.line);
  }
}

mod type_function_user_local_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1533:type_function_user_local_scope`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_user_local_scope

  #[cfg(test)]
  #[test]
  fn type_function_user_local_scope() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
type function foo()
    return "hi"
end
local function test()
    type function bar()
        return types.singleton(foo())
    end

    return ("" :: any) :: bar<>
end
local a = test()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "\"hi\"",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_function_user_metatable_serialization {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1461:type_function_user_metatable_serialization`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_function_user_metatable_serialization

  #[cfg(test)]
  #[test]
  fn type_function_user_metatable_serialization() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function makemttbl()
            local metaprops = {
                [types.singleton("ma")] = types.boolean
            }
            local mt = types.newtable(metaprops)

            local props = {
                [types.singleton("a")] = types.number
            }
            return types.newtable(props, nil, mt)
        end

        type function id(x)
            return x
        end

        local a: number = {} :: id<makemttbl<>>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got '{ @metatable { ma: boolean }, { a: number } }'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_no_eq_field {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1425:type_function_user_no_eq_field`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_no_eq_field

  #[cfg(test)]
  #[test]
  fn type_function_user_no_eq_field() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function test(x)
            return types.singleton(x.__eq(x, types.number))
        end
        local function ok(tbl: test<number>): never return tbl end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'test' type function errored at runtime: [string \"test\"]:3: attempt to call a nil value",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_no_metatable_writes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1408:type_function_user_no_metatable_writes`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_no_metatable_writes

  #[cfg(test)]
  #[test]
  fn type_function_user_no_metatable_writes() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function test(x)
            local a = x.__index
            a.is = function() return false end
            return types.singleton(x.is("number"))
        end
        local function ok(tbl: test<number>): never return tbl end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'test' type function errored at runtime: [string \"test\"]:4: attempt to index nil with 'is'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_no_type_methods_on_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1377:type_function_user_no_type_methods_on_types`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_no_type_methods_on_types

  #[cfg(test)]
  #[test]
  fn type_function_user_no_type_methods_on_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function test(x)
            return if (types :: any).is(x, "number") then types.string else types.boolean
        end
        local function ok(tbl: test<number>): never return tbl end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'test' type function errored at runtime: [string \"test\"]:3: attempt to call a nil value",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_no_types_functions_on_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1393:type_function_user_no_types_functions_on_type`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_no_types_functions_on_type

  #[cfg(test)]
  #[test]
  fn type_function_user_no_types_functions_on_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function test(x)
            return x.singleton("a")
        end
        local function ok(tbl: test<number>): never return tbl end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'test' type function errored at runtime: [string \"test\"]:3: attempt to call a nil value",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_nonstrict_mode {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1489:type_function_user_nonstrict_mode`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_nonstrict_mode

  #[cfg(test)]
  #[test]
  fn type_function_user_nonstrict_mode() {
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
--!nonstrict
type function foo() return types.string end
local a: foo<> = "a"
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_oss_1887_basic_match {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2783:type_function_user_oss_1887_basic_match`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record UnappliedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_user_oss_1887_basic_match

  #[cfg(test)]
  #[test]
  fn type_function_user_oss_1887_basic_match() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function foo()
            return types.string
        end
        local f: foo = "123"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      &result.errors[0].data,
      TypeErrorData::UnappliedTypeFunction(_)
    ));
  }
}

mod type_function_user_oss_1887_basic_mismatch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2764:type_function_user_oss_1887_basic_mismatch`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnappliedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_oss_1887_basic_mismatch

  #[cfg(test)]
  #[test]
  fn type_function_user_oss_1887_basic_mismatch() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function foo()
            return types.number
        end
        local f: foo = "123"
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      &result.errors[0].data,
      TypeErrorData::UnappliedTypeFunction(_)
    ));
    match &result.errors[1].data {
      TypeErrorData::TypeMismatch(err) => {
        assert_eq!("string", to_string_type_id(err.given_type));
        assert_eq!("number", to_string_type_id(err.wanted_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_oss_1887_udtf_table_mismatch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2743:type_function_user_oss_1887_udtf_table_mismatch`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnappliedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_oss_1887_udtf_table_mismatch

  #[cfg(test)]
  #[test]
  fn type_function_user_oss_1887_udtf_table_mismatch() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function create_table_with_key()
            local tbl = types.newtable()
            tbl:setproperty(types.singleton "key", types.optional(types.number))
            return tbl
        end
        local my_tbl: create_table_with_key = {key = "123"}
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      &result.errors[0].data,
      TypeErrorData::UnappliedTypeFunction(_)
    ));
    match &result.errors[1].data {
      TypeErrorData::TypeMismatch(err) => {
        assert_eq!("string", to_string_type_id(err.given_type));
        assert_eq!("number?", to_string_type_id(err.wanted_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_oss_1887_udtf_with_optional_missing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2709:type_function_user_oss_1887_udtf_with_optional_missing`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record UnappliedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_user_oss_1887_udtf_with_optional_missing

  #[cfg(test)]
  #[test]
  fn type_function_user_oss_1887_udtf_with_optional_missing() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function create_table_with_key()
            local tbl = types.newtable()
            tbl:setproperty(types.singleton "key", types.unionof(types.string, types.singleton(nil)))
            return tbl
        end
        local a: create_table_with_key = {}
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      &result.errors[0].data,
      TypeErrorData::UnappliedTypeFunction(_)
    ));
  }
}

mod type_function_user_oss_1887_udtf_with_optional_present {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2726:type_function_user_oss_1887_udtf_with_optional_present`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record UnappliedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_user_oss_1887_udtf_with_optional_present

  #[cfg(test)]
  #[test]
  fn type_function_user_oss_1887_udtf_with_optional_present() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function create_table_with_key()
            local tbl = types.newtable()
            tbl:setproperty(types.singleton "key", types.unionof(types.string, types.singleton(nil)))
            return tbl
        end
        local a: create_table_with_key = { key = "123" }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      &result.errors[0].data,
      TypeErrorData::UnappliedTypeFunction(_)
    ));
  }
}

mod type_function_user_oss_2164_table_subtyping_bug {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2879:type_function_user_oss_2164_table_subtyping_bug`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_function_user_oss_2164_table_subtyping_bug

  #[cfg(test)]
  #[test]
  fn type_function_user_oss_2164_table_subtyping_bug() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _fix = ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type function tblpartial(tbl: type)
            assert(tbl:is("table"), "tblpartial can only be applied to tables")
            local new = types.newtable()

            for k, v in tbl:properties() do
                local read = assert(v.read, "properties cannot be write-only")
                new:setreadproperty(k, types.optional(read))
            end

            return new
        end

        local function tblmerge<T>(base: T, override: tblpartial<T>): T error("unimplemented") end
        tblmerge({ a = 1 }, {}) -- Type '{  }' could not be converted into '{ read a: number? }'
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_outer_generics_irreducible {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2223:type_function_user_outer_generics_irreducible`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_outer_generics_irreducible

  #[cfg(test)]
  #[test]
  fn type_function_user_outer_generics_irreducible() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type function func(t)
    return t
end

type wrap<T> = { a: func<T?> }

local x: wrap<string> = nil :: any
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: string? }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut opts
      )
    );
  }
}

mod type_function_user_print_to_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1586:type_function_user_print_to_error`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_print_to_error

  #[cfg(test)]
  #[test]
  fn type_function_user_print_to_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver_v2 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function t0(a)
            print("Where does this go")
            print(a.tag)
            return types.any
        end
        local a: t0<string>
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Where does this go",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!("string", to_string_type_error(&result.errors[1]));
  }
}

mod type_function_user_print_to_error_plus_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1604:type_function_user_print_to_error_plus_error`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_print_to_error_plus_error

  #[cfg(test)]
  #[test]
  fn type_function_user_print_to_error_plus_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function t0(a)
            print("Where does this go")
            print(a.tag)
            error("test")
        end
        local a: t0<string>
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Where does this go",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!("string", to_string_type_error(&result.errors[1]));
    assert_eq!(
      "'t0' type function errored at runtime: [string \"t0\"]:5: test",
      to_string_type_error(&result.errors[2])
    );
  }
}

mod type_function_user_print_to_error_plus_no_result {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1624:type_function_user_print_to_error_plus_no_result`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_print_to_error_plus_no_result

  #[cfg(test)]
  #[test]
  fn type_function_user_print_to_error_plus_no_result() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver_v2 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function t0(a)
            print("Where does this go")
            print(a.tag)
        end
        local a: t0<string>
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Where does this go",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!("string", to_string_type_error(&result.errors[1]));
    assert_eq!(
      "'t0' type function: returned a non-type value",
      to_string_type_error(&result.errors[2])
    );
  }
}

mod type_function_user_tag_field {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1440:type_function_user_tag_field`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_tag_field

  #[cfg(test)]
  #[test]
  fn type_function_user_tag_field() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function test(x)
            return types.singleton(x.tag)
        end

        local function ok1(tbl: test<number>): never return tbl end
        local function ok2(tbl: test<string>): never return tbl end
        local function ok3(tbl: test<{}>): never return tbl end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be unreachable, but got '\"number\"'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be unreachable, but got '\"string\"'",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Expected this to be unreachable, but got '\"table\"'",
      to_string_type_error(&result.errors[2])
    );
  }
}

mod type_function_user_thread_and_buffer_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:228:type_function_user_thread_and_buffer_types`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_thread_and_buffer_types

  #[cfg(test)]
  #[test]
  fn type_function_user_thread_and_buffer_types() {
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
        type function work_with_thread(x)
            if x:is("thread") then
                return types.thread
            end
            return types.string
        end
        type X = thread
        local function ok(idx: work_with_thread<X>): thread return idx end
    "#,
      ),
      None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function work_with_buffer(x)
            if x:is("buffer") then
                return types.buffer
            end
            return types.string
        end
        type X = buffer
        local function ok(idx: work_with_buffer<X>): buffer return idx end
    "#,
      ),
      None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_type_alias_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2336:type_function_user_type_alias_call`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_call

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_call() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type Test<T> = T?

type function foo(t)
    return Test(t)
end

local x: foo<{a: number}> = { a = 2 }
local y: foo<{b: number}> = { b = 2 }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: number }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ b: number }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("y")),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_type_alias_call_indirect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2357:type_function_user_type_alias_call_indirect`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_call_indirect

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_call_indirect() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type Test<T> = T?

type function foo(t)
    return Test(t)
end

type function bar(t)
    return foo(t)
end

local x: bar<{a: number}> = { a = 2 }
local y: bar<{b: number}> = { b = 2 }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: number }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ b: number }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("y")),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_type_alias_call_indirect_levels {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2382:type_function_user_type_alias_call_indirect_levels`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_call_indirect_levels

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_call_indirect_levels() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_ast::records::position::Position;
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
type Test<T> = T?

type function foo(t)
    return Test(t)
end

do
    type function bar(t)
        return foo(t)
    end

    local x: bar<{a: number}> = { a = 2 }
    local y: bar<{b: number}> = { b = 2 }

    print(x, y)
end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: number }?",
      to_string_type_id_to_string_options(
        fixture
          .base
          .require_type_at_position_position(Position::new(15, 10)),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ b: number }?",
      to_string_type_id_to_string_options(
        fixture
          .base
          .require_type_at_position_position(Position::new(15, 13)),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_type_alias_call_with_reduction {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2459:type_function_user_type_alias_call_with_reduction`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_call_with_reduction

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_call_with_reduction() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
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
type Test<T> = rawget<T, "a">

type function foo(t)
    return Test(t)
end

local x: foo<{ a: number }> = 2
local y: foo<{ a: string }> = "x"
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      "number",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      "string",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("y")),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_type_alias_can_call_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2577:type_function_user_type_alias_can_call_packs`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_can_call_packs

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_can_call_packs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type Test<T, U...> = (U...) -> T

type function foo(t)
    return Test(types.number, types.string, t)
end

local x: foo<boolean>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "(string, boolean) -> number",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut opts
      )
    );
  }
}

mod type_function_user_type_alias_implicit_export {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2481:type_function_user_type_alias_implicit_export`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_type_alias_implicit_export

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_implicit_export() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
type Test<T> = rawget<T, "a">

export type function foo(t)
    return Test(t)
end
local x: foo<{ a: number }> = 2
return {}
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(a_result.errors.is_empty(), "{:?}", a_result.errors);
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("game/A", &String::from("x"))
      )
    );

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Test = require(game.A);
local y: Test.foo<{ a: string }> = "x"
    "#,
      ),
      None,
    );
    assert!(b_result.errors.is_empty(), "{:?}", b_result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_function_user_type_alias_implicit_export_indirect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2510:type_function_user_type_alias_implicit_export_indirect`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_type_alias_implicit_export_indirect

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_implicit_export_indirect() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
type Test<T> = rawget<T, "a">

type function foo(t)
    return Test(t)
end

export type function bar(t)
    return foo(t)
end

local x: bar<{ a: number }> = 2
return {}
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(a_result.errors.is_empty(), "{:?}", a_result.errors);
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("game/A", &String::from("x"))
      )
    );

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Test = require(game.A);
local y: Test.bar<{ a: string }> = "x"
    "#,
      ),
      None,
    );
    assert!(b_result.errors.is_empty(), "{:?}", b_result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_function_user_type_alias_not_enough_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2559:type_function_user_type_alias_not_enough_arguments`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_type_alias_not_enough_arguments

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_not_enough_arguments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Test<A, B> = (a: A, b: B) -> A

type function get()
    return Test(types.number)
end

local function ok(idx: get<>): number return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'get' type function errored at runtime: [string \"get\"]:5: not enough arguments to call",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_type_alias_not_too_many_globals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2544:type_function_user_type_alias_not_too_many_globals`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_function_user_type_alias_not_too_many_globals

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_not_too_many_globals() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    return number
end
local function ok(idx: get<>): number return idx end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'number'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_type_alias_reduction_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2596:type_function_user_type_alias_reduction_errors`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_type_alias_reduction_errors

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_reduction_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Test<T, U> = setmetatable<T, U>

type function get()
    return Test(types.number, types.string)
end

local function ok(idx: get<>): number return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'get' type function errored at runtime: [string \"get\"]:5: failed to reduce type function with: Type function instance setmetatable<number, string> is uninhabited",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_type_alias_unordered {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2432:type_function_user_type_alias_unordered`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_unordered

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_unordered() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type function foobar(ty)
    if ty:is("number") then
        return ty
    end
    return TableOf(ty)
end

type TableOf<T> = { prop: T }

type ShouldBeNumber = foobar<number>
type ShouldBeTableOfString = foobar<string>

local x: ShouldBeNumber = 2
local y: ShouldBeTableOfString = { prop = "a" }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      "number",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ prop: string }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("y")),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_type_alias_unreferenced_do_not_block {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2618:type_function_user_type_alias_unreferenced_do_not_block`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_unreferenced_do_not_block

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_unreferenced_do_not_block() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type function foo(t)
    return types.unionof(types.number, t)
end

type Test = foo<string>

local x: foo<boolean>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "boolean | number",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut opts
      )
    );
  }
}

mod type_function_user_type_alias_values {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2411:type_function_user_type_alias_values`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_function_user_type_alias_values

  #[cfg(test)]
  #[test]
  fn type_function_user_type_alias_values() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
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
type Test = { a: number }

type function foo(t)
    return types.unionof(Test, t)
end

local x: foo<nil> = { a = 2 }
local y: foo<string> = "a"
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::new(true);
    assert_eq!(
      "{ a: number }?",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("x")),
        &mut x_opts
      )
    );

    let mut y_opts = ToStringOptions::new(true);
    assert_eq!(
      "string | { a: number }",
      to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("y")),
        &mut y_opts
      )
    );
  }
}

mod type_function_user_type_functions_can_mutate_cloned_type_aliases {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2860:type_function_user_type_functions_can_mutate_cloned_type_aliases`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_type_functions_can_mutate_cloned_type_aliases

  #[cfg(test)]
  #[test]
  fn type_function_user_type_functions_can_mutate_cloned_type_aliases() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _frozen = ScopedFastFlag::new(&FFlag::LuauTypeFunctionSupportsFrozen, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type myType = { woof: string }

        type function create_table_with_key()
            local tbl = types.copy(myType)
            tbl:setproperty(types.singleton "key", types.optional(types.number))
            return tbl
        end
        local my_tbl: create_table_with_key<> = { key = 123, woof = "woof" }
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_type_functions_cannot_try_to_mutate_type_aliases {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2835:type_function_user_type_functions_cannot_try_to_mutate_type_aliases`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_function_user_type_functions_cannot_try_to_mutate_type_aliases

  #[cfg(test)]
  #[test]
  fn type_function_user_type_functions_cannot_try_to_mutate_type_aliases() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _frozen = ScopedFastFlag::new(&FFlag::LuauTypeFunctionSupportsFrozen, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type myType = {}

        type function create_table_with_key()
            myType:setproperty(types.singleton "key", types.optional(types.number))
            return myType
        end
        local my_tbl: create_table_with_key<> = {key = "123"}
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'create_table_with_key' type function errored at runtime: [string \"create_table_with_key\"]:5: type.setproperty: cannot be called to mutate a frozen type, use `types.copy` to make a copy",
      to_string_type_error(&result.errors[0])
    );
    match &result.errors[1].data {
      TypeErrorData::TypeMismatch(err) => {
        assert_eq!("{ key: string }", to_string_type_id(err.given_type));
        assert_eq!(
          "create_table_with_key<>",
          to_string_type_id(err.wanted_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_type_functions_many_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2904:type_function_user_type_functions_many_arguments`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_type_functions_many_arguments

  #[cfg(test)]
  #[test]
  fn type_function_user_type_functions_many_arguments() {
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
type function many(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak)
    return a0
end

local x: many<number, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any> = 1
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_typecheck_failure {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2210:type_function_user_typecheck_failure`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_user_typecheck_failure

  #[cfg(test)]
  #[test]
  fn type_function_user_typecheck_failure() {
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
type function foo()
    return types.singleton({1})
end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_function_user_typecheck_success {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2197:type_function_user_typecheck_success`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_function_user_typecheck_success

  #[cfg(test)]
  #[test]
  fn type_function_user_typecheck_success() {
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
type function foo(x: type)
    return types.singleton(x.tag)
end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_typeof_into_type_function_should_not_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2798:type_function_user_typeof_into_type_function_should_not_crash`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_typeof_into_type_function_should_not_crash

  #[cfg(test)]
  #[test]
  fn type_function_user_typeof_into_type_function_should_not_crash() {
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
        type function identity(t: type)
            return t
        end

        type func<parameters...> = typeof(function(...: parameters...) end)
        local whomp: <T>(arg1: T) -> identity<T>
        whomp(function(...) end :: func<any>)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_typeof_is_not_a_valid_type_function_name {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2321:type_function_user_typeof_is_not_a_valid_type_function_name`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::identifier (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_typeof_is_not_a_valid_type_function_name

  #[cfg(test)]
  #[test]
  fn type_function_user_typeof_is_not_a_valid_type_function_name() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function typeof(t)
	        return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "typeof cannot be used as an identifier for a type function or alias",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_typeof_type_userdata_returns_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2140:type_function_user_typeof_type_userdata_returns_type`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_function_user_typeof_type_userdata_returns_type

  #[cfg(test)]
  #[test]
  fn type_function_user_typeof_type_userdata_returns_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver_v2 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function test(t)
    print(typeof(t))
    return t
end

local _:test<number>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!("type", to_string_type_error(&result.errors[0]));
  }
}

mod type_function_user_udtf_any_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:141:type_function_user_udtf_any_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_any_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_any_methods_work() {
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
        type function getany()
            local ty = types.any
            if ty:is("any") then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getany<>): any return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_any_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:126:type_function_user_udtf_any_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_any_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_any_serialization_works() {
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
        type function serialize_any(arg)
            return arg
        end
        type type_being_serialized = any
        local function ok(idx: serialize_any<type_being_serialized>): any return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_areequal_stack_overflow_on_deep_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3085:type_function_user_udtf_areequal_stack_overflow_on_deep_types`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_areequal_stack_overflow_on_deep_types

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_areequal_stack_overflow_on_deep_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function deep_eq()
            local depth = 50000
            local function build()
                local t = types.newtable()
                for i = 1, depth do
                    local outer = types.newtable()
                    outer:setproperty(types.singleton("x"), t)
                    t = outer
                end
                return t
            end
            local a = build()
            local b = build()
            if a == b then
                return types.boolean
            end
            return types.string
        end

        local x: deep_eq<> = true
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'deep_eq' type function errored at runtime: Internal recursion counter limit exceeded in areEqual",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_boolean_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:175:type_function_user_udtf_boolean_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_boolean_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_boolean_methods_work() {
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
        type function getboolean()
            local ty = types.boolean
            if ty:is("boolean") then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getboolean<>): boolean return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_boolean_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:160:type_function_user_udtf_boolean_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_boolean_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_boolean_serialization_works() {
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
        type function serialize_bool(arg)
            return arg
        end
        type type_being_serialized = boolean
        local function ok(idx: serialize_bool<type_being_serialized>): boolean return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_boolsingleton_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:304:type_function_user_udtf_boolsingleton_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_boolsingleton_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_boolsingleton_methods_work() {
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
        type function getboolsingleton()
            local ty = types.singleton(true)
            if ty:is("singleton") and ty:value() then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getboolsingleton<>): true return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_boolsingleton_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:289:type_function_user_udtf_boolsingleton_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_boolsingleton_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_boolsingleton_serialization_works() {
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
        type function serialize_boolsingleton(arg)
            return arg
        end
        type type_being_serialized = true
        local function ok(idx: serialize_boolsingleton<type_being_serialized>): true return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_calling_each_other {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1103:type_function_user_udtf_calling_each_other`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_calling_each_other

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_calling_each_other() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function foo()
            return "hi"
        end
        type function bar()
            return types.singleton(foo())
        end
        local function ok(idx: bar<>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"hi\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_calling_each_other_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1123:type_function_user_udtf_calling_each_other_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_calling_each_other_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_calling_each_other_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function first(arg)
            return arg
        end
        type function second(arg)
            return types.singleton(first(arg))
        end
        type function third()
            return second("hi")
        end
        local function ok(idx: third<>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"hi\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_calling_each_other_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1146:type_function_user_udtf_calling_each_other_3`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_calling_each_other_3

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_calling_each_other_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        -- this function should not see 'fourth' function when invoked from 'third' that sees it
        type function first(arg)
            return fourth(arg)
        end
        type function second(arg)
            return types.singleton(first(arg))
        end

        do
            type function fourth(arg)
                return arg
            end
            type function third()
                return second("hi")
            end
            local function ok(idx: third<>): nil return idx end
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'fourth'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "'third' type function errored at runtime: [string \"first\"]:4: attempt to call a nil value",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_user_udtf_calling_each_other_unordered {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1175:type_function_user_udtf_calling_each_other_unordered`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_calling_each_other_unordered

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_calling_each_other_unordered() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function bar()
            return types.singleton(foo())
        end
        type function foo()
            return "hi"
        end
        local function ok(idx: bar<>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("\"hi\"", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_calling_illegal_global {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1266:type_function_user_udtf_calling_illegal_global`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_calling_illegal_global

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_calling_illegal_global() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function illegal(arg)
            gcinfo() -- this should error

            return arg -- this should not be reached
        end

        local function ok(idx: illegal<number>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'gcinfo'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "'illegal' type function errored at runtime: [string \"illegal\"]:3: this function is not supported in type functions",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_user_udtf_check_mutability {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:887:type_function_user_udtf_check_mutability`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_check_mutability

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_check_mutability() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function checkmut()
            local indexer = {
                index = types.number,
                readresult = types.boolean,
                writeresult = types.boolean,
            }
            local ty = types.newtable(nil, indexer, nil) -- {[number]: boolean}
            ty:setproperty(types.singleton("string"), types.number) -- {string: number, [number]: boolean}
            local metatbl = types.newtable(nil, nil, ty) -- { {  }, @metatable { [number]: boolean, string: number } }
            -- mutate the table
            ty:setproperty(types.singleton("string"), nil) -- {[number]: boolean}
            if metatbl:is("table") and metatbl:metatable() then
                return metatbl -- { @metatable { [number]: boolean }, { } }
            end
            -- this should never be returned
            return types.number
        end
        local function ok(idx: checkmut<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ @metatable {boolean}, {  } }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_class_methods_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:840:type_function_user_udtf_class_methods_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_class_methods_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_class_methods_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function getclass(arg)
            local props = arg:properties()
            local indexer = arg:indexer()
            local metatable = arg:metatable()
            return types.newtable(props, indexer, metatable)
        end
        -- forcing an error here to check the exact type of the metatable
        local function ok(idx: getclass<BaseClass>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ BaseField: number, read BaseMethod: (BaseClass, number) -> (), read Touched: Connection }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_class_parent_ops {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2177:type_function_user_udtf_class_parent_ops`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_class_parent_ops

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_class_parent_ops() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function readparentof(arg)
            return arg:readparent()
        end

        type function writeparentof(arg)
            return arg:writeparent()
        end

        local function ok1(idx: readparentof<ChildClass>): BaseClass return idx end
        local function ok2(idx: writeparentof<ChildClass>): BaseClass return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_class_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:812:type_function_user_udtf_class_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_class_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_class_serialization_works() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function serialize_class(arg)
            return arg
        end
        local function ok(idx: serialize_class<BaseClass>): BaseClass return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_class_serialization_works_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:826:type_function_user_udtf_class_serialization_works_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_class_serialization_works_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_class_serialization_works_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function serialize_class(arg)
            return arg
        end
        local function ok(idx: serialize_class<typeof(confusingBaseClassInstance)>): typeof(confusingBaseClassInstance) return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_cloner_missing_integer_crashes_copy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3139:type_function_user_udtf_cloner_missing_integer_crashes_copy`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_cloner_missing_integer_crashes_copy

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_cloner_missing_integer_crashes_copy() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _integer_type = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function copy_int(arg)
            local c = types.copy(arg)
            return c
        end

        local function ok(idx: copy_int<integer>): integer return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_complex_cyclic_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:990:type_function_user_udtf_complex_cyclic_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_complex_cyclic_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_complex_cyclic_serialization_works() {
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
        type function serialize_cycle2(arg)
            return arg
        end
        type Employee = {
            name: string,
            department: Department?
        }
        type Department = {
            name: string,
            manager: Employee?,
            employees: { Employee },
            company: Company?
        }
        type Company = {
            name: string,
            departments: { Department }
        }
        local function ok(idx: serialize_cycle2<Company>): Company return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_copy_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:918:type_function_user_udtf_copy_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_copy_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_copy_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function getcopy()
            local indexer = {
                index = types.number,
                readresult = types.boolean,
                writeresult = types.boolean,
            }
            local ty = types.newtable(nil, indexer, nil) -- {[number]: boolean}
            ty:setproperty(types.singleton("string"), types.number) -- {string: number, [number]: boolean}
            local metaty = types.newtable(nil, nil, ty) -- { {  }, @metatable { [number]: boolean, string: number } }
            local copy = types.copy(metaty)
            -- mutate the table
            ty:setproperty(types.singleton("string"), nil) -- {[number]: boolean}
            if copy:is("table") and copy:metatable() then
                return copy -- { {  }, @metatable { [number]: boolean, string: number } }
            end
            -- this should never be returned
            return types.number
        end
        local function ok(idx: getcopy<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ @metatable { [number]: boolean, string: number }, {  } }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_createtable_bad_metatable {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:970:type_function_user_udtf_createtable_bad_metatable`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_createtable_bad_metatable

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_createtable_bad_metatable() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function badmetatable()
            return types.newtable(nil, nil, types.number)
        end
        local function bad(arg: badmetatable<>) end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::UserDefinedTypeFunctionError(e) => {
        assert_eq!(
          "'badmetatable' type function errored at runtime: [string \"badmetatable\"]:3: types.newtable: expected to be given a table type as a metatable, but got number instead",
          e.message()
        );
      }
      other => panic!("expected UserDefinedTypeFunctionError, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_deep_copy_iteration_limit_null_deref {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3055:type_function_user_udtf_deep_copy_iteration_limit_null_deref`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_deep_copy_iteration_limit_null_deref

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_deep_copy_iteration_limit_null_deref() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::{DFInt, FFlag};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);
    let _serde_limit = ScopedFastInt::new(&DFInt::LuauTypeFunctionSerdeIterationLimit, 10);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function copy_complex(arg)
            local t = types.newtable()
            t:setproperty(types.singleton("a"), types.number)
            t:setproperty(types.singleton("b"), types.string)
            t:setproperty(types.singleton("c"), types.boolean)
            t:setproperty(types.singleton("d"), types.buffer)
            t:setproperty(types.singleton("e"), types.thread)
            t:setproperty(types.singleton("f"), types.newtable())
            t:setproperty(types.singleton("g"), types.newfunction())
            local c = types.copy(t)
            return c
        end

        local function ok(idx: copy_complex<number>): number return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'copy_complex' type function errored at runtime: [string \"copy_complex\"]:11: types.copy: complexity limit reached during type copy",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_double_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2664:type_function_user_udtf_double_definition`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_double_definition

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_double_definition() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type function t0<A>()
end
type function t0<A>()
end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Redefinition of type 't0', previously defined at line 2",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_env_alias_serialize_null_deref {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3027:type_function_user_udtf_env_alias_serialize_null_deref`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_env_alias_serialize_null_deref

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_env_alias_serialize_null_deref() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::{DFInt, FFlag};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);
    let _serde_limit = ScopedFastInt::new(&DFInt::LuauTypeFunctionSerdeIterationLimit, 10);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Alias = {
            a: number,
            b: string,
            c: boolean,
            d: nil,
            e: buffer,
            f: thread,
            g: (number, string) -> (boolean, nil),
        }

        type function use_alias()
            return Alias
        end

        local function ok(idx: use_alias<>): Alias return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'use_alias' type function: returned a non-type value",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_flatten_on_intersectionof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:498:type_function_user_udtf_flatten_on_intersectionof`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_flatten_on_intersectionof

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_flatten_on_intersectionof() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        type function foobar()
            local tys = { types.string, types.number, types.unknown, types.boolean }
            local result = types.unknown
            for _, ty in tys do
                result = types.intersectionof(result, ty)
            end
            return result
        end

        local f: foobar<>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "boolean & number & string",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_function_user_udtf_flatten_on_intersectionof_empty {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:519:type_function_user_udtf_flatten_on_intersectionof_empty`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_flatten_on_intersectionof_empty

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_flatten_on_intersectionof_empty() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        type function foobar()
            return types.intersectionof()
        end

        local f: foobar<>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_function_user_udtf_flatten_on_intersectionof_two_things {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:535:type_function_user_udtf_flatten_on_intersectionof_two_things`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_flatten_on_intersectionof_two_things

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_flatten_on_intersectionof_two_things() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function foobar()
            return types.intersectionof(types.unknown, types.string)
        end
        -- forcing an error here to check the exact type of the union
        local function ok(idx: foobar<>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("string", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_flatten_on_unionof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:441:type_function_user_udtf_flatten_on_unionof`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_flatten_on_unionof

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_flatten_on_unionof() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function foobar()
            local tys = { types.string, types.number, types.never, types.boolean, types.singleton(nil) }
            local result = types.never
            for _, ty in tys do
                result = types.unionof(result, ty)
            end
            return result
        end
        -- forcing an error here to check the exact type of the union
        local function ok(idx: foobar<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "(boolean | number | string)?",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_flatten_on_unionof_empty {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:464:type_function_user_udtf_flatten_on_unionof_empty`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_flatten_on_unionof_empty

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_flatten_on_unionof_empty() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        type function foobar()
            return types.unionof()
        end

        local f: foobar<>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "never",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_function_user_udtf_flatten_on_unionof_two_things {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:480:type_function_user_udtf_flatten_on_unionof_two_things`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_flatten_on_unionof_two_things

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_flatten_on_unionof_two_things() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function foobar()
            return types.unionof(types.string, types.never)
        end
        -- forcing an error here to check the exact type of the union
        local function ok(idx: foobar<>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("string", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_follow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1336:type_function_user_udtf_follow`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_follow

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_follow() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type t0 = any
        type function t0()
            return types.any
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Redefinition of type 't0', previously defined at line 2",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_function_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:779:type_function_user_udtf_function_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_function_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_function_methods_work() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
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
        type function getfunction()
            local ty = types.newfunction(nil, nil) -- () -> ()
            ty:setparameters({types.string, types.number}, nil) -- (string, number) -> ()
            ty:setreturns(nil, types.boolean) -- (string, number) -> (...boolean)
            if ty:is("function") then
                -- creating a copy of `ty` parameters
                local arr: {type} = {}
                local args = ty:parameters().head
                if args then
                    for index, val in args do
                        table.insert(arr, val)
                    end
                end
                return types.newfunction({head = arr}, ty:returns()) -- (string, number) -> (...boolean)
            end
            -- this should never be returned
            return types.number
        end
        local function ok(idx: getfunction<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "(string, number) -> (...boolean)",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_function_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:764:type_function_user_udtf_function_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_function_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_function_serialization_works() {
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
        type function serialize_func(arg)
            return arg
        end
        type type_being_serialized = (boolean, number, nil) -> (...string)
        local function ok(idx: serialize_func<type_being_serialized>): (boolean, number, nil) -> (...string) return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_function_type_cant_call_get_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1083:type_function_user_udtf_function_type_cant_call_get_props`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_function_type_cant_call_get_props

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_function_type_cant_call_get_props() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function hello(arg)
            local arr = arg:properties()
        end
        local function ok(idx: hello<() -> ()>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::UserDefinedTypeFunctionError(e) => {
        assert_eq!(
          "'hello' type function errored at runtime: [string \"hello\"]:3: type.properties: expected self to be either a table or class, but got function instead",
          e.message()
        );
      }
      other => panic!("expected UserDefinedTypeFunctionError, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_fuzz_environment_scope_crash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2679:type_function_user_udtf_fuzz_environment_scope_crash`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_fuzz_environment_scope_crash

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_fuzz_environment_scope_crash() {
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
local _, running = ...
type function t255() end
if _ then
    type function t1() end
    type function t6(l0,...) end
    type function t255<A...>() end
    export type function t0<A>() end
else
    type function t1(...) end
    type function t66<A...>(...) end
    type function t255() end
    if running then
        export type function t255() end
        type function t0(l0) end
    end
end
type function t0(l0,...) end
export type function t66(...)
    export type function t255() end
end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1748:type_function_user_udtf_generic_api_1`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_1

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_1() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    local generics = arg:generics()
    local T = generics[1]
    return types.newfunction({ head = {T} }, { head = {T} }, {T})
end

type test = <T, U>(T, { x: <T>(y: T) -> (), y: U }, U) -> ()

local function ok(idx: pass<test>): <T>(T) -> (T) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1767:type_function_user_udtf_generic_api_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    local generics = arg:generics()
    local T = generics[1]
    local f = types.newfunction()
    f:setparameters({T, T});
    f:setreturns({T});
    f:setgenerics({T});
    return f
end

type test = <T, U>(T, { x: <T>(y: T) -> (), y: U }, U) -> ()

local function ok(idx: pass<test>): <T>(T, T) -> (T) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1790:type_function_user_udtf_generic_api_3`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_3

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_3() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass()
    local T = types.generic("T")
    assert(T.tag == "generic")
    assert(T:name() == "T")
    assert(T:ispack() == false)

    local Us, Vs = types.generic("U", true), types.generic("V", true)
    assert(Us.tag == "generic")
    assert(Us:name() == "U")
    assert(Us:ispack() == true)

    local f = types.newfunction()
    f:setparameters({T}, Us);
    f:setreturns({T}, Vs);
    f:setgenerics({T, Us, Vs});
    return f
end

local function ok(idx: pass<>): <T, U..., V...>(T, U...) -> (T, V...) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1820:type_function_user_udtf_generic_api_4`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_4

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_4() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass()
    local T, U = types.generic("T"), types.generic("U")

    -- <T>(T) -> ()
    local func = types.newfunction({ head = {T} }, {}, {T});

    -- { x: <T>(T) -> (), y: U }
    local tbl = types.newtable({ [types.singleton("x")] = func, [types.singleton("y")] = U })

    -- <T, U>(T, { x: <T>(T) -> (), y: U }, U) -> ()
    return types.newfunction({ head = {T, tbl, U } }, {}, {T, U})
end

type test = <T, U>(T, { x: <T>(y: T) -> (), y: U }, U) -> ()

local function ok(idx: pass<>): test return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_5 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1846:type_function_user_udtf_generic_api_5`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_5

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_5() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass()
    local T = types.generic("T")
    return types.newfunction({ head = {T} }, {}, {types.copy(T)})
end

local function ok(idx: pass<>): <T>(T) -> () return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_6 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1862:type_function_user_udtf_generic_api_6`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_6

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_6() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    local generics = arg:generics()
    local T, U = generics[1], generics[2]
    local f = types.newfunction()
    f:setparameters({T});
    f:setreturns({U});
    f:setgenerics({T, U});
    return f
end

local function m(a, b)
    return {x = a, y = b}
end

type test = typeof(m)

local function ok(idx: pass<test>): <T, U>(T) -> (U) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_7 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1889:type_function_user_udtf_generic_api_7`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_7

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_7() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    local p, r = arg:parameters(), arg:returns()
    local f = types.newfunction()
    f:setparameters(p.head, p.tail);
    f:setreturns(r.head, r.tail);
    f:setgenerics(arg:generics());
    return f
end

type test = <T, U...>(T, U...) -> (T, U...)

local function ok(idx: pass<test>): <T, U...>(T, U...) -> (T, U...) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_8 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1911:type_function_user_udtf_generic_api_8`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_8

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_8() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    local p, r = arg:parameters(), arg:returns()
    local f = types.newfunction()
    f:setparameters(p.head, p.tail);
    f:setreturns(r.head, r.tail);
    f:setgenerics(arg:generics());
    return f
end

type test = <U...>(U...) -> (U...)

local function ok(idx: pass<test>): <T>(T, T) -> (T, T) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_api_error_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1953:type_function_user_udtf_generic_api_error_1`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_1

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, Us = types.generic("T"), types.generic("U", true)
    return types.newfunction({}, {}, {Us, T})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'get' type function errored at runtime: [string \"get\"]:4: types.newfunction: generic type cannot follow a generic pack",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_api_error_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1972:type_function_user_udtf_generic_api_error_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, Us = types.generic("T"), types.generic("U", true)
    return types.newfunction({ head = {T} }, {}, {})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Generic type 'T' is not in a scope of the active generic function",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_api_error_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1988:type_function_user_udtf_generic_api_error_3`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_3

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, U = types.generic("T"), types.generic("U")

    -- <U>(U) -> ()
    local func = types.newfunction({ head = {U} }, {}, {U});

    -- broken: <T>(T, <U>(U) -> (), U) -> ()
    return types.newfunction({ head = {T, func, U } }, {}, {T})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Generic type 'U' is not in a scope of the active generic function",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_api_error_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2009:type_function_user_udtf_generic_api_error_4`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_4

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_4() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, Us = types.generic("T"), types.generic("U", true)
    return types.newfunction({ head = {T} }, { tail = Us }, {T, T})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Duplicate type parameter 'T'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_api_error_5 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2025:type_function_user_udtf_generic_api_error_5`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_5

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_5() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, Ts = types.generic("T"), types.generic("T", true)
    return types.newfunction({ head = {T} }, { tail = Ts }, {T, Ts})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Duplicate type parameter 'T'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_api_error_6 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2041:type_function_user_udtf_generic_api_error_6`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_6

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_6() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, Us = types.generic("T"), types.generic("U", true)
    return types.newfunction({ head = {Us} }, {}, {T, Us})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Generic type pack 'U...' cannot be placed in a type position",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_api_error_7 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2057:type_function_user_udtf_generic_api_error_7`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_7

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_api_error_7() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function get()
    local T, Us = types.generic("T"), types.generic("U", true)
    return types.newfunction({ tail = Us }, {}, {T})
end
local function ok(idx: get<>): false return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Generic type pack 'U...' is not in a scope of the active generic function",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_generic_cloning_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1697:type_function_user_udtf_generic_cloning_1`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_cloning_1

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_cloning_1() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    return types.copy(arg)
end

type test = <T, U>(T, { x: <T>(y: T) -> (), y: U }, U) -> ()

local function ok(idx: pass<test>): test return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_cloning_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1714:type_function_user_udtf_generic_cloning_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_cloning_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_cloning_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    return types.copy(arg)
end

type test = <T, U...>(T) -> (T, U...)

local function ok(idx: pass<test>): test return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_equality {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1731:type_function_user_udtf_generic_equality`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_equality

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_equality() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    return types.singleton(types.copy(arg) == arg)
end

type test = <T, U...>(T) -> (T, U...)

local function ok(idx: pass<test>): true return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_equality_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1933:type_function_user_udtf_generic_equality_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_equality_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_equality_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type function get()
    local T, Us = types.generic("T"), types.generic("U", true)

    local tbl1 = types.newtable({ [types.singleton("x")] = T })
    local tbl2 = types.newtable({ [types.singleton("x")] = Us }) -- it is possible to have invalid types in-flight

    return types.singleton(tbl1 == tbl2)
end

local function ok(idx: get<>): false return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_serialization_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1642:type_function_user_udtf_generic_serialization_1`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_serialization_1

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_serialization_1() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    return arg
end

type test = <T, U>(T, { x: <T>(y: T) -> (), y: U }, U) -> ()

local function ok(idx: pass<test>): test return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_serialization_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1659:type_function_user_udtf_generic_serialization_2`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_serialization_2

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_serialization_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    return arg
end

type test = <T, U...>(T) -> (T, U...)

local function ok(idx: pass<test>): test return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_generic_serialization_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1676:type_function_user_udtf_generic_serialization_3`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_generic_serialization_3

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_generic_serialization_3() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    return arg
end

local function m(a, b)
    return {x = a, y = b}
end

type test = typeof(m)

local function ok(idx: pass<test>): test return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_identity_preserves_parameter_names {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2937:type_function_user_udtf_identity_preserves_parameter_names`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item type_function_user_udtf_identity_preserves_parameter_names

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_identity_preserves_parameter_names() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id, records::function_type::FunctionType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _serialize_arg_names = ScopedFastFlag::new(&FFlag::LuauTypeFunctionSerializeArgNames, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function identity(t)
    return t
end

type baz = string
type foo = (foo: number, bar: baz) -> baz
type bar = identity<foo>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let bar_ty = fixture.base.require_type_alias(&String::from("bar"));
    let ftv = get_type_id::<FunctionType>(bar_ty).expect("expected FunctionType");

    assert_eq!(2, ftv.arg_names().len());
    let first = ftv.arg_names()[0]
      .as_ref()
      .expect("expected first argument name");
    assert_eq!("foo", first.name);
    let second = ftv.arg_names()[1]
      .as_ref()
      .expect("expected second argument name");
    assert_eq!("bar", second.name);
  }
}

mod type_function_user_udtf_intersection_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:572:type_function_user_udtf_intersection_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_intersection_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_intersection_methods_work() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
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
        type function getintersection()
            local tbl1 = types.newtable(nil, nil, nil)
            tbl1:setproperty(types.singleton("boolean"), types.boolean) -- {boolean: boolean}
            tbl1:setproperty(types.singleton("number"), types.number) -- {boolean: boolean, number: number}
            local tbl2 = types.newtable(nil, nil, nil)
            tbl2:setproperty(types.singleton("boolean"), types.boolean) -- {boolean: boolean}
            tbl2:setproperty(types.singleton("string"), types.string) -- {boolean: boolean, string: string}
            local ty = types.intersectionof(tbl1, tbl2)
            if ty:is("intersection") then
                -- creating a copy of `ty`
                local arr = {}
                for index, value in ty:components() do
                    table.insert(arr, value)
                end
                return types.intersectionof(table.unpack(arr))
            end
            -- this should never be returned
            return types.string
        end
        -- forcing an error here to check the exact type of the intersection
        local function ok(idx: getintersection<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ boolean: boolean, number: number } & { boolean: boolean, string: string }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_intersection_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:553:type_function_user_udtf_intersection_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_intersection_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_intersection_serialization_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function serialize_intersection(arg)
            return arg
        end
        type type_being_serialized = { boolean: boolean, number: number } & { boolean: boolean, string: string }
        -- forcing an error here to check the exact type of the intersection
        local function ok(idx: serialize_intersection<type_being_serialized>): nil return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ boolean: boolean, number: number } & { boolean: boolean, string: string }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_math_reset {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1222:type_function_user_udtf_math_reset`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - translates_to -> rust_item type_function_user_udtf_math_reset

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_math_reset() {
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
        type function foo(x)
            return types.singleton(tostring(math.random(1, 100)))
        end
        local x: foo<'a'> = ('' :: any) :: foo<'b'>
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_metatable_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:732:type_function_user_udtf_metatable_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_metatable_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_metatable_methods_work() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function getmetatable()
            local indexer = {
                index = types.number,
                readresult = types.boolean,
                writeresult = types.boolean,
            }
            local ty = types.newtable(nil, indexer, nil) -- {[number]: boolean}
            ty:setproperty(types.singleton("string"), types.number) -- {string: number, [number]: boolean}
            local metatbl = types.newtable(nil, nil, ty) -- { {  }, @metatable { [number]: boolean, string: number } }
            metatbl:setmetatable(types.newtable(nil, indexer, nil)) -- { {  }, @metatable { [number]: boolean } }
            local ret = metatbl:metatable()
            if metatbl:is("table") and metatbl:metatable() then
                return ret -- { @metatable { [number]: boolean } }
            end
            -- this should never be returned
            return types.number
        end
        -- forcing an error here to check the exact type of the metatable
        local function ok(idx: getmetatable<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("{boolean}", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_metatable_serialization_follows {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2652:type_function_user_udtf_metatable_serialization_follows`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_metatable_serialization_follows

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_metatable_serialization_follows() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
_ = setmetatable(_(),_),(_) or _ == _ or f
while _() do
export type function t0<O,I>(l0,l0)
end
type t39<A> = t0<{write [Vector3]:any},typeof(_),l0.t0,any>
end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_negation_inner {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:631:type_function_user_udtf_negation_inner`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_negation_inner

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_negation_inner() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(t)
    return types.negationof(t):inner()
end

type function fail(t)
    return t:inner()
end

local function ok(idx: pass<number>): number return idx end
local function notok(idx: fail<number>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'fail' type function errored at runtime: [string \"fail\"]:7: type.inner: cannot call inner method on non-negation type: `number` type",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_negation_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:607:type_function_user_udtf_negation_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_negation_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_negation_methods_work() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function getnegation()
            local ty = types.negationof(types.string)
            if ty:is("negation") then
                return ty
            end
            -- this should never be returned
            return types.number
        end

        -- forcing an error here to check the exact type of the negation
        local function ok(idx: getnegation<>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("~string", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_never_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:107:type_function_user_udtf_never_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_never_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_never_methods_work() {
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
        type function getnever()
            local ty = types.never
            if ty:is("never") then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getnever<>): never return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_never_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:92:type_function_user_udtf_never_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_never_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_never_serialization_works() {
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
        type function serialize_never(arg)
            return arg
        end
        type type_being_serialized = never
        local function ok(idx: serialize_never<type_being_serialized>): never return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_newtable_can_do_readonly_or_writeonly_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:674:type_function_user_udtf_newtable_can_do_readonly_or_writeonly_types`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_newtable_can_do_readonly_or_writeonly_types

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_newtable_can_do_readonly_or_writeonly_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function gettable()
            return types.newtable{[types.singleton("foo")] = { read = types.number }, [types.singleton("bar")] = { write = types.string }}
        end

        -- forcing an error here to check the exact type of the table
        local function ok(idx: gettable<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[1].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ write bar: string, read foo: number }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_nil_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:39:type_function_user_udtf_nil_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_nil_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_nil_methods_work() {
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
        type function getnil()
            local ty = types.singleton(nil)
            if ty:is("nil") then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getnil<>): nil return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_nil_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:24:type_function_user_udtf_nil_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_nil_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_nil_serialization_works() {
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
        type function serialize_nil(arg)
            return arg
        end
        type type_being_serialized = nil
        local function ok(idx: serialize_nil<type_being_serialized>): nil return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_no_shared_state {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1195:type_function_user_udtf_no_shared_state`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_no_shared_state

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_no_shared_state() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type function foo()
            if not glob then
                glob = 'a'
            else
                glob ..= 'b'
            end

            return glob
        end
        type function bar(prefix)
            return types.singleton(prefix:value() .. foo())
        end
        local function ok1(idx: bar<'x'>): nil return idx end
        local function ok2(idx: bar<'y'>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(5, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'glob'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "'bar' type function errored at runtime: [string \"foo\"]:4: attempt to modify a readonly table",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_user_udtf_number_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:209:type_function_user_udtf_number_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_number_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_number_methods_work() {
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
        type function getnumber()
            local ty = types.number
            if ty:is("number") then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getnumber<>): number return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_number_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:194:type_function_user_udtf_number_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_number_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_number_serialization_works() {
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
        type function serialize_num(arg)
            return arg
        end
        type type_being_serialized = number
        local function ok(idx: serialize_num<type_being_serialized>): number return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_optional_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:376:type_function_user_udtf_optional_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_optional_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_optional_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function numberhuh()
            return types.optional(types.number)
        end
        -- forcing an error here to check the exact type of the union
        local function ok(idx: numberhuh<>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("number?", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_optional_works_on_unions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:394:type_function_user_udtf_optional_works_on_unions`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_optional_works_on_unions

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_optional_works_on_unions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function foobar()
            local ty = types.unionof(types.string, types.number, types.boolean)
            return types.optional(ty)
        end
        -- forcing an error here to check the exact type of the union
        local function ok(idx: foobar<>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "(boolean | number | string)?",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_optionify {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1237:type_function_user_udtf_optionify`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_optionify

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_optionify() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
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
        type function optionify(tbl)
            if not tbl:is("table") then
                error("Argument is not a table")
            end
            for k, v in tbl:properties() do
                tbl:setproperty(k, types.unionof(v.read, types.singleton(nil)))
            end
            return tbl
        end
        type Person = {
            name: string,
            age: number,
            alive: boolean
        }
        local function ok(idx: optionify<Person>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ age: number?, alive: boolean?, name: string? }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_print_tab_char_fix {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2157:type_function_user_udtf_print_tab_char_fix`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_print_tab_char_fix

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_print_tab_char_fix() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver_v2 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function test(t)
            print(1,2)

            return t
        end

        local _:test<number>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!("1\t2", to_string_type_error(&result.errors[0]));
  }
}

mod type_function_user_udtf_recovery_no_upvalues {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1314:type_function_user_udtf_recovery_no_upvalues`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_recovery_no_upvalues

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_recovery_no_upvalues() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver_v2 = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local var

        type function save_upvalue(arg)
            var = 1
            return arg
        end

        type test = "test"
        local function ok(idx: save_upvalue<test>): "test"
            return idx
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type function cannot reference outer local 'var'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_recursion_and_gc {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1289:type_function_user_udtf_recursion_and_gc`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_recursion_and_gc

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_recursion_and_gc() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function foo(tbl)
            local count = 0
            for k,v in tbl:properties() do count += 1 end
            if count < 100 then
                tbl:setproperty(types.singleton(`m{count}`), types.string)
                foo(tbl)
            end
            for i = 1,100 do table.create(10000) end
            return tbl
        end
        type Test = {}
        local function ok(idx: foo<Test>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(_) => {}
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_serialize_iteration_limit_null_deref {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2968:type_function_user_udtf_serialize_iteration_limit_null_deref`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_serialize_iteration_limit_null_deref

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_serialize_iteration_limit_null_deref() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::{DFInt, FFlag};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);
    let _serde_limit = ScopedFastInt::new(&DFInt::LuauTypeFunctionSerdeIterationLimit, 10);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function pass(arg)
            return arg
        end

        type Complex = {
            a: number,
            b: string,
            c: boolean,
            d: nil,
            e: buffer,
            f: thread,
            g: (number, string) -> (boolean, nil),
        }

        local function ok(idx: pass<Complex>): Complex return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Complexity limit reached when passing a type to a type function",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_setgenerics_wrong_argcount_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3156:type_function_user_udtf_setgenerics_wrong_argcount_check`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_setgenerics_wrong_argcount_check

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_setgenerics_wrong_argcount_check() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function extra_arg()
            local f = types.newfunction()
            local g = types.generic("T")
            f:setgenerics({g}, "extra")
            return f
        end

        local x: extra_arg<> = nil
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Argument count mismatch. Function expects 1 to 2 arguments, but 3 are specified",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "'extra_arg' type function errored at runtime: [string \"extra_arg\"]:5: type.setgenerics: expected 2 arguments, but got 3",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_function_user_udtf_setmetatable_wrong_error_tag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3117:type_function_user_udtf_setmetatable_wrong_error_tag`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_setmetatable_wrong_error_tag

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_setmetatable_wrong_error_tag() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function foo()
            local t = types.newtable()
            t:setmetatable(types.number)
            return t
        end

        local x: foo<> = nil
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'foo' type function errored at runtime: [string \"foo\"]:4: type.setmetatable: expected the argument to be a table, but got number instead",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_simple_cyclic_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:950:type_function_user_udtf_simple_cyclic_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_simple_cyclic_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_simple_cyclic_serialization_works() {
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
        type function serialize_cycle(arg)
            return arg
        end
        type basety = {
            first: basety2
        }
        type basety2 = {
            second: basety
        }
        local function ok(idx: serialize_cycle<basety>): basety return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_singleton_equality_bool {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2094:type_function_user_udtf_singleton_equality_bool`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_singleton_equality_bool

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_singleton_equality_bool() {
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
type function compare(arg)
    return types.singleton(types.singleton(false) == arg)
end

local function ok1(idx: compare<false>): true return idx end
local function ok2(idx: compare<true>): false return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_singleton_equality_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2117:type_function_user_udtf_singleton_equality_string`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_singleton_equality_string

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_singleton_equality_string() {
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
type function compare(arg)
    return types.singleton(types.singleton("") == arg)
end

local function ok(idx: compare<"">): true return idx end
local function ok(idx: compare<"a">): false return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_string_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:270:type_function_user_udtf_string_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_string_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_string_methods_work() {
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
        type function getstring()
            local ty = types.string
            if ty:is("string") then
                return ty
            end
            -- this should never be returned
            return types.boolean
        end
        local function ok(idx: getstring<>): string return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_string_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:255:type_function_user_udtf_string_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_string_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_string_serialization_works() {
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
        type function serialize_str(arg)
            return arg
        end
        type type_being_serialized = string
        local function ok(idx: serialize_str<type_being_serialized>): string return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_strip_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1352:type_function_user_udtf_strip_indexer`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function strip (Common/src/StringUtils.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_strip_indexer

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_strip_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
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
        type function stripindexer(tbl)
            if not tbl:is("table") then
                error("can only strip the indexer on a table!")
            end
            tbl:setindexer(types.never, types.never)
            return tbl
        end

        type map = { [number]: string, foo: string }
        -- forcing an error here to check the exact type
        local function ok(tbl: stripindexer<map>): never return tbl end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("{ foo: string }", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_strsingleton_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:338:type_function_user_udtf_strsingleton_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_strsingleton_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_strsingleton_methods_work() {
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
        type function getstrsingleton()
            local ty = types.singleton("hungry hippo")
            if ty:is("singleton") and ty:value() == "hungry hippo" then
                return ty
            end
            -- this should never be returned
            return types.number
        end
        local function ok(idx: getstrsingleton<>): "hungry hippo" return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_strsingleton_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:323:type_function_user_udtf_strsingleton_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_strsingleton_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_strsingleton_serialization_works() {
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
        type function serialize_strsingleton(arg)
            return arg
        end
        type type_being_serialized = "popcorn and movies!"
        local function ok(idx: serialize_strsingleton<type_being_serialized>): "popcorn and movies!" return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_table_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:694:type_function_user_udtf_table_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_table_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_table_methods_work() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function gettable()
            local indexer = {
                index = types.number,
                readresult = types.boolean,
                writeresult = types.boolean,
            }
            local ty = types.newtable(nil, indexer, nil) -- {[number]: boolean}
            ty:setproperty(types.singleton("string"), types.number) -- {string: number, [number] = boolean}
            ty:setproperty(types.singleton("number"), types.string) -- {string: number, number: string, [number] = boolean}
            ty:setproperty(types.singleton("string"), nil) -- {number: string, [number] = boolean}
            local ret = types.newtable(nil, nil, nil) -- {}
            -- creating a copy of `ty`
            for k, v in ty:properties() do
                ret:setreadproperty(k, v.read)
                ret:setwriteproperty(k, v.write)
            end
            if ret:is("table") then
                ret:setindexer(types.boolean, types.string) -- {number: string, [boolean] = string}
                return ret -- {number: string, [boolean] = string}
            end
            -- this should never be returned
            return types.number
        end
        -- forcing an error here to check the exact type of the table
        local function ok(idx: gettable<>): never return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ [boolean]: string, number: string }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_table_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:655:type_function_user_udtf_table_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_table_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_table_serialization_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function serialize_table(arg)
            return arg
        end
        type type_being_serialized = { boolean: boolean, number: number, [string]: number }
        -- forcing an error here to check the exact type of the table
        local function ok(idx: serialize_table<type_being_serialized>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "{ [string]: number, boolean: boolean, number: number }",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_type_alias_call_serialize_null_deref {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2996:type_function_user_udtf_type_alias_call_serialize_null_deref`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_type_alias_call_serialize_null_deref

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_type_alias_call_serialize_null_deref() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::{DFInt, FFlag};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);
    let _serde_limit = ScopedFastInt::new(&DFInt::LuauTypeFunctionSerdeIterationLimit, 10);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Big<T> = {
            a: T,
            b: string,
            c: boolean,
            d: nil,
            e: buffer,
            f: thread,
            g: (T, string) -> (boolean, nil),
        }

        type function apply(arg)
            return Big(arg)
        end

        local function ok(idx: apply<number>): Big<number> return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'apply' type function errored at runtime: [string \"apply\"]:13: Complexity limit reached when passing a type to a type alias",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_type_alias_registration_follows {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2637:type_function_user_udtf_type_alias_registration_follows`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_type_alias_registration_follows

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_type_alias_registration_follows() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
export type t110 = ""type--"
function _<t32...,t0...,t0...,t0...>(...):(any)&(any)
end
if _ then
else
    export type t110 = ""type--"
    function _<t32...,t0...,t0...,t0...>(...):(any)&(any)
    end
end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_type_overrides_call_metamethod {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1040:type_function_user_udtf_type_overrides_call_metamethod`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_type_overrides_call_metamethod

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_type_overrides_call_metamethod() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function hello(arg)
            error(type(arg))
        end
        local function ok(idx: hello<string>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::UserDefinedTypeFunctionError(e) => {
        assert_eq!(
          "'hello' type function errored at runtime: [string \"hello\"]:3: userdata",
          e.message()
        );
      }
      other => panic!("expected UserDefinedTypeFunctionError, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_type_overrides_eq_metamethod {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1058:type_function_user_udtf_type_overrides_eq_metamethod`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_type_overrides_eq_metamethod

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_type_overrides_eq_metamethod() {
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
        type function hello()
            local p1 = types.string
            local p2 = types.string
            local t1 = types.newtable(nil, nil, nil)
            t1:setproperty(types.singleton("string"), types.boolean)
            t1:setmetatable(t1)
            local t2 = types.newtable(nil, nil, nil)
            t2:setproperty(types.singleton("string"), types.boolean)
            t1:setmetatable(t1)
            if p1 == p2 and t1 == t2 then
                return types.number
            end
            return types.unknown
        end
        local function ok(idx: hello<>): number return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_union_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:413:type_function_user_udtf_union_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_union_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_union_methods_work() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function getunion()
            local ty = types.unionof(types.string, types.number, types.boolean)
            if ty:is("union") then
                -- creating a copy of `ty`
                local arr = {}
                for _, value in ty:components() do
                    table.insert(arr, value)
                end
                return types.unionof(table.unpack(arr))
            end
            -- this should never be returned
            return types.number
        end
        -- forcing an error here to check the exact type of the union
        local function ok(idx: getunion<>): never return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "boolean | number | string",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_union_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:357:type_function_user_udtf_union_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_union_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_union_serialization_works() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
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
        type function serialize_union(arg)
            return arg
        end
        type type_being_serialized = number | string | boolean
        -- forcing an error here to check the exact type of the union
        local function ok(idx: serialize_union<type_being_serialized>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!(
          "boolean | number | string",
          to_string_type_id(tm.given_type)
        );
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_unknown_methods_work {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:73:type_function_user_udtf_unknown_methods_work`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_unknown_methods_work

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_unknown_methods_work() {
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
        type function getunknown()
            local ty = types.unknown
            if ty:is("unknown") then
                return ty
            end
            -- this should never be returned
            return types.string
        end
        local function ok(idx: getunknown<>): unknown return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_unknown_serialization_works {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:58:type_function_user_udtf_unknown_serialization_works`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_unknown_serialization_works

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_unknown_serialization_works() {
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
        type function serialize_unknown(arg)
            return arg
        end
        type type_being_serialized = unknown
        local function ok(idx: serialize_unknown<type_being_serialized>): unknown return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_udtf_unsupported_type_function_application_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2919:type_function_user_udtf_unsupported_type_function_application_error`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_function_user_udtf_unsupported_type_function_application_error

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_unsupported_type_function_application_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _structured_errors = ScopedFastFlag::new(&FFlag::LuauTypeFunctionStructuredErrors, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function f<D, l>(data: D & {}, index: l | "Test"): index<D, l>
    return data[index]
end

local test = f :: test<typeof(f)>
type function test(t: type) return t end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Type functions do not currently support types of the form 'index<D, l>'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_function_user_udtf_user_error_is_reported {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1018:type_function_user_udtf_user_error_is_reported`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_udtf_user_error_is_reported

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_user_error_is_reported() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
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
        type function errors_if_string(arg)
            if arg:is("string") then
                local a = 1
                error("We are in a math class! not english")
            end
            return arg
        end
        local function ok(idx: errors_if_string<string>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::UserDefinedTypeFunctionError(e) => {
        assert_eq!(
          "'errors_if_string' type function errored at runtime: [string \"errors_if_string\"]:5: We are in a math class! not english",
          e.message()
        );
      }
      other => panic!("expected UserDefinedTypeFunctionError, got {other:?}"),
    }
  }
}

mod type_function_user_udtf_variadic_api {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2073:type_function_user_udtf_variadic_api`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_function_user_udtf_variadic_api

  #[cfg(test)]
  #[test]
  fn type_function_user_udtf_variadic_api() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type function pass(arg)
    local p, r = arg:parameters(), arg:returns()
    local f = types.newfunction()
    f:setparameters({p.tail}, p.head[1]);
    f:setreturns({r.tail}, r.head[1]);
    return f
end

type test = (string, ...number) -> (number, ...string)

local function ok(idx: pass<test>): (number, ...string) -> (string, ...number) return idx end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_function_user_write_of_readonly_is_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:861:type_function_user_write_of_readonly_is_nil`
  //! Source: `tests/TypeFunction.user.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeFunction.user.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeFunction.user.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_function_user_write_of_readonly_is_nil

  #[cfg(test)]
  #[test]
  fn type_function_user_write_of_readonly_is_nil() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function getclass(arg)
            local props = arg:properties()
            local table = types.newtable(props)
            local singleton = types.singleton("BaseMethod")

            if table:writeproperty(singleton) then
                return types.singleton(true)
            else
                return types.singleton(false)
            end
        end
        -- forcing an error here to check the exact type of the metatable
        local function ok(idx: getclass<BaseClass>): nil return idx end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
      TypeErrorData::TypeMismatch(tm) => {
        assert_eq!("false", to_string_type_id(tm.given_type));
      }
      other => panic!("expected TypeMismatch, got {other:?}"),
    }
  }
}

mod non_string_error_value {
  #[cfg(test)]
  #[test]
  fn non_string_error_value() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _structured_errors = ScopedFastFlag::new(&FFlag::LuauTypeFunctionStructuredErrors, true);
    let _fix_type_name_typo = ScopedFastFlag::new(&FFlag::LuauUdtfFixTypeNameTypo, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type function foo()
            error({})
        end

        local x: foo<> = 5
      "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'foo' type function errored at runtime: raised an error of type table",
      to_string_type_error(&result.errors[0])
    );
  }
}
