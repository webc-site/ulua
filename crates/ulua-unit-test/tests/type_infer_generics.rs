extern crate alloc;

mod type_infer_generics_apply_type_function_nested_generics_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1572:type_infer_generics_apply_type_function_nested_generics_1`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_apply_type_function_nested_generics_1

  #[cfg(test)]
  #[test]
  fn type_infer_generics_apply_type_function_nested_generics_1() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type MyObject = {
            getReturnValue: <V>(cb: () -> V) -> V
        }
        local object: MyObject = {
            getReturnValue = function<U>(cb: () -> U): U
                return cb()
            end,
        }

        type ComplexObject<T> = {
            id: T,
            nested: MyObject
        }

        local complex: ComplexObject<string> = {
            id = "Foo",
            nested = object,
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_apply_type_function_nested_generics_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1600:type_infer_generics_apply_type_function_nested_generics_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_apply_type_function_nested_generics_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_apply_type_function_nested_generics_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type MyObject = {
	getReturnValue: <V>(cb: () -> V) -> V
}
type ComplexObject<T> = {
	id: T,
	nested: MyObject
}

function f(complex: ComplexObject<string>)
    local x = complex.nested.getReturnValue(function(): string
        return ""
    end)

    local y = complex.nested.getReturnValue(function()
        return 3
    end)
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_apply_type_function_nested_generics_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1627:type_infer_generics_apply_type_function_nested_generics_3`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_apply_type_function_nested_generics_3

  #[cfg(test)]
  #[test]
  fn type_infer_generics_apply_type_function_nested_generics_3() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local getReturnValue: <V>(cb: () -> V) -> V = nil :: any

        local y = getReturnValue(function() return nil :: any end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_better_mismatch_error_messages {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_better_mismatch_error_messages() {
    use alloc::string::String;

    use ulua_analysis::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<T>(...: T...)
            return ...
        end

        function g<T...>(a: T)
            return a
        end
    "#,
      ),
      None,
    );

    let (f_err_index, g_err_index) = if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(3, result.errors.len(), "{:?}", result.errors);
      (1, 2)
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      (0, 1)
    };

    let f_err = type_error_data_ref::<SwappedGenericTypeParameter>(&result.errors[f_err_index])
      .expect("expected SwappedGenericTypeParameter");
    assert_eq!("T", f_err.name);
    assert_eq!(SwappedGenericTypeParameter::PACK, f_err.kind);

    let g_err = type_error_data_ref::<SwappedGenericTypeParameter>(&result.errors[g_err_index])
      .expect("expected SwappedGenericTypeParameter");
    assert_eq!("T", g_err.name);
    assert_eq!(SwappedGenericTypeParameter::TYPE, g_err.kind);
  }
}

mod type_infer_generics_bidirectional_checking_and_generalization_play_nice {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1736:type_infer_generics_bidirectional_checking_and_generalization_play_nice`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_bidirectional_checking_and_generalization_play_nice

  #[cfg(test)]
  #[test]
  fn type_infer_generics_bidirectional_checking_and_generalization_play_nice() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = function(a)
            return a()
        end

        local a = foo(function() return 1 end)
        local b = foo(function() return "bar" end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_generics_bound_tables_do_not_clone_original_fields {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_bound_tables_do_not_clone_original_fields() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local exports = {}
local nested = {}

nested.name = function(t, k)
    local a = t.x.y
    return rawget(t, k)
end

exports.nested = nested
return exports
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_calling_self_generic_methods {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_calling_self_generic_methods() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = {}
        function x:id(x) return x end
        function x:f()
            local x: string = self:id("hi")
            local y: number = self:id(37)
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "expected errors");
  }
}

mod type_infer_generics_check_generic_function {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_generic_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id<a>(x:a): a
            return x
        end
        local x: string = id("hi")
        local y: number = id(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_generics_check_generic_local_function {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_generic_local_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<a>(x:a): a
            return x
        end
        local x: string = id("hi")
        local y: number = id(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_generics_check_generic_local_function_2 {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_generic_local_function_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<a>(x:a): a
            return x
        end
        local x = id("hi")
        local y = id(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_generics_check_generic_typepack_function {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_generic_typepack_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id<a...>(...: a...): (a...) return ... end
        local x: string, y: boolean = id("hi", true)
        local z: number = id(37)
        id()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_check_mutual_generic_functions {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_mutual_generic_functions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id1<a>(x:a):a
            local y: string = id2("hi")
            local z: number = id2(37)
            return x
        end

        function id2<a>(x:a):a
            local y: string = id1("hi")
            local z: number = id1(37)
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_check_mutual_generic_functions_errors {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_mutual_generic_functions_errors() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id1(x)
            local y: string = id2(37) -- odd
            local z: number = id2("hi") -- even
            return x
        end

        function id2(x)
            local y: string = id1(37) -- odd
            local z: number = id1("hi") -- even
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    for i in (0..4).step_by(2) {
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[i]).expect("expected TypeMismatch");
      assert_eq!("string", to_string_type_id(tm.wanted_type));
      assert_eq!("number", to_string_type_id(tm.given_type));
    }

    for i in (1..4).step_by(2) {
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[i]).expect("expected TypeMismatch");
      assert_eq!("number", to_string_type_id(tm.wanted_type));
      assert_eq!("string", to_string_type_id(tm.given_type));
    }
  }
}

mod type_infer_generics_check_mutual_generic_functions_unannotated {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_mutual_generic_functions_unannotated() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id1(x)
            local y: string = id2("hi")
            local z: number = id2(37)
            return x
        end

        function id2(x)
            local y: string = id1("hi")
            local z: number = id1(37)
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_check_nested_generic_function {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_nested_generic_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f()
            local function id<a>(x:a): a
                return x
            end
            local x: string = id("hi")
            local y: number = id(37)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_check_recursive_generic_function {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_check_recursive_generic_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<a>(x:a):a
            local y: string = id("hi")
            local z: number = id(37)
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_cli_179086_dont_ignore_explicit_variadics {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2085:type_infer_generics_cli_179086_dont_ignore_explicit_variadics`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_cli_179086_dont_ignore_explicit_variadics

  #[cfg(test)]
  #[test]
  fn type_infer_generics_cli_179086_dont_ignore_explicit_variadics() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        type Example<T...> = { Method: (T...) -> () }

        local function CreateExample<T...>(Method: (T...) -> ()): Example<T...>
            local self = {}
            self.Method = Method
            return self
        end

        local Object: Example<string> = CreateExample(function(a: string) end)

        Object.Method("Hello World!")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2157:type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing

