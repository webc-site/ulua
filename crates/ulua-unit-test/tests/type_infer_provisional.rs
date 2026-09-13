extern crate alloc;

mod type_infer_provisional_assert_and_many_nested_typeof_contexts {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1402:type_infer_provisional_assert_and_many_nested_typeof_contexts`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_assert_and_many_nested_typeof_contexts

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_assert_and_many_nested_typeof_contexts() {
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
        local foo: unknown = nil :: any
        assert(typeof(foo) == "table")
        if typeof(typeof(foo.x)) == "string" then
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_assign_table_with_refined_property_with_a_similar_type_is_illegal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:813:type_infer_provisional_assign_table_with_refined_property_with_a_similar_type_is_illegal`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_assign_table_with_refined_property_with_a_similar_type_is_illegal

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_assign_table_with_refined_property_with_a_similar_type_is_illegal() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {x: number?} = {x = nil}

        if t.x then
            local u: {x: number} = t
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = "Expected this to be exactly\n\t'{ x: number }'\nbut got\n\t'{ x: number? }'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'number', but got 'number?'";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_provisional_bail_early_if_unification_is_too_complicated {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:315:type_infer_provisional_bail_early_if_unification_is_too_complicated`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnificationTooComplex (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_bail_early_if_unification_is_too_complicated

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_bail_early_if_unification_is_too_complicated() {
    use alloc::string::String;

    use ulua_analysis::records::unification_too_complex::UnificationTooComplex;
    use ulua_common::FInt;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _tarjan_child_limit = ScopedFastInt::new(&FInt::LuauTarjanChildLimit, 1);
    let _iteration_limit = ScopedFastInt::new(&FInt::LuauTypeInferIterationLimit, 1);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Result
        Result = setmetatable({}, {})
        Result.__index = Result
        function Result.new(okValue)
            local self = setmetatable({}, Result)
            self:constructor(okValue)
            return self
        end
        function Result:constructor(okValue)
            self.okValue = okValue
        end
        function Result:ok(val) return Result.new(val) end
        function Result:a(p0, p1, p2, p3, p4) return Result.new((self.okValue)) or p0 or p1 or p2 or p3 or p4 end
        function Result:b(p0, p1, p2, p3, p4) return Result:ok((self.okValue)) or p0 or p1 or p2 or p3 or p4 end
        function Result:c(p0, p1, p2, p3, p4) return Result:ok((self.okValue)) or p0 or p1 or p2 or p3 or p4 end
        function Result:transpose(a)
            return a and self.okValue:z(function(some)
                return Result:ok(some)
            end) or Result:ok(self.okValue)
        end
    "#,
        ),
        None,
    );

    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<UnificationTooComplex>(error).is_some()),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_provisional_bidirectional_inference_variadic_type_pack_read_only_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1417:type_infer_provisional_bidirectional_inference_variadic_type_pack_read_only_prop`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_bidirectional_inference_variadic_type_pack_read_only_prop

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_bidirectional_inference_variadic_type_pack_read_only_prop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        local foo: { read bar: (...string) -> () } = {
            bar = function (foobar)
                print(foobar)
            end
        }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 24))
      )
    );
  }
}

mod type_infer_provisional_bin_prov {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1214:type_infer_provisional_bin_prov`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_bin_prov

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_bin_prov() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Bin = {}

        function Bin:add(item)
            self.head = { item = item}
            return item
        end

        function Bin:destroy()
            while self.head do
                local item = self.head.item
                if type(item) == "function" then
                    item()
                elseif item.Destroy ~= nil then
                end
                self.head = self.head.next
            end
        end
    "#,
      ),
      None,
    );
  }
}

mod type_infer_provisional_choose_the_right_overload_for_pcall {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:431:type_infer_provisional_choose_the_right_overload_for_pcall`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_choose_the_right_overload_for_pcall

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_choose_the_right_overload_for_pcall() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): number
            if math.random() > 0.5 then
                return 5
            else
                error("something")
            end
        end

        local ok, res = pcall(f)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("ok")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("res")))
    );
  }
}

