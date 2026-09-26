extern crate alloc;

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_any_type_in_for_loop_should_propagate() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::LuauPropagateTypeAnnotationsInForInLoops, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        function my_iter(): any
            return {}
        end

        for index: number, value: string in my_iter() do
            print(index, value)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 18
    }))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 25
    }))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_cli_68448_iterators_need_not_accept_nil() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function makeEnum(members)
            local enum = {}
            for _, memberName in ipairs(members) do
                enum[memberName] = memberName
            end
            return enum
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "<a>({a}) -> { [a]: a }",
    to_string_type_id_to_string_options(
      fixture.base.require_type_string("makeEnum"),
      &mut ToStringOptions::new(true),
    )
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_correctly_scope_locals_while() {
  use ulua_analysis::records::unknown_symbol::UnknownSymbol;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        while true do
            local a = 1
        end

        print(a) -- oops!
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let us = type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("a", us.name());
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_iteration_explore_raycast_minimization() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local testResults = {}
        for _, testData in pairs(testResults) do
        end

        table.insert(testResults, {})
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_iteration_fragmented_keys() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function isIndexKey(k, contiguousLength)
            return true
        end

        local function getTableLength(tbl)
            local length = 1
            local value = rawget(tbl, length)
            while value ~= nil do
                length += 1
                value = rawget(tbl, length)
            end
            return length - 1
        end

        local function rawpairs(t)
            return next, t, nil
        end

        local function getFragmentedKeys(tbl)
            local keys = {}
            local keysLength = 0
            local tableLength = getTableLength(tbl)
            for key, _ in rawpairs(tbl) do
                if not isIndexKey(key, tableLength) then
                    keysLength = keysLength + 1
                    keys[keysLength] = key
                end
            end
            return keys, keysLength, tableLength
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_iteration_minimized_fragmented_keys_1() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function rawpairs(t)
            return next, t, nil
        end

        local function getFragmentedKeys(tbl)
            local _ = rawget(tbl, 0)
            for _ in rawpairs(tbl) do
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_iteration_minimized_fragmented_keys_2() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function getFragmentedKeys(tbl)
            local _ = rawget(tbl, 0)
            for _ in next, tbl, nil do
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_iteration_minimized_fragmented_keys_3() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function getFragmentedKeys(tbl)
            local _ = rawget(tbl, 0)
            for _ in pairs(tbl) do
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_iteration_on_never_gives_never() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local iter: never
        local ans
        for xs in iter do
            ans = xs
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "nil"
  } else {
    "never"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.base.require_type_string("ans"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_dcr_xpath_candidates() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Instance = {}
        local function findCandidates(instances: { Instance },  path: { string })
            for _, name in ipairs(path) do
            end
            return {}
        end

        local canditates = findCandidates({}, {})
        for _, canditate in ipairs(canditates) do end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_ensure_local_in_loop_does_not_escape() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x = 42
        repeat
            local x = ""
        until true
        -- The local inside the loop should have no effect on the local
        -- outside the loop.
        local y = x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("y"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_explicit_types_in_for_loop_should_propagate() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::LuauPropagateTypeAnnotationsInForInLoops, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        function my_iter(): {[number]: string}
            return {}
        end

        for index: number, value: string in my_iter() do
            print(index, value)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 18
    }))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 25
    }))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local n
        local s
        for i, v in pairs({ "foo" }) do
            n = i
            s = v
            print(i, v)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string("n"))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string("s"))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 18
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 21
      }))
    );
  } else {
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string("n"))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string("s"))
    );
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_annotations_apply_inside_lambdas() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::type_mismatch::TypeMismatch,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::LuauPropagateTypeAnnotationsInForInLoops, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        function my_iter(): any
            return {}
        end

        for index: number in my_iter() do
            local fn = function()
                index = ""
            end
            fn()
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let err = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(err.wanted_type));
  assert_eq!("string", to_string_type_id(err.given_type));
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_annotations_apply_to_function_expressions() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::type_mismatch::TypeMismatch,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::LuauPropagateTypeAnnotationsInForInLoops, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        function my_iter(): any
            return {}
        end

        local function takesString(s: string) end

        for index: number in my_iter() do
            takesString(index)
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let err = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("string", to_string_type_id(err.wanted_type));
  assert_eq!("number", to_string_type_id(err.given_type));
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values() {
  use ulua_analysis::records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    type_mismatch::TypeMismatch,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        local function hasDivisors(value: number, table)
            return false
        end

        function prime_iter(state, index)
            while hasDivisors(index, state) do
                index += 1
            end

            state[index] = true
            return index
        end

        function primes1()
            return prime_iter, {}
        end

        function primes2()
            return prime_iter, {}, ""
        end

        function primes3()
            return prime_iter, {}, 2
        end

        for p in primes1() do print(p) end -- mismatch in argument count

        for p in primes2() do print(p) end -- mismatch in argument types, prime_iter takes {}, number, we are given {}, string

        for p in primes3() do print(p) end -- no error
    "#
,
      None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let acm =
    type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(CountMismatchContext::Arg, acm.context());
  assert_eq!(2, acm.expected());
  assert_eq!(1, acm.actual());

  let tm = type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!(fixture.base.get_builtins().number_type, tm.wanted_type);
  assert_eq!(fixture.base.get_builtins().string_type, tm.given_type);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_error_on_iterator_requiring_args_but_none_given() {
  use ulua_analysis::records::count_mismatch::{CountMismatch, CountMismatchContext};
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function prime_iter(state, index)
            return 1
        end

        for p in prime_iter do print(p) end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let acm =
    type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(CountMismatchContext::Arg, acm.context());
  assert_eq!(2, acm.expected());
  assert_eq!(0, acm.actual());
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_on_error() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(x)
            gobble.prop = x.otherprop
        end

        local p
        for _, part in i_am_not_defined do
            p = part
            f(part)
            part.thirdprop = false
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "*error-type*?"
  } else {
    "*error-type*"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("p"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_on_non_function() {
  use ulua_analysis::records::cannot_call_non_function::CannotCallNonFunction;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local bad_iter = 5

        for a in bad_iter() do
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
    .expect("expected CannotCallNonFunction");
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_should_fail_with_non_function_iterator() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo = "bar"
        for i, v in foo do
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Cannot call a value of type string",
    to_string_type_error(&result.errors[0])
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_where_iteratee_is_free() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        function _:_(...)
        end

        repeat
            if _ then
            else
                _ = ...
            end
        until _

        for _ in _() do
        end
    "#,
    None,
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_with_custom_iterator() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function primes()
            return function (state: number) end,  2
        end

        for p, q in primes do
            q = ""
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
  assert_eq!(fixture.get_builtins().string_type, tm.given_type);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_with_incompatible_args_to_iterator() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_ast::records::{location::Location, position::Position};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function my_iter(state: string, index: number)
            return state, index
        end

        local my_state = {}
        local first_index = "first"

        -- Type errors here.  my_state and first_index cannot be passed to my_iter
        for a, b in my_iter, my_state, first_index do
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!(
    Location {
      begin: Position {
        line: 9,
        column: 20,
      },
      end: Position {
        line: 9,
        column: 27,
      },
    },
    result.errors[0].location
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_with_next() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local n
        local s
        for i, v in next, { "foo" } do
            n = i
            s = v
            print(i, v)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string("n"))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string("s"))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 18
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 21
      }))
    );
  } else {
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string("n"))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string("s"))
    );
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_with_next_and_multiple_elements() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local n
        local s
        for i, v in next, { "foo", "bar" } do
            n = i
            s = v
            print(i, v)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string("n"))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_string("s"))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 18
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 21
      }))
    );
  } else {
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string("n"))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_string("s"))
    );
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_loop_with_zero_iterators_dcr() {
  use ulua_analysis::records::generic_error::GenericError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function no_iter() end
        for key in no_iter() do end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
  assert_eq!(
    "for..in loops require at least one value to iterate over.  Got zero",
    err.message()
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_require() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        for _ in require do
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_surprising_iterator() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function broken(): (...() -> ())
    return function() end, function() end
end

for p in broken() do print(p) end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_with_a_custom_iterator_should_type_check() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::LuauPropagateTypeAnnotationsInForInLoops, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function range(l, h): () -> number
            return function()
                return l
            end
        end

        for n: string in range(1, 10) do
            print(n)
        end
    "#,
    None,
  );

  if fflag::LuauPropagateTypeAnnotationsInForInLoops.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_with_an_iterator_of_type_any() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local it: any
        local a, b
        for i, v in it do
            a, b = i, v
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_with_generic_next() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        for k: number, v: number in next, {1, 2, 3} do
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_in_with_just_one_iterator_is_ok() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function keys(dictionary)
            local new = {}
            local index = 1

            for key in pairs(dictionary) do
                new[index] = key
                index = index + 1
            end

            return new
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_loop() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local q
        for i=0, 50, 2 do
            q = i
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "number?"
  } else {
    "number"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("q"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_loop_lower_bound_is_string() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        for i: unknown = 1, 10 do end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_loop_lower_bound_is_string_2() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        for i: never = 1, 10 do end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be unreachable, but got 'number'",
    to_string_type_error(&result.errors[0])
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_for_loop_lower_bound_is_string_3() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        for i: number | string = 1, 10 do end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_forin_metatable_iter_mm() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Iterable<T...> = typeof(setmetatable({}, {} :: {
            __iter: (Iterable<T...>) -> () -> T...
        }))

        for i, v in {} :: Iterable<...number> do
            print(i, v)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 6,
      column: 18
    }))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 6,
      column: 21
    }))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_forin_metatable_no_iter_mm() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local t = setmetatable({1, 2, 3}, {})

        for i, v in t do
            print(i, v)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 4,
      column: 18
    }))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 4,
      column: 21
    }))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_fuzz_fail_missing_instantitation_follow() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        function _(l0:number)
        return _
        end
        for _ in _(8) do
        end
    "#,
    None,
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_incorrect_type_annotation_types_in_loop_should_propagate_with_errors() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::type_mismatch::TypeMismatch,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::LuauPropagateTypeAnnotationsInForInLoops, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        function my_iter(): any
            return {}
        end
        for index: number, value: string in my_iter() do
            index = ""
            print(index)
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let err = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(err.wanted_type));
  assert_eq!("string", to_string_type_id(err.given_type));
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_ipairs_produces_integral_indices() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local key
        for i, e in ipairs({}) do key = i end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "number?"
  } else {
    "number"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.base.require_type_string("key"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iter_constraint_before_loop_body() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local T = {
    	    fields = {},
        }

        function f()
            for u, v in pairs(T.fields) do
                T.fields[u] = nil
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iter_mm_results_are_lvalue() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local foo = setmetatable({}, {
            __iter = function()
                return pairs({1, 2, 3})
            end,
        })

        for k, v in foo do
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iterate_array_of_singletons() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        type Direction = "Left" | "Right" | "Up" | "Down"
        local Instructions: { Direction } = { "Left", "Down" }

        for _, step in Instructions do
            local dir: Direction = step
            print(dir)
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iterate_over_free_table() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function print(x) end

        function dump(tbl)
            print(tbl.whatever)
            for k, v in tbl do
                print(k)
                print(v)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iterate_over_properties() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function f()
            local t = { p = 5, q = "hello" }
            for k, v in t do
                return k, v
            end

            error("")
        end

        local k, v = f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "unknown",
    to_string_type_id(fixture.base.require_type_string("k"))
  );
  assert_eq!(
    "unknown",
    to_string_type_id(fixture.base.require_type_string("v"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iterate_over_properties_nonstrict() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        local function f()
            local t = { p = 5, q = "hello" }
            for k, v in t do
                return k, v
            end

            error("")
        end

        local k, v = f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iteration_no_table_passed() {
  use ulua_analysis::records::generic_error::GenericError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"

type Iterable = typeof(setmetatable(
    {},
    {}::{
        __iter: (self: Iterable) -> (any, number) -> (number, string)
    }
))

local t: Iterable

for a, b in t do end
"#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let ge = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
  assert_eq!(
    "__iter metamethod must return (next[, table[, state]])",
    ge.message()
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iteration_preserves_error_suppression() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _v1 = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function first(x: any)
            for k, v in pairs(x) do
                print(k, v)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "*error-type* | ~nil",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 3,
      column: 22
    }))
  );
  assert_eq!(
    "any",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 3,
      column: 25
    }))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iteration_regression_issue_69967() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Iterable = typeof(setmetatable(
            {},
            {}::{
                __iter: (self: Iterable) -> () -> (number, string)
            }
        ))

        local t: Iterable

        for a, b in t do end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_iteration_regression_issue_69967_alt() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type Iterable = typeof(setmetatable(
            {},
            {}::{
                __iter: (self: Iterable) -> () -> (number, string)
            }
        ))

        local t: Iterable
        local x, y

        for a, b in t do
            x = a
            y = b
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let (expected_x, expected_y) = if !fflag::DebugLuauForceOldSolver.get() {
    ("number?", "string?")
  } else {
    ("number", "string")
  };
  assert_eq!(
    expected_x,
    to_string_type_id(fixture.base.require_type_string("x"))
  );
  assert_eq!(
    expected_y,
    to_string_type_id(fixture.base.require_type_string("y"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_loop_iter_basic() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t: {string} = {}
        local key
        for k: number in t do
        end
        for k: number, v: string in t do
        end
        for k, v in t do
            key = k
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "number?"
  } else {
    "number"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("key"))
  );
}