  #[cfg(test)]
  #[test]
  fn type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _instantiate_before_push =
      ScopedFastFlag::new(&FFlag::LuauInstantiateFunctionTypeBeforePush, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Parent = {
            Func1:<P...> (self: Parent, value: boolean, P...) -> (Parent?),
            Func2: (self: Parent, value: boolean) -> (Parent?),
        }

        export type Child = {
            Parent: Parent,
            Func: (self: Child) -> (Child?),
        }

        local Parent = {} :: Parent
        local Child = {} :: Child

        function Parent:Func1(value, ...)
            if value then return self else return nil end
        end

        function Parent:Func2(value)
            if value then return self else return nil end
        end

        function Child:Func()
            if math.random() > 0.5 then return self else return nil end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_correctly_instantiate_polymorphic_member_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1205:type_infer_generics_correctly_instantiate_polymorphic_member_functions`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_generics_correctly_instantiate_polymorphic_member_functions

  #[cfg(test)]
  #[test]
  fn type_infer_generics_correctly_instantiate_polymorphic_member_functions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{
        function_type::FunctionType, primitive_type::PrimitiveType, table_type::TableType,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _assert_on_forced_constraint =
      ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local T = {}

        function T:foo()
            return T:bar(5)
        end

        function T:bar(i)
            return i
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = get_type_id::<TableType>(fixture.require_type_string(&String::from("T")))
      .expect("expected TableType");

    let foo_prop = t
      .props
      .get(&String::from("foo"))
      .expect("expected foo property");
    let foo_ty = foo_prop.read_ty.expect("expected readable foo type");
    let foo_ty = follow_type_id(foo_ty);
    let foo = get_type_id::<FunctionType>(foo_ty).expect("expected FunctionType for foo");

    let ret = first(foo.ret_types(), false).expect("expected return type");
    let ret = follow_type_id(ret);

    assert_eq!(Some(PrimitiveType::NUMBER), fixture.get_primitive_type(ret));
  }
}

mod type_infer_generics_do_not_always_instantiate_generic_intersection_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1674:type_infer_generics_do_not_always_instantiate_generic_intersection_types`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_do_not_always_instantiate_generic_intersection_types

  #[cfg(test)]
  #[test]
  fn type_infer_generics_do_not_always_instantiate_generic_intersection_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Array<T> = { [number]: T }

        type Array_Statics = {
            new: <T>() -> Array<T>,
        }

        local _Arr : Array<any> & Array_Statics = {} :: Array_Statics
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_do_not_infer_generic_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1502:type_infer_generics_do_not_infer_generic_functions`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_do_not_infer_generic_functions

  #[cfg(test)]
  #[test]
  fn type_infer_generics_do_not_infer_generic_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = if !FFlag::DebugLuauForceOldSolver.get() {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local function sum<T>(x: T, y: T, z: (T, T) -> T) return z(x, y) end

            local function sumrec(f: typeof(sum))
                return sum(2, 3, function<X>(g: X, h: X): add<X, X> return g + h end)
            end

            local b = sumrec(sum) -- ok
            local c = sumrec(
                function(d, e, f)
                    return f(d, e)
                end
            ) -- type binders are not inferred
        "#,
        ),
        None,
      );

      assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
      );
      assert_eq!(
        "<T>(T, T, (T, T) -> T) -> T",
        to_string_type_id(fixture.base.require_type_string(&String::from("sum")))
      );
      assert_eq!(
        "<T>(T, T, (T, T) -> T) -> T",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
          line: 7,
          column: 29
        }))
      );
      result
    } else {
      fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local function sum<a>(x: a, y: a, f: (a, a) -> a) return f(x, y) end

            local function sumrec(f: typeof(sum))
                return sum(2, 3, function(a, b) return a + b end)
            end

            local b = sumrec(sum) -- ok
            local c = sumrec(function(x, y, f) return f(x, y) end) -- type binders are not inferred
        "#,
        ),
        None,
      )
    };

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_do_not_infer_generic_functions_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1543:type_infer_generics_do_not_infer_generic_functions_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_do_not_infer_generic_functions_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_do_not_infer_generic_functions_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _force_new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type t = <a>(a, a, (a, a) -> a) -> a
        type u = (number, number, <X>(X, X) -> X) -> number

        local foo = (nil :: any) :: t
        local bar : u = foo
        "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_dont_leak_generic_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_dont_leak_generic_types() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(y)
            -- this will only typecheck if we infer z: any
            -- so f: (any)->(any)
            local z = y
            local function id(x)
                z = x -- this assignment is what forces z: any
                return x
            end
            local x: string = id("hi")
            local y: number = id(37)
            return z
        end
        -- so this assignment should fail
        local b: boolean = f(true)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert!(!result.errors.is_empty(), "{:?}", result.errors);
    }
  }
}

mod type_infer_generics_dont_leak_inferred_generic_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_dont_leak_inferred_generic_types() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(y)
            local z = y
            local function id(x)
                z = x
                return x
            end
            local x: string = id("hi")
            local y: number = id(37)
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert!(!result.errors.is_empty(), "{:?}", result.errors);
    }
  }
}

mod type_infer_generics_dont_substitute_bound_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_dont_substitute_bound_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { m: <a>(a) -> T }
        function f(t : T)
            local x: T = t.m(37)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_dont_unify_bound_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:497:type_infer_generics_dont_unify_bound_types`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_dont_unify_bound_types

  #[cfg(test)]
  #[test]
  fn type_infer_generics_dont_unify_bound_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type F = <a>() -> <b>(a, b) -> a
        type G = <b>(b, b) -> b
        local f: F = function<a>()
          local x
          return function<b>(y: a, z: b): a
            if not(x) then x = y end
            return x
          end
        end
        -- This assignment shouldn't typecheck
        -- If it does, it means we instantiated
        -- f as () -> <b>(X, b) -> X, then unified X to be b
        local g: G = f()
        -- Oh dear, if that works then the type system is unsound
        local a : string = g("not a number", "hi")
        local b : number = g(5, 37)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_generics_duplicate_generic_type_packs {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_duplicate_generic_type_packs() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,a...>() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_duplicate_generic_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_duplicate_generic_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a,a>(x:a):a return x end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_ensure_that_invalid_generic_instantiations_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2003:type_infer_generics_ensure_that_invalid_generic_instantiations_error`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_ensure_that_invalid_generic_instantiations_error

  #[cfg(test)]
  #[test]
  fn type_infer_generics_ensure_that_invalid_generic_instantiations_error() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local func: <T>(T, (T) -> ()) -> () = nil :: any
        local foobar: (number) -> () = nil :: any
        func({}, foobar)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  }
}

mod type_infer_generics_ensure_that_invalid_generic_instantiations_error_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2015:type_infer_generics_ensure_that_invalid_generic_instantiations_error_1`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_ensure_that_invalid_generic_instantiations_error_1

  #[cfg(test)]
  #[test]
  fn type_infer_generics_ensure_that_invalid_generic_instantiations_error_1() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        function insert<T>(arr: {T}, value: T)
            return arr
        end

        local a: {number} = {}

        local b = insert(a, "five")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  }
}

mod type_infer_generics_error_detailed_function_mismatch_generic_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:841:type_infer_generics_error_detailed_function_mismatch_generic_pack`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericTypePackCountMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_error_detailed_function_mismatch_generic_pack

  #[cfg(test)]
  #[test]
  fn type_infer_generics_error_detailed_function_mismatch_generic_pack() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        generic_type_pack_count_mismatch::GenericTypePackCountMismatch, type_mismatch::TypeMismatch,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type C = () -> ()
type D = <T...>() -> ()

local c: C
local d: D = c
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      let generic_mismatch = type_error_data_ref::<GenericTypePackCountMismatch>(&result.errors[0])
        .expect("expected GenericTypePackCountMismatch");
      assert_eq!(1, generic_mismatch.sub_ty_generic_pack_count());
      assert_eq!(0, generic_mismatch.super_ty_generic_pack_count());

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!("() -> ()", to_string_type_id(mismatch.given_type));
      assert_eq!("<T...>() -> ()", to_string_type_id(mismatch.wanted_type));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        "different number of generic type pack parameters",
        mismatch.reason
      );
    }
  }
}

mod type_infer_generics_error_detailed_function_mismatch_generic_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:778:type_infer_generics_error_detailed_function_mismatch_generic_types`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericTypeCountMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_error_detailed_function_mismatch_generic_types