mod type_infer_provisional_cli_181248_intersection_of_indexers_should_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1608:type_infer_provisional_cli_181248_intersection_of_indexers_should_error`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_cli_181248_intersection_of_indexers_should_error

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_cli_181248_intersection_of_indexers_should_error() {
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
        local tbl: { good: boolean } & { bad: boolean }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_provisional_cli_181248_union_of_indexers_should_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1623:type_infer_provisional_cli_181248_union_of_indexers_should_error`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_cli_181248_union_of_indexers_should_error

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_cli_181248_union_of_indexers_should_error() {
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
        local tbl: { good: boolean } | { bad: boolean }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_provisional_cli_181248_union_of_indexers_with_one_good_option_should_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1638:type_infer_provisional_cli_181248_union_of_indexers_with_one_good_option_should_error`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_cli_181248_union_of_indexers_with_one_good_option_should_error

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_cli_181248_union_of_indexers_with_one_good_option_should_error() {
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
        local tbl: { good: boolean } | { [string]: string }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "*error-type* | string",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_provisional_cli_181248_unreduced_intersection_of_indexers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1654:type_infer_provisional_cli_181248_unreduced_intersection_of_indexers`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_cli_181248_unreduced_intersection_of_indexers

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_cli_181248_unreduced_intersection_of_indexers() {
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
        local tbl: { [string]: string | number } & { [string]: string | boolean }
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(boolean | string) & (number | string)",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_provisional_cli_181248_unreduced_union_of_indexers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1668:type_infer_provisional_cli_181248_unreduced_union_of_indexers`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_cli_181248_unreduced_union_of_indexers

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_cli_181248_unreduced_union_of_indexers() {
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
        local tbl: { [string]: "hi" } | { [string]: string}
        local key: string
        local val = tbl[key]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "\"hi\" | string",
      to_string_type_id(fixture.base.require_type_string(&String::from("val")))
    );
  }
}

mod type_infer_provisional_dcr_can_partially_dispatch_a_constraint {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:485:type_infer_provisional_dcr_can_partially_dispatch_a_constraint`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_dcr_can_partially_dispatch_a_constraint

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_dcr_can_partially_dispatch_a_constraint() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function hasDivisors(value: number)
        end

        function prime_iter(state, index)
            hasDivisors(index)
            index += 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(unknown, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("prime_iter")))
      );
    } else {
      assert_eq!(
        "<a>(a, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("prime_iter")))
      );
    }
  }
}

mod type_infer_provisional_discriminate_from_x_not_equal_to_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:285:type_infer_provisional_discriminate_from_x_not_equal_to_nil`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_discriminate_from_x_not_equal_to_nil

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_discriminate_from_x_not_equal_to_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = {x: string, y: number} | {x: nil, y: nil}

        local function f(t: T)
            if t.x ~= nil then
                local foo = t
            else
                local bar = t
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ x: string, y: number }",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
      assert_eq!(
        "{ x: nil, y: nil }",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
      );
    } else {
      assert_eq!(
        "{ x: string, y: number }",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
      );
      assert_eq!(
        "{ x: nil, y: nil } | { x: string, y: number }",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
      );
    }
  }
}

mod type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:362:type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - translates_to -> rust_item type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f() end

        local g = function() return f() end

        local x = (f()) -- should error: no return values to assign from the call to f
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "() -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
      assert_eq!(
        "() -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
      );
      assert_eq!(
        "nil",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
      );
    } else {
      assert_eq!(
        "() -> (a...)",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
      );
      assert_eq!(
        "<a...>() -> (a...)",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
      );
      assert_eq!(
        "any",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
      );
    }
  }
}

mod type_infer_provisional_error_on_eq_metamethod_returning_a_type_other_than_boolean {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:239:type_infer_provisional_error_on_eq_metamethod_returning_a_type_other_than_boolean`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_error_on_eq_metamethod_returning_a_type_other_than_boolean

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_error_on_eq_metamethod_returning_a_type_other_than_boolean() {
    use alloc::string::String;

    use ulua_analysis::records::generic_error::GenericError;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local tab = {a = 1}
        setmetatable(tab, {__eq = function(a, b): number
            return 1
        end})
        local tab2 = tab

        local a = tab2 == tab
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!("Metamethod '__eq' must return type 'boolean'", ge.message());
  }
}

mod type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:887:type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ref<T> = { val: T }

        local function useRef<T>(x: T): Ref<T?>
            return { val = x }
        end

        local x: Ref<number?> = useRef(nil)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_provisional_floating_generics_should_not_be_allowed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:914:type_infer_provisional_floating_generics_should_not_be_allowed`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_provisional_floating_generics_should_not_be_allowed

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_floating_generics_should_not_be_allowed() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local assign : <T, U, V, W>(target: T, source0: U?, source1: V?, source2: W?, ...any) -> T & U & V & W = (nil :: any)