mod type_infer_loops_loop_iter_metamethod {
  //! 上游 `tests/TypeInfer.loops.test.cpp` 将以下 4 个 `__iter` 元方法推断用例
  //! （loop_iter_metamethod_nil / _not_enough_returns / _ok / _ok_with_inference）
  //! 整体置于 `#if 0` 下禁用（CLI-116499 / CLI-116500，Free types persisting until
  //! typechecking time）。本 port 与上游保持一致，**有意不测**这些用例，故此处不提供
  //! `#[test]`——避免空壳测试永远 pass、制造假绿信号。
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_loop_iter_no_indexer_nonstrict() {
  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
        local t = {}
        for k, v in t do
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_loop_iter_no_indexer_strict() {
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t = {}
        for k, v in t do
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_loop_iter_trailing_nil() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t: {string} = {}
        local extra
        for k, v, e in t do
            extra = e
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_string("extra"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_loop_typecheck_crash_on_empty_optional() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local t = {}
        for _ in t do
            for _ in assert(missing()) do
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_lti_fuzzer_uninitialized_loop_crash() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        for l0=_,_ do
            return _()
        end
    "#,
    None,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_oss_1413() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function KahanSum(values: {number}): number
            local sum: number = 0
            local compensator: number = 0
            for _, value in values do
                local y = value - compensator
                local t = sum + y
                compensator = (t - sum) - y
                sum = t
            end
            return sum
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function HistogramString(values: {number})
            local histogram = {}
            values = table.clone(values)
            table.sort(values)

            local count = #values
            local range = (count - 1)

            local digitIndex = range // 2 + 1
            while digitIndex < count and values[digitIndex] == 0 do
                digitIndex = count - ((count - digitIndex) // 2)
            end
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function fun1()
            local foo = 1
            local bar = foo - foo + foo
            while false do
                foo = bar
            end
        end
        local function fun2()
            local foo = 1
            while false do
                local bar = foo - foo + foo
                foo = bar
            end
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_oss_1480() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Part = { Parent: Part? }
        type Instance = Part

        local part = {} :: Part

        local currentParent: Instance? = part.Parent
        while currentParent ~= nil do
            currentParent = currentParent.Parent
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_oss_1851_union_of_many_strings() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
--!strict
type union = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

local example: { [union]: number } = {}

for key in example do
end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_pairs_should_not_retroactively_add_an_indexer() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        local prices = {
            hat = 1,
            bat = 2,
        }
        print(prices.wwwww)
        for _, _ in pairs(prices) do
        end
        print(prices.wwwww)
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_properly_infer_iteratee_is_a_free_table() {
  use ulua_analysis::records::unknown_property::UnknownProperty;
  use ulua_ast::records::{location::Location, position::Position};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        for iter in pairs({}) do
            iter:g().p = true
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 12,
        },
        end: Position {
          line: 2,
          column: 18,
        },
      },
      result.errors[0].location
    );
  } else {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_rbxl_place_file_crash_for_wrong_constraints() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local VehicleParameters = {
    -- These are default values in the case the package structure is broken
	StrutSpringStiffnessFront = 28000,
}

local function updateFromConfiguration()
	for property, value in pairs(VehicleParameters) do
        VehicleParameters[property] = value
	end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_repeat_is_linearish() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local x = nil
        if math.random () > 0.5 then
            x = ""
            repeat
                error("spooky scary error")
            until true
        end
        -- The repeat in the above branch unconditionally fires the error, so
        -- this should _always_ be `nil`
        local y = x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "nil",
    to_string_type_id(fixture.base.require_type_string("y"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_repeat_loop() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local i
        repeat
            i = 'hi'
        until true
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "string?"
  } else {
    "string"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("i"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_repeat_loop_assignment() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local x = nil
        repeat
            x = 42
        until math.random() > 0.5
        local y = x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("y"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_repeat_loop_assignment_with_break() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local x = nil
        repeat
            x = 42
        until math.random() > 0.5
        local y = x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("y"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_repeat_loop_condition_binds_to_its_block() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        repeat
            local x = true
        until x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_repeat_unconditionally_fires_error() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local x = nil
        repeat
            x = 42
        until true
        -- `x` should unconditionally be `number` here as the assignment
        -- above will _always_ run.
        local y = x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("y"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_symbols_in_repeat_block_should_not_be_visible_beyond_until_condition() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        repeat
            local x = true
        until x

        print(x)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_trivial_ipairs_usage() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local next, t, s = ipairs({1, 2, 3})
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "({number}, number) -> (number?, number)",
    to_string_type_id(fixture.base.require_type_string("next"))
  );
  assert_eq!(
    "{number}",
    to_string_type_id(fixture.base.require_type_string("t"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("s"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_try_dispatch_iterable_function_under_constrained_loop_should_not_assert() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
local function foo(Instance)
    for _, Child in next, Instance:GetChildren() do
    end
end
    "#,
    None,
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_unreachable_code_after_infinite_loop() {
  use ulua_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            function unreachablecodepath(a): number
                while true do
                    if a then return 10 end
                end
                -- unreachable
            end
            unreachablecodepath(4)
        "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            function reachablecodepath(a): number
                while true do
                    if a then break end
                    return 10
                end

                print("x") -- correct error
            end
            reachablecodepath(4)
        "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
    .expect("expected FunctionExitsWithoutReturning");

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            function unreachablecodepath(a): number
                repeat
                    if a then return 10 end
                until false

                -- unreachable
            end
            unreachablecodepath(4)
        "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            function reachablecodepath(a, b): number
                repeat
                    if a then break end

                    if b then return 10 end
                until false

                print("x") -- correct error
            end
            reachablecodepath(4)
        "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
    .expect("expected FunctionExitsWithoutReturning");

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
            function unreachablecodepath(a: number?): number
                repeat
                    return 10
                until a ~= nil

                -- unreachable
            end
            unreachablecodepath(4)
        "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_varlist_declared_by_for_in_loop_should_be_free() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local T = {}

        function T.f(p)
            for i, v in pairs(p) do
                T.f(v)
            end
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  } else {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_while_loop() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local i
        while true do
            i = 8
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "number?"
  } else {
    "number"
  };
  assert_eq!(
    expected,
    to_string_type_id(fixture.require_type_string("i"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_while_loop_assign_different_type() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function takesString(_: string) end
        local function takesNil(_: nil) end
        local function foo()
            local x = ""
            takesString(x)
            while math.random () > 0.5 do
                x = nil
                takesNil(x)
            end
            return x
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "() -> string?",
    to_string_type_id(fixture.base.require_type_string("foo"))
  );
}

// Ported from `tests/TypeInfer.loops.test.cpp`.
// Source: `tests/TypeInfer.loops.test.cpp`
#[test]
fn type_infer_loops_while_loop_error_in_body() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function foo()
            local x = ""
            while math.random () > 0.5 do
                x = nil
                error("why did you make x nil tho")
            end
            return x
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "() -> string",
    to_string_type_id(fixture.base.require_type_string("foo"))
  );
}
// 假缺口（无需移植，对照 `tests/TypeInfer.loops.test.cpp`，共 4 例）：
// - loop_iter_metamethod_nil（:864）、loop_iter_metamethod_not_enough_returns
//   （:881）、loop_iter_metamethod_ok（:903）、loop_iter_metamethod_ok_with_
//   inference（:921）——cpp 侧全部整段包在 `#if 0`（CLI-116499/CLI-116500）中，
//   属上游死码，不在 cpp 有效覆盖面；rc-r01 旧账 3 为假缺口。