  #[cfg(test)]
  #[test]
  fn type_infer_generics_error_detailed_function_mismatch_generic_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        generic_type_count_mismatch::GenericTypeCountMismatch, type_mismatch::TypeMismatch,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type C = () -> ()
type D = <T>() -> ()

local c: C
local d: D = c
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      let generic_mismatch = type_error_data_ref::<GenericTypeCountMismatch>(&result.errors[0])
        .expect("expected GenericTypeCountMismatch");
      assert_eq!(1, generic_mismatch.sub_ty_generic_count());
      assert_eq!(0, generic_mismatch.super_ty_generic_count());

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!("() -> ()", to_string_type_id(mismatch.given_type));
      assert_eq!("<T>() -> ()", to_string_type_id(mismatch.wanted_type));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        "different number of generic type parameters",
        mismatch.reason
      );
    }
  }
}

mod type_infer_generics_factories_of_generics {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_factories_of_generics() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { id: <a>(a) -> a }
        type Factory = { build: () -> T }

        local f: Factory = {
            build = function(): T
                return {
                    id = function<a>(x:a):a
                        return x
                    end
                }
            end
        }
        local x: T = f.build()
        local y: string = x.id("hi")
        local z: number = x.id(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_follow_bound_type_packs_in_generic_type_visitor {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1878:type_infer_generics_follow_bound_type_packs_in_generic_type_visitor`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_generics_follow_bound_type_packs_in_generic_type_visitor

  #[cfg(test)]
  #[test]
  fn type_infer_generics_follow_bound_type_packs_in_generic_type_visitor() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function (_(_,_,nil))
(if l0 then typeof else `{_:_()}`,typeof).n0<A...,A...>(l0)
function _:_():typeof<A...>()
end
function _:_().typeof<A...>()
end
end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_generics_function_arguments_can_be_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_function_arguments_can_be_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(g: <a>(a)->a)
            local x: number = g(37)
            local y: string = g("hi")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_function_results_can_be_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_function_results_can_be_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f() : <a>(a)->a
            local function id<a>(x:a):a return x end
            return id
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generalization_no_cyclic_intersections {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1753:type_infer_generics_generalization_no_cyclic_intersections`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_generalization_no_cyclic_intersections

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generalization_no_cyclic_intersections() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f, t, n = pairs({"foo"})
        local k, v = f(t)
    "#,
      ),
      None,
    );

    assert_eq!(
      "({string}, number?) -> (number?, string)",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
    assert_eq!(
      "{string}",
      to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("k")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("v")))
    );
  }
}