        -- We have a big problem here: The generics U, V, and W are not bound to anything!
        -- Things get strange because of this.
        local benchmark = assign({})
        local options = benchmark.options
        do
            local resolve2: any = nil
            options.fn({
                resolve = function(...)
                    resolve2(...)
                end,
            })
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_for_in_loop_with_zero_iterators {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:559:type_infer_provisional_for_in_loop_with_zero_iterators`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_for_in_loop_with_zero_iterators

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_for_in_loop_with_zero_iterators() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function no_iter() end
        for key in no_iter() do end -- This should not be ok
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_free_is_not_bound_to_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:474:type_infer_provisional_free_is_not_bound_to_any`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_free_is_not_bound_to_any

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_free_is_not_bound_to_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(f: (any) -> (), x)
            f(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "((any) -> (), any) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_provisional_free_options_can_be_unified_together {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:938:type_infer_provisional_free_options_can_be_unified_together`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
  //!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Unifier (Analysis/include/Luau/Unifier.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> enum Variance (Analysis/include/Luau/Unifier.h)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_provisional_free_options_can_be_unified_together

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_free_options_can_be_unified_together() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{to_string_options::ToStringOptions, union_type::UnionType},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::try_unify_fixture::TryUnifyFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = TryUnifyFixture::new();
    let nil_type = fixture.get_builtins().nil_type;

    let free1 = fixture.fresh_type();
    let option1 = fixture.arena.add_type(UnionType {
      options: vec![nil_type, free1],
    });

    let free2 = fixture.fresh_type();
    let option2 = fixture.arena.add_type(UnionType {
      options: vec![nil_type, free2],
    });

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(option1, option2, false, false, None);
    assert!(!fixture.state.failure, "{:?}", fixture.state.errors);

    fixture.state.log.commit();

    // C++ declares a single `ToStringOptions opts;` and reuses it across both
    // to_string calls, so the shared nameMap mints 'a for option1 and 'b for option2.
    let mut opts = ToStringOptions::default();
    assert_eq!(
      "'a?",
      to_string_type_id_to_string_options(option1, &mut opts)
    );
    assert_eq!(
      "'b?",
      to_string_type_id_to_string_options(option2, &mut opts)
    );
  }
}

mod type_infer_provisional_free_options_cannot_be_unified_together {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:526:type_infer_provisional_free_options_cannot_be_unified_together`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
  //!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Unifier (Analysis/include/Luau/Unifier.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> enum Variance (Analysis/include/Luau/Unifier.h)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_provisional_free_options_cannot_be_unified_together

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_free_options_cannot_be_unified_together() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{to_string_options::ToStringOptions, union_type::UnionType},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::try_unify_fixture::TryUnifyFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = TryUnifyFixture::new();
    let nil_type = fixture.get_builtins().nil_type;

    let free1 = fixture.fresh_type();
    let option1 = fixture.arena.add_type(UnionType {
      options: vec![nil_type, free1],
    });

    let free2 = fixture.fresh_type();
    let option2 = fixture.arena.add_type(UnionType {
      options: vec![nil_type, free2],
    });

    fixture
      .state
      .try_unify_type_id_type_id_bool_bool_literal_properties(option1, option2, false, false, None);
    assert!(!fixture.state.failure, "{:?}", fixture.state.errors);

    fixture.state.log.commit();

    // C++ declares a single `ToStringOptions opts;` and reuses it across both
    // to_string calls, so the shared nameMap mints 'a for option1 and 'b for option2.
    let mut opts = ToStringOptions::default();
    assert_eq!(
      "'a?",
      to_string_type_id_to_string_options(option1, &mut opts)
    );
    assert_eq!(
      "'b?",
      to_string_type_id_to_string_options(option2, &mut opts)
    );
  }
}

mod type_infer_provisional_function_indexer_satisfies_reading_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1351:type_infer_provisional_function_indexer_satisfies_reading_property`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_function_indexer_satisfies_reading_property

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_function_indexer_satisfies_reading_property() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = setmetatable({}, {
            __index = function (_, _prop: string): number
                return 42
            end
        })

        local function readX(tbl: { read X: number })
            print(tbl.X)
        end

        -- This should work as `__index` being a function should semantically
        -- be the same as having an indexer.
        readX(t)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
      type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    let mut options = ToStringOptions::new(true);
    assert_eq!(
      "{ @metatable { __index: (unknown, string) -> number }, {  } }",
      to_string_type_id_to_string_options(err.given_type, &mut options)
    );
    assert_eq!("{ read X: number }", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:452:type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (number, string, boolean)
            if math.random() > 0.5 then
                return 5, "hello", true
            else
                error("something")
            end
        end

        local ok, res, s, b = pcall(f)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("ok")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("res")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_provisional_functions_with_mismatching_arity {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:681:type_infer_provisional_functions_with_mismatching_arity`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_provisional_functions_with_mismatching_arity

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_functions_with_mismatching_arity() {
    use alloc::string::String;

    use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

    let mut fixture = IsSubtypeFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: (number) -> ()
        local b: () -> ()

        local c: () -> number
    "#,
      ),
      None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));

    assert!(!fixture.is_subtype(a, b));
    assert!(!fixture.is_subtype(a, c));
    assert!(!fixture.is_subtype(b, c));
  }
}

mod type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:764:type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param() {
    use alloc::string::String;

    use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

    let mut fixture = IsSubtypeFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: (number?) -> ()
        local b: (number) -> ()
        local c: (number, any) -> ()
    "#,
      ),
      None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));

    assert!(!fixture.is_subtype(b, a));
    assert!(!fixture.is_subtype(c, a));
    assert!(fixture.is_subtype(a, b));
  }
}

mod type_infer_provisional_functions_with_mismatching_arity_but_optional_parameters {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:704:type_infer_provisional_functions_with_mismatching_arity_but_optional_parameters`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_provisional_functions_with_mismatching_arity_but_optional_parameters

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_functions_with_mismatching_arity_but_optional_parameters() {
    use alloc::string::String;

    use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

    let mut fixture = IsSubtypeFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: (number?) -> ()
        local b: (number) -> ()
        local c: (number, number?) -> ()
    "#,
      ),
      None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));