mod type_infer_generics_generic_argument_count_just_right {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1043:type_infer_generics_generic_argument_count_just_right`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_generic_argument_count_just_right

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_argument_count_just_right() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function test2(a: number, b: string)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test2, 1, "")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_argument_count_too_few {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:991:type_infer_generics_generic_argument_count_too_few`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_argument_count_too_few

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_argument_count_too_few() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function test(a: number)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let cm =
        type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
      assert_eq!(2, cm.expected());
      assert_eq!(1, cm.actual());
      assert_eq!(CountMismatch::ARG, cm.context());
    } else {
      assert_eq!(
        "Argument count mismatch. Function 'wrapper' expects 2 arguments, but only 1 is specified",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_generics_generic_argument_count_too_many {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1017:type_infer_generics_generic_argument_count_too_many`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_argument_count_too_many

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_argument_count_too_many() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error, records::count_mismatch::CountMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function test2(a: number, b: string)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test2, 1, "", 3)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let cm =
        type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
      assert_eq!(3, cm.expected());
      assert_eq!(4, cm.actual());
      assert_eq!(CountMismatch::ARG, cm.context());
    } else {
      assert_eq!(
        "Argument count mismatch. Function 'wrapper' expects 3 arguments, but 4 are specified",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_generics_generic_argument_pack_type_inferred_from_return {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1059:type_infer_generics_generic_argument_pack_type_inferred_from_return`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_generic_argument_pack_type_inferred_from_return

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_argument_pack_type_inferred_from_return() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function test2(a: number)
            return "hello"
        end

        function wrapper<A...>(f: (number) -> A..., ...: A...)
        end

        wrapper(test2, 1)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("string", to_string_type_id(tm.wanted_type));
      assert_eq!("number", to_string_type_id(tm.given_type));
    } else {
      assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_generics_generic_argument_pack_type_inferred_from_return_no_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1087:type_infer_generics_generic_argument_pack_type_inferred_from_return_no_error`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_argument_pack_type_inferred_from_return_no_error

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_argument_pack_type_inferred_from_return_no_error() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function test2(a: number)
    return "hello"
end

function wrapper<A...>(f: (number) -> A..., ...: A...)
end

wrapper(test2, "hello")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_factories {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_factories() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<a> = { id: (a) -> a }
        type Factory = { build: <a>() -> T<a> }

        local f: Factory = {
            build = function<a>(): T<a>
                return {
                    id = function(x:a):a
                        return x
                    end
                }
            end
        }
        local y: string = f.build().id("hi")
        local z: number = f.build().id(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1159:type_infer_generics_generic_function`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_function

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id(x) return x end
        local a = id(55)
        local b = id(nil)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a>(a) -> a",
      to_string_type_id(fixture.require_type_string(&String::from("id")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_generics_generic_function_mismatch_with_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:809:type_infer_generics_generic_function_mismatch_with_argument`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record GenericTypeCountMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_function_mismatch_with_argument

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_function_mismatch_with_argument() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        generic_type_count_mismatch::GenericTypeCountMismatch, type_mismatch::TypeMismatch,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type C = (number) -> ()
type D = <T>(number) -> ()

local c: C
local d: D = c
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      let generic_mismatch = type_error_data_ref::<GenericTypeCountMismatch>(&result.errors[0])
        .expect("expected GenericTypeCountMismatch");
      assert_eq!(1, generic_mismatch.sub_ty_generic_count());
      assert_eq!(0, generic_mismatch.super_ty_generic_count());

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!("(number) -> ()", to_string_type_id(mismatch.given_type));
      assert_eq!("<T>(number) -> ()", to_string_type_id(mismatch.wanted_type));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);

      let mismatch =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        "different number of generic type parameters",
        mismatch.reason
      );
    }
  }
}

mod type_infer_generics_generic_functions_dont_cache_type_parameters {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:873:type_infer_generics_generic_functions_dont_cache_type_parameters`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_functions_dont_cache_type_parameters

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_functions_dont_cache_type_parameters() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
-- See https://github.com/luau-lang/luau/issues/332
-- This function has a type parameter with the same name as clones,
-- so if we cache type parameter names for functions these get confused.
-- function id<Z>(x : Z) : Z
function id<X>(x : X) : X
  return x
end

function clone<X, Y>(dict: {[X]:Y}): {[X]:Y}
  local copy = {}
  for k, v in pairs(dict) do
    copy[k] = v
  end
  return copy
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_functions_in_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_functions_in_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { id: <a>(a) -> a }
        local x: T = { id = function<a>(x:a):a return x end }
        local y: string = x.id("hi")
        local z: number = x.id(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_functions_should_be_memory_safe {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:896:type_infer_generics_generic_functions_should_be_memory_safe`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_functions_should_be_memory_safe

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_functions_should_be_memory_safe() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
-- At one point this produced a UAF
type T<a> = { a: U<a>, b: a }
type U<a> = { c: T<a>?, d : a }
local x: T<number> = { a = { c = nil, d = 5 }, b = 37 }
x.a.c = x
local y: T<string> = { a = { c = nil, d = 5 }, b = 37 }
y.a.c = y
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);

      let mismatch1 =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      let mismatch2 =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");

      assert_eq!(
        Location {
          begin: Position {
            line: 7,
            column: 42
          },
          end: Position {
            line: 7,
            column: 43
          },
        },
        result.errors[0].location
      );
      assert_eq!("number", to_string_type_id(mismatch1.given_type));
      assert_eq!("string", to_string_type_id(mismatch1.wanted_type));

      assert_eq!(
        Location {
          begin: Position {
            line: 7,
            column: 51
          },
          end: Position {
            line: 7,
            column: 53
          },
        },
        result.errors[1].location
      );
      assert_eq!("number", to_string_type_id(mismatch2.given_type));
      assert_eq!("string", to_string_type_id(mismatch2.wanted_type));
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      let expected = "Expected this to be exactly 'T<string>', but got 'y'\n\
caused by:\n  \
Property 'a' is not compatible.\n\
Expected this to be exactly 'U<string>', but got '{| c: T<string>?, d: number |}'\n\
caused by:\n  \
Property 'd' is not compatible.\n\
Expected this to be exactly 'string', but got 'number'";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_generics_generic_implicit_explicit_name_clash {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1778:type_infer_generics_generic_implicit_explicit_name_clash`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_implicit_explicit_name_clash

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_implicit_explicit_name_clash() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function apply<a>(func, argument: a)
            return func(argument)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<a, b...>((a) -> (b...), a) -> (b...)",
      to_string_type_id(fixture.require_type_string(&String::from("apply")))
    );
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1892:type_infer_generics_generic_packs_in_contravariant_position`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f<A>(foo: (A) -> ()): () end
function g<B...>(...: B...): () end
f(g)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1903:type_infer_generics_generic_packs_in_contravariant_position_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: (number) -> (number)): () end
type T = <A...>(A...) -> A...
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1915:type_infer_generics_generic_packs_in_contravariant_position_3`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_3

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_3() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: <B...>(B...) -> B...): () end
type T = <A...>(A...) -> A...
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_4 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1927:type_infer_generics_generic_packs_in_contravariant_position_4`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_4

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_4() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: <A...>(A...) -> A...): () end
type T = <B..., C...>(B...) -> C...
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_5 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1941:type_infer_generics_generic_packs_in_contravariant_position_5`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_5

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_5() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: (number) -> number): () end
type T = <A...>(A...) -> number
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_6 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1953:type_infer_generics_generic_packs_in_contravariant_position_6`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_6

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_6() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: (...number) -> number): () end
type T = <A...>(A...) -> number
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_7 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1965:type_infer_generics_generic_packs_in_contravariant_position_7`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_7

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_7() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: () -> ()): () end
type T = <A...>() -> A...
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_packs_in_contravariant_position_8 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1977:type_infer_generics_generic_packs_in_contravariant_position_8`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_packs_in_contravariant_position_8

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_packs_in_contravariant_position_8() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
function f(foo: () -> ()): () end
type T = <A...>(A...) -> A...
local t: T
f(t)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_type_functions_work_in_subtyping {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1791:type_infer_generics_generic_type_functions_work_in_subtyping`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_type_functions_work_in_subtyping

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_functions_work_in_subtyping() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function addOne<T>(x: T): add<T, number> return x + 1 end

        local function six(): number
            return addOne(5)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_type_pack_parentheses {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_pack_parentheses() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...>(...: a...): any return (...) end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_generics_generic_type_pack_syntax {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_pack_syntax() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...>(...: a...): (a...) return ... end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a...>(a...) -> (a...)",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_generics_generic_type_pack_unification_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:937:type_infer_generics_generic_type_pack_unification_1`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_type_pack_unification_1

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_pack_unification_1() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type Dispatcher = {
	useMemo: <T...>(create: () -> T...) -> T...
}

local TheDispatcher: Dispatcher = {
	useMemo = function<U...>(create: () -> U...): U...
		return create()
	end
}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_type_pack_unification_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:955:type_infer_generics_generic_type_pack_unification_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_type_pack_unification_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_pack_unification_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type Dispatcher = {
	useMemo: <T...>(create: () -> T...) -> T...
}

local TheDispatcher: Dispatcher = {
	useMemo = function(create)
		return create()
	end
}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_type_pack_unification_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:973:type_infer_generics_generic_type_pack_unification_3`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_type_pack_unification_3

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_pack_unification_3() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
type Dispatcher = {
	useMemo: <S,T...>(arg: S, create: (S) -> T...) -> T...
}

local TheDispatcher: Dispatcher = {
	useMemo = function<T,U...>(arg: T, create: (T) -> U...): U...
		return create(arg)
	end
}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1836:type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
export type t1<T...> = {
    foo: (self: t1<T...>, bar: (T...) -> ()) -> ()
}

export type t2<T...> = {
    baz: (self: t2<T...>) -> t1<T...>,
}

export type t3<T...> = {
    f: (self: t3<T...>, T...)->  (),
    g: t1<T...>,
    h: t1<(Player, T...)>
}

local t2 = {}

function t2.new<T...>(): t2<T...>
end

local function create_t3<T...>(): t3<T...>
    local t2_1 = t2.new()
    local t2_2 = t2.new()
    local my_t3 = {
        f = function(_self: t3<T...>, ...: T...) end,
        g = t2_1:baz(),
        h = t2_2:baz()
    }
    return my_t3
end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1809:type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings

  #[cfg(test)]
  #[test]
  fn type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Dispatch<A> = (A) -> ()
type BasicStateAction<S> = ((S) -> S) | S

function updateReducer<S, I, A>(reducer: (S, A) -> S, initialArg: I, init: ((I) -> S)?): (S, Dispatch<A>)
    return 1 :: any, 2 :: any
end

function basicStateReducer<S>(state: S, action: BasicStateAction<S>): S
    return action
end

function updateState<S>(initialState: (() -> S) | S): (S, Dispatch<BasicStateAction<S>>)
    return updateReducer(basicStateReducer, initialState)
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_gh_1985_array_of_union_for_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2047:type_infer_generics_gh_1985_array_of_union_for_generic`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_gh_1985_array_of_union_for_generic

  #[cfg(test)]
  #[test]
  fn type_infer_generics_gh_1985_array_of_union_for_generic() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function clear<T>(arr: { T }) table.clear(arr) end
        local a: { true | false }
        -- This obviously shouldn't error, '{ true | false }' should fit '{ T }'
        -- TypeError: The generic type parameter Twas found to have invalid bounds. Its lower bounds were [true, false], and its upper bounds were [true].
        clear(a)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_gh_1985_array_of_union_for_generic_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2060:type_infer_generics_gh_1985_array_of_union_for_generic_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_gh_1985_array_of_union_for_generic_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_gh_1985_array_of_union_for_generic_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<T>(arr: { T }): { T } return arr end
        local a: { true | false }
        local b = id(a)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_higher_rank_polymorphism_should_not_accept_instantiated_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1709:type_infer_generics_higher_rank_polymorphism_should_not_accept_instantiated_arguments`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_higher_rank_polymorphism_should_not_accept_instantiated_arguments

  #[cfg(test)]
  #[test]
  fn type_infer_generics_higher_rank_polymorphism_should_not_accept_instantiated_arguments() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
    let _instantiate_in_subtyping = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict

local function instantiate(f: <a>(a) -> a): (number) -> number
    return f
end

instantiate(function(x: string) return "foo" end)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("<a>(a) -> a", to_string_type_id(tm.wanted_type));
    assert_eq!("<a>(string) -> string", to_string_type_id(tm.given_type));
  }
}

mod type_infer_generics_hof_subtype_instantiation_regression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1691:type_infer_generics_hof_subtype_instantiation_regression`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_hof_subtype_instantiation_regression

  #[cfg(test)]
  #[test]
  fn type_infer_generics_hof_subtype_instantiation_regression() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict

local function defaultSort<T>(a: T, b: T)
    return true
end
type A = any
return function<T>(array: {T}): {T}
    table.sort(array, defaultSort)
    return array
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_id_function_do_not_leak_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2143:type_infer_generics_id_function_do_not_leak_generic`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_id_function_do_not_leak_generic

  #[cfg(test)]
  #[test]
  fn type_infer_generics_id_function_do_not_leak_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<T>(t: T) return t end
        local function foo(x)
            id(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(unknown) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_generics_infer_generic_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:317:type_infer_generics_infer_generic_function`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method PathBuilder::rets (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_function

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id(x)
            return x
        end
        local x: string = id("hi")
        local y: number = id(37)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let id_type = fixture.require_type_string(&String::from("id"));
    let id_fun = get_type_id::<FunctionType>(id_type).expect("expected FunctionType");
    let (args, _) = flatten_type_pack_id(id_fun.arg_types());
    let (rets, _) = flatten_type_pack_id(id_fun.ret_types());

    assert_eq!(1, id_fun.generics().len());
    assert_eq!(0, id_fun.generic_packs().len());
    assert_eq!(
      follow_type_id(args[0]),
      follow_type_id(id_fun.generics()[0])
    );
    assert_eq!(
      follow_type_id(rets[0]),
      follow_type_id(id_fun.generics()[0])
    );
  }
}

mod type_infer_generics_infer_generic_function_function_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1389:type_infer_generics_infer_generic_function_function_argument`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_argument

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_function_function_argument() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if !FFlag::DebugLuauForceOldSolver.get() {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local function sum<a>(x: a, y: a, f: (a, a) -> add<a>)
                return f(x, y)
            end
            return sum(2, 3, function<T>(a: T, b: T): add<T> return a + b end)
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local function sum<a>(x: a, y: a, f: (a, a) -> a)
                return f(x, y)
            end
            return sum(2, 3, function(a, b) return a + b end)
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_generics_infer_generic_function_function_argument_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1415:type_infer_generics_infer_generic_function_function_argument_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_argument_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_function_function_argument_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function map<a, b>(arr: {a}, f: (a) -> b): {b}
            local r = {}
            for i,v in ipairs(arr) do
                table.insert(r, f(v))
            end
            return r
        end
        local a = {1, 2, 3}
        local r = map(a, function(a: number) return a + a > 100 end)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "{boolean}",
      to_string_type_id(fixture.base.require_type_string(&String::from("r")))
    );
  }
}

mod type_infer_generics_infer_generic_function_function_argument_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1433:type_infer_generics_infer_generic_function_function_argument_3`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_argument_3

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_function_function_argument_3() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foldl<a, b>(arr: {a}, init: b, f: (b, a) -> b)
            local r = init
            for i,v in ipairs(arr) do
                r = f(r, v)
            end
            return r
        end
        local a = {1, 2, 3}
        local r = foldl(a, {s=0,c=0}, function(a: {s: number, c: number}, b: number) return {s = a.s + b, c = a.c + 1} end)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ c: number, s: number } | { c: number, s: number }"
    } else {
      "{| c: number, s: number |}"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.base.require_type_string(&String::from("r")))
    );
  }
}