    assert!(!fixture.is_subtype(b, a));
    assert!(!fixture.is_subtype(c, a));
    assert!(fixture.is_subtype(a, b));
  }
}

mod type_infer_provisional_generic_type_leak_to_module_interface {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:572:type_infer_provisional_generic_type_leak_to_module_interface`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record AnyType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_provisional_generic_type_leak_to_module_interface

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_generic_type_leak_to_module_interface() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, get_type_alt_j::get_type_id,
        to_string_to_string_alt_d::to_string_type_pack_id,
      },
      records::any_type::AnyType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
local wrapStrictTable

local metatable = {
    __index = function(self, key)
        local value = self.__tbl[key]
        if type(value) == "table" then
            -- unification of the free 'wrapStrictTable' with this function type causes generics of this function to leak out of scope
            return wrapStrictTable(value, self.__name .. "." .. key)
        end
        return value
    end,
}

return wrapStrictTable
    "#,
        ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local wrapStrictTable = require(game.A)

local Constants = {}

return wrapStrictTable(Constants, "Constants")
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("*error-type*", to_string_type_pack_id(module.return_type));
    } else {
      let result = first(module.return_type, true).expect("expected first return type");
      assert!(get_type_id::<AnyType>(result).is_some(), "{:?}", result);
    }
  }
}

mod type_infer_provisional_generic_type_leak_to_module_interface_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:616:type_infer_provisional_generic_type_leak_to_module_interface_variadic`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_generic_type_leak_to_module_interface_variadic

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_generic_type_leak_to_module_interface_variadic() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      first::first, to_string_to_string_alt_c::to_string_type_id,
      to_string_to_string_alt_d::to_string_type_pack_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
local wrapStrictTable

local metatable = {
    __index = function<T>(self, key, ...: T)
        local value = self.__tbl[key]
        if type(value) == "table" then
            -- unification of the free 'wrapStrictTable' with this function type causes generics of this function to leak out of scope
            return wrapStrictTable(value, self.__name .. "." .. key)
        end
        return ...
    end,
}

return wrapStrictTable
    "#,
        ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local wrapStrictTable = require(game.A)

local Constants = {}

return wrapStrictTable(Constants, "Constants")
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!("*error-type*", to_string_type_pack_id(module.return_type));
    } else {
      let result = first(module.return_type, true).expect("expected first return type");
      assert_eq!("any", to_string_type_id(result));
    }
  }
}