mod type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1454:type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1

  #[cfg(test)]
  #[test]
  #[ignore = "leaked-generic divergence in new-solver generic inference: var `a` infers to \
`add<unknown, unknown> | number` instead of `number | number` (2 UninhabitedTypeFunction errors \
vs upstream's 1). Both call sites select the correct overload (verified) and store a resolved \
overload whose generic T = `add<t1,t1> | number`; upstream collapses the a-site's inner generic \
`t1` to `number` (so `add<number,number>` reduces away) while this port seals `t1` to `unknown`. \
The leaf functions on the path are faithful 1:1 ports: numericBinopTypeFunction \
(Analysis/src/BuiltinTypeFunctions.cpp:390), generalizeType (Analysis/src/Generalization.cpp:730), \
TypeRemover::process (Generalization.cpp:669, only descends Union/Intersection — not type \
functions, matching C++), the ReduceConstraint force path (ConstraintSolver.cpp:2879) and the \
call-dispatch hasBound/instantiate2 logic (ConstraintSolver.cpp:1801). The divergence is emergent \
from generic-instantiation + generalization + type-function-reduction scheduling, not a single \
mistranslated function; fixing it needs core new-solver generalization changes (out of scope, \
high regression risk against the 2222-test baseline)."]
  fn type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local g12: (<T>(T, (T) -> T) -> T) & (<T>(T, T, (T, T) -> T) -> T)

        local a = g12(1, function(x) return x + x end)
        local b = g12(1, 2, function(x, y) return x + y end)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "number | number",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "add<unknown, unknown> | number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
      );
    }
  }
}