mod type_infer_provisional_indexing_union_of_indexers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1435:type_infer_provisional_indexing_union_of_indexers`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_indexing_union_of_indexers

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_indexing_union_of_indexers() {
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
        local function foo(
            t: { [string]: number } | { [number]: number }
        )
            return t[true]
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_intersection_of_functions_of_different_arities {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:660:type_infer_provisional_intersection_of_functions_of_different_arities`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_infer_provisional_intersection_of_functions_of_different_arities

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_intersection_of_functions_of_different_arities() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

    let mut fixture = IsSubtypeFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (any) -> ()
        type B = (any, any) -> ()
        type T = A & B

        local a: A
        local b: B
        local t: T
    "#,
      ),
      None,
    );

    let _a = fixture.base.require_type_string(&String::from("a"));
    let _b = fixture.base.require_type_string(&String::from("b"));

    assert_eq!(
      "((any) -> ()) & ((any, any) -> ())",
      to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_provisional_it_should_be_agnostic_of_actual_size {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:179:type_infer_provisional_it_should_be_agnostic_of_actual_size`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_it_should_be_agnostic_of_actual_size

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_it_should_be_agnostic_of_actual_size() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x, y, ...)
            if not y then return x end
            return f(x, ...)
        end

        f(3, 2, 1, 0)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_lookup_prop_of_intersection_containing_unions_of_tables_that_have_the_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:869:type_infer_provisional_lookup_prop_of_intersection_containing_unions_of_tables_that_have_the_prop`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_provisional_lookup_prop_of_intersection_containing_unions_of_tables_that_have_the_prop

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_lookup_prop_of_intersection_containing_unions_of_tables_that_have_the_prop()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function mergeOptions<T>(options: T & ({variable: string} | {variable: number}))
            return options.variable
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_loop_unsoundness {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1313:type_infer_provisional_loop_unsoundness`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - translates_to -> rust_item type_infer_provisional_loop_unsoundness

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_loop_unsoundness() {
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
        local f = function () return 42 end
        while true do
            f = f()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_luau_polyfill_array_filter {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:96:type_infer_provisional_luau_polyfill_array_filter`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_luau_polyfill_array_filter

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_luau_polyfill_array_filter() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
-- Implements Javascript's `Array.prototype.filter` as defined below
-- https://developer.cmozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
type Array<T> = { [number]: T }
type callbackFn<T> = (element: T, index: number, array: Array<T>) -> boolean
type callbackFnWithThisArg<T, U> = (thisArg: U, element: T, index: number, array: Array<T>) -> boolean
type Object = { [string]: any }
return function<T, U>(t: Array<T>, callback: callbackFn<T> | callbackFnWithThisArg<T, U>, thisArg: U?): Array<T>

	local len = #t
	local res = {}
	if thisArg == nil then
		for i = 1, len do
			local kValue = t[i]
			if kValue ~= nil then
				if (callback :: callbackFn<T>)(kValue, i, t) then
					res[i] = kValue
				end
			end
		end
	else
	end

	return res
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_luau_polyfill_map_entries {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1018:type_infer_provisional_luau_polyfill_map_entries`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_luau_polyfill_map_entries

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_luau_polyfill_map_entries() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("Module/Map"),
      String::from(
        r#"
--!strict

type Object = { [any]: any }
type Array<T> = { [number]: T }
type Table<T, V> = { [T]: V }
type Tuple<T, V> = Array<T | V>

local Map = {}

export type Map<K, V> = {
	size: number,
	-- method definitions
	set: (self: Map<K, V>, K, V) -> Map<K, V>,
	get: (self: Map<K, V>, K) -> V | nil,
	clear: (self: Map<K, V>) -> (),
	delete: (self: Map<K, V>, K) -> boolean,
	has: (self: Map<K, V>, K) -> boolean,
	keys: (self: Map<K, V>) -> Array<K>,
	values: (self: Map<K, V>) -> Array<V>,
	entries: (self: Map<K, V>) -> Array<Tuple<K, V>>,
	ipairs: (self: Map<K, V>) -> any,
	[K]: V,
	_map: { [K]: V },
	_array: { [number]: K },
}

function Map:entries()
	return {}
end

local function coerceToTable(mapLike: Map<any, any> | Table<any, any>): Array<Tuple<any, any>>
    local e = mapLike:entries();
    return e
end

    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/Map"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_luau_roact_use_state_minimization {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1177:type_infer_provisional_luau_roact_use_state_minimization`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_luau_roact_use_state_minimization

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_luau_roact_use_state_minimization() {
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
        type BasicStateAction<S> = ((S) -> S) | S
        type Dispatch<A> = (A) -> ()

        local function useState<S>(
            initialState: (() -> S) | S
        ): (S, Dispatch<BasicStateAction<S>>)
            -- fake impl that obeys types
            local val = if type(initialState) == "function" then initialState() else initialState
            return val, function(value)
                return value
            end
        end

        local test, setTest = useState(nil :: string?)

        setTest(nil) -- this line causes the type to be narrowed in the old solver!!!

        local function update(value: string)
            print(test)
            setTest(value)
        end

        update("hello")
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_luau_roact_use_state_nilable_state_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1138:type_infer_provisional_luau_roact_use_state_nilable_state_1`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_provisional_luau_roact_use_state_nilable_state_1

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_luau_roact_use_state_nilable_state_1() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Dispatch<A> = (A) -> ()
        type BasicStateAction<S> = ((S) -> S) | S

        type ScriptConnection = { Disconnect: (ScriptConnection) -> () }

        local blah = nil :: any

        local function useState<S>(
            initialState: (() -> S) | S,
            ...
        ): (S, Dispatch<BasicStateAction<S>>)
            return blah, blah
        end

        local a, b = useState(nil :: ScriptConnection?)

        if a then
            a:Disconnect()
            b(nil :: ScriptConnection?)
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        Location::new(Position::new(19, 14), Position::new(19, 41)),
        result.errors[0].location
      );
    }
  }
}

mod type_infer_provisional_lvalue_equals_another_lvalue_with_no_overlap {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:262:type_infer_provisional_lvalue_equals_another_lvalue_with_no_overlap`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_lvalue_equals_another_lvalue_with_no_overlap

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_lvalue_equals_another_lvalue_with_no_overlap() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: string, b: boolean?)
            if a == b then
                local foo, bar = a, b
            else
                local foo, bar = a, b
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
    );
    assert_eq!(
      "boolean?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 33)))
    );
    assert_eq!(
      "boolean?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 36)))
    );
  }
}

mod type_infer_provisional_normalization_limit_in_unify_with_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1102:type_infer_provisional_normalization_limit_in_unify_with_any`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function formatAppend (Common/src/StringUtils.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_normalization_limit_in_unify_with_any

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_normalization_limit_in_unify_with_any() {
    use alloc::string::String;

    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _normalize_cache_limit = ScopedFastInt::new(&FInt::LuauNormalizeCacheLimit, 1000);

    let parts = 100;
    let mut source = String::new();
    for i in 0..parts {
      source.push_str(&alloc::format!("type T{i} = {{ f{i}: number }}\n"));
    }

    source.push_str("type Instance = { new: (('s0', extra: Instance?) -> T0)");
    for i in 1..parts {
      source.push_str(&alloc::format!(" & (('s{i}', extra: Instance?) -> T{i})"));
    }
    source.push_str(" }\n");

    source.push_str(
      r#"
local Instance: Instance = {} :: any

local function foo(a: typeof(Instance.new)) return if a then 2 else 3 end

foo(1 :: any)
"#,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options(&source, None);

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_optional_class_instances_are_invariant_new_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1000:type_infer_provisional_optional_class_instances_are_invariant_new_solver`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_optional_class_instances_are_invariant_new_solver

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_optional_class_instances_are_invariant_new_solver() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::create_some_extern_types::create_some_extern_types, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    create_some_extern_types(fixture.get_frontend());
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(ref: {read current: Parent?})
        end

        function bar(ref: {read current: Child?})
            foo(ref)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_optional_class_instances_are_invariant_old_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:982:type_infer_provisional_optional_class_instances_are_invariant_old_solver`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_optional_class_instances_are_invariant_old_solver

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_optional_class_instances_are_invariant_old_solver() {
    use alloc::string::String;

    use ulua_unit_test::{
      functions::create_some_extern_types::create_some_extern_types, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    create_some_extern_types(fixture.get_frontend());
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(ref: {current: Parent?})
        end

        function bar(ref: {current: Child?})
            foo(ref)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_oss_2305_keyof_index_example {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1530:type_infer_provisional_oss_2305_keyof_index_example`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_oss_2305_keyof_index_example

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_oss_2305_keyof_index_example() {
    use alloc::string::String;

    use ulua_analysis::records::uninhabited_type_function::UninhabitedTypeFunction;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local settingsTable = {}

        type Settings = typeof(settingsTable)

        local settings = {}

        function settings.getTopic<T>(topic: keyof<Settings> & T): { setting: <U>(setting: keyof<index<Settings, T>> & U) -> (index<index<Settings, T>, U>) }
            return {
                setting = function<U>(setting: keyof<index<Settings, T>> & U): index<index<Settings, T>, U>
                    return settingsTable[topic][setting]
                end
            }
        end

        return settings
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0]).unwrap_or_else(|| {
      panic!(
        "expected UninhabitedTypeFunction, got {:?}",
        result.errors[0]
      )
    });
  }
}

mod type_infer_provisional_pcall_calling_pcall {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1564:type_infer_provisional_pcall_calling_pcall`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_pcall_calling_pcall

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_pcall_calling_pcall() {
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
        --!strict
        pcall(pcall)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_refine_unknown_to_table_and_test_two_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1329:type_infer_provisional_refine_unknown_to_table_and_test_two_props`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_provisional_refine_unknown_to_table_and_test_two_props

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_refine_unknown_to_table_and_test_two_props() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_property::UnknownProperty;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: unknown): string
            if typeof(x) == 'table' then
                if typeof(x.foo) == 'string' and typeof(x.bar) == 'string' then
                    return x.foo .. x.bar
                end
            end
            return ''
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownProperty>(&result.errors[0])
      .unwrap_or_else(|| panic!("expected UnknownProperty, got {:?}", result.errors[0]));
    assert_eq!(Position::new(3, 56), result.errors[0].location.begin);
    assert_eq!(Position::new(3, 61), result.errors[0].location.end);
  }
}

mod type_infer_provisional_setmetatable_constrains_free_type_into_free_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:195:type_infer_provisional_setmetatable_constrains_free_type_into_free_table`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_provisional_setmetatable_constrains_free_type_into_free_table

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_setmetatable_constrains_free_type_into_free_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {}
        local b
        setmetatable(a, b)
        b = 1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("{-  -}", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_provisional_specialization_binds_with_prototypes_too_early {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:402:type_infer_provisional_specialization_binds_with_prototypes_too_early`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_specialization_binds_with_prototypes_too_early

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_specialization_binds_with_prototypes_too_early() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id(x) return x end
        local n2n: (number) -> number = id
        local s2s: (string) -> string = id
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

mod type_infer_provisional_table_containing_non_final_type_is_erroneously_cached {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1252:type_infer_provisional_table_containing_non_final_type_is_erroneously_cached`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
  //!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
  //!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_table_containing_non_final_type_is_erroneously_cached

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_table_containing_non_final_type_is_erroneously_cached() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::records::{property_type::Property, table_type::TableType};
    use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

    let mut fixture = NormalizeFixture::default();
    let scope = fixture.get_global_scope();
    let builtins = fixture.base.builtin_types;

    let mut table = TableType::new();
    let free_ty = unsafe {
      fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&*builtins, scope)
    };
    table
      .props
      .insert(String::from("foo"), Property::rw_type_id(free_ty));
    let table_ty = fixture.arena.add_type(table);

    let n1 = fixture
      .normalize(table_ty)
      .expect("expected normalized table");
    let n2 = fixture
      .normalize(table_ty)
      .expect("expected normalized table");

    assert!(Arc::ptr_eq(&n1, &n2));
  }
}

mod type_infer_provisional_table_insert_with_a_singleton_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:840:type_infer_provisional_table_insert_with_a_singleton_argument`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_table_insert_with_a_singleton_argument

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_table_insert_with_a_singleton_argument() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(t, x)
            if x == "hi" or x == "bye" then
                table.insert(t, x)
            end