mod type_infer_generics_infer_generic_function_function_overloaded_pt_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1478:type_infer_generics_infer_generic_function_function_overloaded_pt_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_overloaded_pt_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_function_function_overloaded_pt_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local g12: (<T>(T, (T) -> T) -> T) & (<T>(T, T, (T, T) -> T) -> T)

        local a = g12({x=1}, function(x) return {x=-x.x} end)
        local b = g12({x=1}, {x=2}, function(x, y) return {x=x.x + y.x} end)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "{ x: number } | { x: unm<unknown> }",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "{ x: add<unknown, unknown> } | { x: number } | { x: number }",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "{| x: number |}",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
      assert_eq!(
        "{| x: number |}",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
      );
    }
  }
}

mod type_infer_generics_infer_generic_local_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:340:type_infer_generics_infer_generic_local_function`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method PathBuilder::rets (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_generics_infer_generic_local_function

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_local_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id(x)
            return x
        end
        local x: string = id("hi")
        local y: number = id(37)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let id_type = fixture.require_type_string(&String::from("id"));
    let id_fun = get_type_id::<FunctionType>(id_type).expect("expected FunctionType");
    let (args, _) = flatten_type_pack_id(id_fun.arg_types());
    let (rets, _) = flatten_type_pack_id(id_fun.ret_types());

    assert_eq!(1, id_fun.generics().len());
    assert_eq!(0, id_fun.generic_packs().len());
    assert_eq!(
      follow_type_id(args[0]),
      follow_type_id(id_fun.generics()[0])
    );
    assert_eq!(
      follow_type_id(rets[0]),
      follow_type_id(id_fun.generics()[0])
    );
  }
}

mod type_infer_generics_infer_generic_property {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_generic_property() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t.m = function(x) return x end
        local x: string = t.m("hi")
        local y: number = t.m(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_infer_nested_generic_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:363:type_infer_generics_infer_nested_generic_function`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_infer_nested_generic_function

  #[cfg(test)]
  #[test]
  fn type_infer_generics_infer_nested_generic_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f()
            local function id(x)
                return x
            end
            local x: string = id("hi")
            local y: number = id(37)
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_inferred_local_vars_can_be_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_inferred_local_vars_can_be_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id(x) return x end
        print("This is bogus") -- TODO: CLI-39916
        local f = id
        local x: string = f("hi")
        local y: number = f(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_instantiate_cyclic_generic_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1252:type_infer_generics_instantiate_cyclic_generic_function`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_generics_instantiate_cyclic_generic_function

  #[cfg(test)]
  #[test]
  fn type_infer_generics_instantiate_cyclic_generic_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(o)
            o:method()
        end

        function g(o)
            f(o)
        end
    "#,
      ),
      None,
    );

    let g = fixture.require_type_string(&String::from("g"));
    let g_fun = get_type_id::<FunctionType>(g).expect("expected FunctionType");

    let arg = first(g_fun.arg_types(), false).expect("expected argument type");
    let arg = follow_type_id(arg);
    let arg_table = get_type_id::<TableType>(arg)
      .unwrap_or_else(|| panic!("expected table but got {}", to_string_type_id(arg)));

    let method_prop = arg_table
      .props
      .get(&String::from("method"))
      .expect("expected method property");
    let method_ty = method_prop.read_ty.expect("expected readable method type");
    let method_ty = follow_type_id(method_ty);
    let method_function =
      get_type_id::<FunctionType>(method_ty).expect("expected method FunctionType");

    let method_arg = first(method_function.arg_types(), false).expect("expected method argument");
    assert_eq!(follow_type_id(method_arg), follow_type_id(arg));
  }
}

mod type_infer_generics_instantiate_generic_function_in_assignments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1288:type_infer_generics_instantiate_generic_function_in_assignments`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_generics_instantiate_generic_function_in_assignments

  #[cfg(test)]
  #[test]
  fn type_infer_generics_instantiate_generic_function_in_assignments() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(a, b)
            return a(b)
        end

        function bar()
            local c: ((number)->number, number)->number = foo -- no error
            c = foo -- no error
            local d: ((number)->number, string)->number = foo -- error from arg 2 (string) not being convertible to number from the call a(b)
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      "((number) -> number, string) -> number",
      to_string_type_id(tm.wanted_type)
    );

    let expected_given =
      if FFlag::LuauInstantiateInSubtyping.get() || !FFlag::DebugLuauForceOldSolver.get() {
        "<a, b...>((a) -> (b...), a) -> (b...)"
      } else {
        "((number) -> number, number) -> number"
      };
    assert_eq!(expected_given, to_string_type_id(tm.given_type));
  }
}