            return t
        end

        local t = foo({}, "hi")
        table.insert(t, "totally_unrelated_type" :: "totally_unrelated_type")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{string}",
        to_string_type_id(fixture.base.require_type_string(&String::from("t")))
      );
    } else {
      assert_eq!(
        "{string | string}",
        to_string_type_id(fixture.base.require_type_string(&String::from("t")))
      );
    }
  }
}

mod type_infer_provisional_table_unification_infinite_recursion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1065:type_infer_provisional_table_unification_infinite_recursion`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_table_unification_infinite_recursion

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_table_unification_infinite_recursion() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
local tbl = {}

function tbl:f1(state)
    self.someNonExistentvalue2 = state
end

function tbl:f2()
    self.someNonExistentvalue:Dc()
end

function tbl:f3()
    self:f2()
    self:f1(false)
end
return tbl
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local tbl = require(game.A)
tbl:f3()
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_typeguard_inference_incomplete {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:41:type_infer_provisional_typeguard_inference_incomplete`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_typeguard_inference_incomplete

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_typeguard_inference_incomplete() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let code = String::from(
      r#"
        function f(a)
            if type(a) == "boolean" then
                local a1 = a
            elseif a.fn() then
                local a2 = a
            end
        end
    "#,
    );

    let expected = r#"
        function f(a:{fn:()->(T,U...)}): ()
            if type(a) == 'boolean' then
                local a1:boolean=a
            elseif a.fn() then
                local a2:{fn:()->(T,U...)}=a
            end
        end
    "#;

    let expected_with_new_solver = r#"
        function f(a:{fn:()->(unknown,...unknown)}): ()
            if type(a) == 'boolean' then
                local a1:{fn:()->(unknown,...unknown)}&boolean=a
            elseif a.fn() then
                local a2:{fn:()->(unknown,...unknown)}&(userdata|function|nil|number|integer|string|thread|buffer|table)=a
            end
        end
    "#;

    let expected_with_new_solver_nointeger = r#"
        function f(a:{fn:()->(unknown,...unknown)}): ()
            if type(a) == 'boolean' then
                local a1:{fn:()->(unknown,...unknown)}&boolean=a
            elseif a.fn() then
                local a2:{fn:()->(unknown,...unknown)}&(userdata|function|nil|number|string|thread|buffer|table)=a
            end
        end
    "#;

    let mut fixture = Fixture::fixture_bool(false);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      if FFlag::LuauIntegerType2.get() {
        expected_with_new_solver
      } else {
        expected_with_new_solver_nointeger
      }
    } else {
      expected
    };

    assert_eq!(expected, fixture.decorate_with_types(&code));
  }
}

mod type_infer_provisional_unification_inferring_never_for_refined_param {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1382:type_infer_provisional_unification_inferring_never_for_refined_param`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_unification_inferring_never_for_refined_param

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_unification_inferring_never_for_refined_param() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function __remove(__: number?) end

        function __removeItem(self, itemId: number)
            local index = self.getItem(itemId)
            if index then
               __remove(index)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ read getItem: (number) -> (never, ...unknown) }, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("__removeItem")))
    );
  }
}

mod type_infer_provisional_unify_more_complex_unions_that_include_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:969:type_infer_provisional_unify_more_complex_unions_that_include_nil`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_unify_more_complex_unions_that_include_nil

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_unify_more_complex_unions_that_include_nil() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Record = {prop: (string | boolean)?}

        function concatPagination(prop: (string | boolean | nil)?): Record
            return {prop = prop}
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_union_super_with_multiple_free_members_over_constrains_lower_bounds {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1587:type_infer_provisional_union_super_with_multiple_free_members_over_constrains_lower_bounds`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_union_super_with_multiple_free_members_over_constrains_lower_bounds

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_union_super_with_multiple_free_members_over_constrains_lower_bounds() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _propagate_bounds = ScopedFastFlag::new(
      &FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds,
      true,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<T, U>(x: T | U, y: T): T
            return y
        end
        local a = f(true, 1)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean | number",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_provisional_unions_should_work_with_bidirectional_typechecking {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1449:type_infer_provisional_unions_should_work_with_bidirectional_typechecking`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_unions_should_work_with_bidirectional_typechecking

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_unions_should_work_with_bidirectional_typechecking() {
    use alloc::string::String;

    use ulua_analysis::records::type_mismatch::TypeMismatch;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type dog = { name: string }
        local function bark(arg: { [dog]: dog | { left: dog?, right: dog? } })
            -- do something
            return arg
        end

        local molly: dog = { name = "molly" }
        local draco: dog = { name = "draco" }
        local cindy: dog = { name = "cindy" }
        local laika: dog = { name = "laika" }

        -- this should work because they should match with the left-right dog variant with optionals!
        bark{ [molly] = { left = laika }, [draco] = { right = cindy } }
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0])
      .unwrap_or_else(|| panic!("expected TypeMismatch, got {:?}", result.errors[0]));
    type_error_data_ref::<TypeMismatch>(&result.errors[1])
      .unwrap_or_else(|| panic!("expected TypeMismatch, got {:?}", result.errors[1]));
  }
}