mod type_infer_generics_instantiate_generic_function_in_assignments_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1317:type_infer_generics_instantiate_generic_function_in_assignments_2`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_generics_instantiate_generic_function_in_assignments_2

  #[cfg(test)]
  #[test]
  fn type_infer_generics_instantiate_generic_function_in_assignments_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(a, b)
            return a(b)
        end

        function bar()
            local _: (string, string)->number = foo -- string cannot be converted to (string)->number
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      "(string, string) -> number",
      to_string_type_id(tm.wanted_type)
    );

    let expected_given =
      if FFlag::LuauInstantiateInSubtyping.get() || !FFlag::DebugLuauForceOldSolver.get() {
        "<a, b...>((a) -> (b...), a) -> (b...)"
      } else {
        "((string) -> number, string) -> number"
      };
    assert_eq!(expected_given, to_string_type_id(tm.given_type));
  }
}

mod type_infer_generics_instantiated_function_argument_names_old_solver {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_instantiated_function_argument_names_old_solver() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<T, U...>(a: T, ...: U...) end

        f(1, 2, 3)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture
      .find_type_at_position_position(Position { line: 3, column: 8 })
      .expect("expected type at position");
    let mut opts = ToStringOptions {
      function_type_arguments: true,
      ..Default::default()
    };
    assert_eq!(
      "(a: number, number, number) -> ()",
      to_string_type_id_to_string_options(ty, &mut opts)
    );
  }
}

mod type_infer_generics_instantiation_sharing_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_instantiation_sharing_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(z)
          local o = {}
          o.x = o
          o.y = {5}
          o.z = z
          return o
        end
        local o1 = f(true)
        local x1, y1, z1 = o1.x, o1.y, o1.z
        local o2 = f("hi")
        local x2, y2, z2 = o2.x, o2.y, o2.z
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let x1 = fixture.require_type_string(&String::from("x1"));
    let x2 = fixture.require_type_string(&String::from("x2"));
    let y1 = fixture.require_type_string(&String::from("y1"));
    let y2 = fixture.require_type_string(&String::from("y2"));
    let z1 = fixture.require_type_string(&String::from("z1"));
    let z2 = fixture.require_type_string(&String::from("z2"));

    assert_ne!(x1, x2);
    assert_eq!(y1, y2);
    assert_ne!(z1, z2);
  }
}

mod type_infer_generics_local_vars_can_be_instantiated_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_local_vars_can_be_instantiated_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id(x) return x end
        print("This is bogus") -- TODO: CLI-39916
        local f: (number)->number = id
        local g: (string)->string = id
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_local_vars_can_be_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_local_vars_can_be_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<a>(x:a):a return x end
        local f: <a>(a)->a = id
        local x: string = f("hi")
        local y: number = f(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_missing_generic_type_parameter {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1766:type_infer_generics_missing_generic_type_parameter`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_missing_generic_type_parameter

  #[cfg(test)]
  #[test]
  fn type_infer_generics_missing_generic_type_parameter() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: T): T return x end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    type_error_data_ref::<UnknownSymbol>(&result.errors[1]).expect("expected UnknownSymbol");
  }
}

mod type_infer_generics_mutable_state_polymorphism {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_mutable_state_polymorphism() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        -- Our old friend the polymorphic identity function
        local function id(x) return x end
        local a: string = id("hi")
        local b: number = id(37)

        -- This allows <a>(a)->a to be expressed without generic function syntax
        type Id = typeof(id)

        -- This function should have type
        -- <a>() -> (a) -> a
        -- not type
        -- () -> <a>(a) -> a
        local function ohDear(): Id
          local y
          function oh(x)
            -- Returns the same x every time it's called
            if not(y) then y = x end
            return y
          end
          return oh
        end

        -- oh dear, f claims to polymorphic which it shouldn't be
        local f: Id = ohDear()

        -- the first call sets y
        local a: string = f("not a number")
        -- so b has value "not a number" at run time
        local b: number = f(37)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_generics_nested_generic_argument_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1103:type_infer_generics_nested_generic_argument_type_packs`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_nested_generic_argument_type_packs

  #[cfg(test)]
  #[test]
  fn type_infer_generics_nested_generic_argument_type_packs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{count_mismatch::CountMismatch, type_mismatch::TypeMismatch},
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function test2(a: number)
    return 3
end

function foo<B...>(f: (B...) -> number, ...: B...)
    return f(...)
end

-- want A... to contain a generic type pack too

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

-- A... = ((B...) -> number, B...))
-- B... = (number)
-- A... = ((number) -> number, number)
wrapper(foo, test2, 3) -- ok
wrapper(foo, test2, 3, 3) -- not ok (too many args)
wrapper(foo, test2) -- not ok (not enough args)
wrapper(foo, test2, "3") -- not ok (type mismatch, string instead of number)
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        Location {
          begin: Position {
            line: 18,
            column: 0
          },
          end: Position {
            line: 18,
            column: 7
          },
        },
        result.errors[0].location
      );
      let cm =
        type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
      assert_eq!(3, cm.expected());
      assert_eq!(4, cm.actual());
      assert_eq!(CountMismatch::ARG, cm.context());

      assert_eq!(
        Location {
          begin: Position {
            line: 19,
            column: 0
          },
          end: Position {
            line: 19,
            column: 7
          },
        },
        result.errors[1].location
      );
      let cm =
        type_error_data_ref::<CountMismatch>(&result.errors[1]).expect("expected CountMismatch");
      assert_eq!(3, cm.expected());
      assert_eq!(2, cm.actual());
      assert_eq!(CountMismatch::ARG, cm.context());

      assert_eq!(
        Location {
          begin: Position {
            line: 20,
            column: 20,
          },
          end: Position {
            line: 20,
            column: 23,
          },
        },
        result.errors[2].location
      );
      let tm =
        type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
      assert_eq!("number", to_string_type_id(tm.wanted_type));
      assert_eq!("string", to_string_type_id(tm.given_type));
    } else {
      assert_eq!(
        "Argument count mismatch. Function 'wrapper' expects 3 arguments, but 4 are specified",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Argument count mismatch. Function 'wrapper' expects 3 arguments, but only 2 are specified",
        to_string_type_error(&result.errors[1])
      );
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[2])
      );
    }
  }
}

mod type_infer_generics_nested_generic_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1989:type_infer_generics_nested_generic_packs`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_nested_generic_packs

  #[cfg(test)]
  #[test]
  fn type_infer_generics_nested_generic_packs() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type T = <A...>(A...) -> (<A...>(A...) -> ())
type U = (string) -> ((number) -> ())
local t: T
local u: U = t
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_no_extra_quantification_for_generic_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1663:type_infer_generics_no_extra_quantification_for_generic_functions`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_no_extra_quantification_for_generic_functions

  #[cfg(test)]
  #[test]
  fn type_infer_generics_no_extra_quantification_for_generic_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo<X, Y>(f : (X) -> Y, x: X)
            return f(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<X, Y>((X) -> Y, X) -> Y",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_generics_no_stack_overflow_from_quantifying {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1360:type_infer_generics_no_stack_overflow_from_quantifying`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record OccursCheckFailed (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_generics_no_stack_overflow_from_quantifying

  #[cfg(test)]
  #[test]
  fn type_infer_generics_no_stack_overflow_from_quantifying() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::occurs_check_failed::OccursCheckFailed,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function _(l0:t0): (any, ()->())
        end

        type t0 = t0 | {}
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let t0 = fixture
      .lookup_type(&String::from("t0"))
      .expect("expected t0 type");
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "any"
    } else {
      "*error-type*"
    };
    assert_eq!(expected, to_string_type_id(t0));

    assert!(
      result
        .errors
        .iter()
        .any(|err| type_error_data_ref::<OccursCheckFailed>(err).is_some()),
      "expected OccursCheckFailed in {:?}",
      result.errors
    );
  }
}

mod type_infer_generics_oss_2075_generic_packs_should_not_be_dropped {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2104:type_infer_generics_oss_2075_generic_packs_should_not_be_dropped`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_oss_2075_generic_packs_should_not_be_dropped

  #[cfg(test)]
  #[test]
  fn type_infer_generics_oss_2075_generic_packs_should_not_be_dropped() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<Return...>(callback: () -> Return...) end

        f(function()
            return 3
        end)

        local function g<Rest...>(callback: (x: string, Rest...) -> any) end
        g(error)

        type X<T...> = {
            value: () -> T...,
        }

        local function foo<T...>(x: X<T...>) end

        local function bar(x: X<string, number>)
            foo(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_properties_can_be_instantiated_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_properties_can_be_instantiated_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: { m: (number)->number } = { m = function(x:number) return x+1 end }
        local function id<a>(x:a):a return x end
        t.m = id
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_properties_can_be_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_properties_can_be_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}
        t.m = function<a>(x: a):a return x end
        local x: string = t.m("hi")
        local y: number = t.m(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_quantification_sharing_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_quantification_sharing_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x) return {5} end
        function g(x, y) return f(x) end
        local z1 = f(5)
        local z2 = g(true, "hi")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let z1 = fixture.require_type_string(&String::from("z1"));
    let z2 = fixture.require_type_string(&String::from("z2"));
    assert_eq!(z1, z2);
  }
}

mod type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1652:type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic

  #[cfg(test)]
  #[test]
  fn type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo<X>(f, x: X)
            return f(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<X, a...>((X) -> (a...), X) -> (a...)",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_generics_quantify_functions_with_no_generics {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1641:type_infer_generics_quantify_functions_with_no_generics`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_generics_quantify_functions_with_no_generics

  #[cfg(test)]
  #[test]
  fn type_infer_generics_quantify_functions_with_no_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(f, x)
            return f(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<a, b...>((a) -> (b...), a) -> (b...)",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_generics_rank_n_types_via_typeof {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_rank_n_types_via_typeof() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function id(x) return x end
        local x: string = id("hi")
        local y: number = id(37)
        -- This allows <a>(a)->a to be expressed without generic function syntax
        type Id = typeof(id)
        -- The rank 1 restriction causes this not to typecheck, since it's
        -- declared as returning a polytype.
        local function returnsId(): Id
          return id
        end
        -- So this won't typecheck
        local f: Id = returnsId()
        local a: string = f("hi")
        local b: number = f(37)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_reject_clashing_generic_and_pack_names {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_reject_clashing_generic_and_pack_names() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_generic_parameter::DuplicateGenericParameter;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a, a...>() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<DuplicateGenericParameter>(&result.errors[0])
      .expect("expected DuplicateGenericParameter");
    assert_eq!("a", err.parameter_name());
  }
}

mod type_infer_generics_self_recursive_instantiated_param {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1344:type_infer_generics_self_recursive_instantiated_param`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_self_recursive_instantiated_param

  #[cfg(test)]
  #[test]
  fn type_infer_generics_self_recursive_instantiated_param() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Table = { a: number }
type Self<T> = T
local a: Self<Table>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Table<Table>"
    } else {
      "Table"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_generics_substitution_with_bound_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1558:type_infer_generics_substitution_with_bound_table`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_generics_substitution_with_bound_table

  #[cfg(test)]
  #[test]
  fn type_infer_generics_substitution_with_bound_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: number }
        local a: A = { x = 1 }
        local b = a
        type B = typeof(b)
        type X<T> = T
        local c: X<B>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_table_isfrozen_and_clear_work_on_any_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2071:type_infer_generics_table_isfrozen_and_clear_work_on_any_table`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_generics_table_isfrozen_and_clear_work_on_any_table

  #[cfg(test)]
  #[test]
  fn type_infer_generics_table_isfrozen_and_clear_work_on_any_table() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Array<T> = { [number]: T }
        type Object = { [string]: any }

        return function(t: Object | Array<any>)
            if not table.isfrozen(t) then
                table.clear(t)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_type_parameters_can_be_polytypes {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_type_parameters_can_be_polytypes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id<a>(x:a):a return x end
        local f: <a>(a)->a = id(id)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_typefuns_sharing_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_typefuns_sharing_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<a> = { x: {a}, y: {number} }
        local o1: T<boolean> = { x = {true}, y = {5} }
        local x1, y1 = o1.x, o1.y
        local o2: T<string> = { x = {"hi"}, y = {37} }
        local x2, y2 = o2.x, o2.y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let x1 = fixture.require_type_string(&String::from("x1"));
    let x2 = fixture.require_type_string(&String::from("x2"));
    let y1 = fixture.require_type_string(&String::from("y1"));
    let y2 = fixture.require_type_string(&String::from("y2"));

    assert_ne!(x1, x2);
    assert_eq!(y1, y2);
  }
}

mod type_infer_generics_typepacks_before_types {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_typepacks_before_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,b>() end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_types_before_typepacks {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_types_before_typepacks() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a,b...>() end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_unions_and_generics {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_unions_and_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type foo = <T>(T | {T}) -> T
        local foo = (nil :: any) :: foo

        type Test = number | {number}
        local res = foo(1 :: Test)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("res")))
      );
    } else {
      assert_eq!(
        "'a",
        to_string_type_id(fixture.require_type_string(&String::from("res")))
      );
    }
  }
}

mod type_infer_generics_variadic_generics {
  #[cfg(test)]
  #[test]
  fn type_infer_generics_variadic_generics() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a>(...: a) end

        type F<a> = (...a) -> ...a
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_generics_variadic_generics_dont_leak {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2128:type_infer_generics_variadic_generics_dont_leak`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_generics_variadic_generics_dont_leak

  #[cfg(test)]
  #[test]
  fn type_infer_generics_variadic_generics_dont_leak() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function makeApplier<A..., R...>(f: (A...) -> (R...))
            return function (... : A...): R...
                f(...)
            end
        end
        local function add(x: number, y: number): number return x + y end
        local f = makeApplier(add)
    "#,
      ),
      None,
    );

    assert_eq!(
      "(number, number) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_generics_xpcall_should_work_with_generics {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2033:type_infer_generics_xpcall_should_work_with_generics`
  //! Source: `tests/TypeInfer.generics.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.generics.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.generics.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_generics_xpcall_should_work_with_generics

  #[cfg(test)]
  #[test]
  fn type_infer_generics_xpcall_should_work_with_generics() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local v: (number) -> (number) = nil :: any

local x = 3

xpcall(v, print, x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