mod type_infer_provisional_update_phonemes_minimized {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1237:type_infer_provisional_update_phonemes_minimized`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_update_phonemes_minimized

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_update_phonemes_minimized() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local video
        function(response)
            for index = 1, #response do
                video = video
            end
            return video
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1276:type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function find_first<T>(tbl: {T}, el)
            for i, e in tbl do
                if e == el then
                    return i
                end
            end
            return nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "<T>({T}, unknown) -> number",
        to_string_type_id(fixture.require_type_string(&String::from("find_first")))
      );
    } else {
      assert_eq!(
        "<T, b>({T}, b) -> number",
        to_string_type_id(fixture.require_type_string(&String::from("find_first")))
      );
    }
  }
}

mod type_infer_provisional_weird_fail_to_unify_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:416:type_infer_provisional_weird_fail_to_unify_type_pack`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_weird_fail_to_unify_type_pack

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_weird_fail_to_unify_type_pack() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f() return end
        local g = function() return f() end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_weirditer_should_not_loop_forever {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:161:type_infer_provisional_weirditer_should_not_loop_forever`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_provisional_weirditer_should_not_loop_forever

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_weirditer_should_not_loop_forever() {
    use alloc::string::String;

    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _type_pack_loop_limit = ScopedFastInt::new(&FInt::LuauTypeInferTypePackLoopLimit, 50);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function toVertexList(vertices, x, y, ...)
            if not (x and y) then return vertices end  -- no more arguments
            vertices[#vertices + 1] = {x = x, y = y}   -- set vertex
            return toVertexList(vertices, ...)         -- recurse
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_provisional_while_body_are_also_refined {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:215:type_infer_provisional_while_body_are_also_refined`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Node (Analysis/src/TopoSortStatements.cpp)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
  //!   - translates_to -> rust_item type_infer_provisional_while_body_are_also_refined

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_while_body_are_also_refined() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Node<T> = { value: T, child: Node<T>? }

        local function visitor<T>(node: Node<T>, f: (T) -> ())
            local current = node

            while current do
                f(current.value)
                current = current.child -- TODO: Can't work just yet. It thinks 'current' can never be nil. :(
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'Node<T>', but got 'Node<T>?'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_provisional_while_loops_fail_to_apply_refinements_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1476:type_infer_provisional_while_loops_fail_to_apply_refinements_1`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_while_loops_fail_to_apply_refinements_1

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_while_loops_fail_to_apply_refinements_1() {
    use alloc::string::String;

    use ulua_analysis::records::optional_value_access::OptionalValueAccess;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type walkoptions = {
	recursive: boolean?,
}

function bing(path : string  | walkoptions, opts: walkoptions?)
    return function ()
        while opts and opts.recursive do
        end
    end
end
    "#,
      ),
      None,
    );

    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<OptionalValueAccess>(error).is_some()),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_provisional_while_loops_fail_to_apply_refinements_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1502:type_infer_provisional_while_loops_fail_to_apply_refinements_2`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_provisional_while_loops_fail_to_apply_refinements_2

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_while_loops_fail_to_apply_refinements_2() {
    use alloc::string::String;

    use ulua_analysis::records::optional_value_access::OptionalValueAccess;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type walkoptions = {
	recursive: boolean?,
}

function bing(path : string  | walkoptions, opts: walkoptions?)
    return function ()
        while true do
            if opts and opts.recursive then
            end
        end
    end
end
    "#,
      ),
      None,
    );

    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<OptionalValueAccess>(error).is_some()),
      "{:?}",
      result.errors
    );
  }
}

mod type_infer_provisional_xpcall_returns_what_f_returns {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:135:type_infer_provisional_xpcall_returns_what_f_returns`
  //! Source: `tests/TypeInfer.provisional.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_provisional_xpcall_returns_what_f_returns

  #[cfg(test)]
  #[test]
  fn type_infer_provisional_xpcall_returns_what_f_returns() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let code = String::from(
      r#"
        local a, b, c = xpcall(function() return 1, "foo" end, function() return "foo", 1 end)
    "#,
    );

    let expected = r#"
        local a:boolean,b:number,c:string=xpcall(function(): (number,string)return 1,'foo'end,function(): (string,number)return'foo',1 end)
    "#;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options(&code, None);

    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(expected, fixture.base.decorate_with_types(&code));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
