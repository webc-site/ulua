extern crate alloc;

mod type_infer_typestates_assign_a_local_and_then_refine_it {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_assign_a_local_and_then_refine_it() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            x = nil

            if typeof(x) == "string" then
                local y: typeof(x) = "hello"
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be unreachable, but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_typestates_assign_different_values_to_x {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_assign_different_values_to_x() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: string? = nil
        local a = x
        x = "hello!"
        local b = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_typestates_assign_in_an_if_branch_without_else {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_assign_in_an_if_branch_without_else() {
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
        --!strict
        local x
        local coinflip : () -> boolean = (nil :: any)

        if coinflip () then
            x = "I win."
        end

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 14
      }))
    );
  }
}

mod type_infer_typestates_assignment_identity {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_assignment_identity() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = 5
        x = x

        local a = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_typestates_assignment_swap {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_assignment_swap() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x, y = 5, "hello"
        x, y = y, x

        local a, b = x, y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_typestates_capture_upvalue_in_returned_function {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_capture_upvalue_in_returned_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function def()
            local i : number = 0
            local function Counter()
                i = i + 1
                return i
            end
            return Counter
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "() -> () -> number",
      to_string_type_id(fixture.require_type_string(&String::from("def")))
    );
  }
}

mod type_infer_typestates_captured_locals_do_not_mutate_upvalue_type {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_captured_locals_do_not_mutate_upvalue_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil

        function f()
            print(x)
            x = "five"
        end

        x = 5
        f()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number?", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
    assert_eq!(
      "number?",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position {
            line: 4,
            column: 18,
          })
      )
    );
  }
}

mod type_infer_typestates_captured_locals_do_not_mutate_upvalue_type_2 {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_captured_locals_do_not_mutate_upvalue_type_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {x = nil}

        function f()
            print(t.x)
            t = {x = "five"}
        end

        t = {x = 5}
        f()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: nil } | { x: number }",
      to_string_type_id_to_string_options(err.wanted_type, &mut opts)
    );
    assert_eq!("{ x: string }", to_string_type_id(err.given_type));
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: nil } | { x: number }",
      to_string_type_id_to_string_options(
        fixture
          .base
          .base
          .require_type_at_position_position(Position {
            line: 4,
            column: 18
          }),
        &mut opts,
      )
    );
    assert_eq!(
      "number?",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position {
            line: 4,
            column: 20,
          })
      )
    );
  }
}

mod type_infer_typestates_compound_assignment {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_compound_assignment() {
    use alloc::string::String;

    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = 5
        x += 7

        local a = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_extraneous_lvalues_are_populated_with_nil {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_extraneous_lvalues_are_populated_with_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (string, number)
            return "hello", 5
        end

        local x, y, z = f()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Function only returns 2 values, but 3 are required here",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("z")))
    );
  }
}

mod type_infer_typestates_fuzzer_normalized_type_variables_are_bad {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_fuzzer_normalized_type_variables_are_bad() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _
        while _[""] do
            _, _ = nil
            while _.n0 do
                _, _ = nil
            end
            _, _ = nil
        end
        while _[""] do
            while if _ then if _ then _ else "" else "" do
                _, _ = nil
                do
                end
                _, _, _ = nil
            end
            _, _ = nil
            _, _, _ = nil
            while _.readi16 do
                _, _ = nil
            end
            _, _ = nil
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_fuzzer_table_freeze_in_binary_expr {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_fuzzer_table_freeze_in_binary_expr() {
    use alloc::string::String;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local _
            if _ or table.freeze(_,_) or table.freeze(_,_) then
            end
        "#,
        ),
        None,
      )
    }));

    let payload = result.expect_err("expected InternalCompilerError");
    assert!(
      payload.is::<InternalCompilerError>(),
      "expected InternalCompilerError panic payload"
    );
  }
}

mod type_infer_typestates_fuzzer_table_freeze_in_conditional_expr {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_fuzzer_table_freeze_in_conditional_expr() {
    use alloc::string::String;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local _
            if
                if table.freeze(_,_) then _ else _
            then
            end
        "#,
        ),
        None,
      )
    }));

    let payload = result.expect_err("expected InternalCompilerError");
    assert!(
      payload.is::<InternalCompilerError>(),
      "expected InternalCompilerError panic payload"
    );
  }
}

mod type_infer_typestates_initialize_x_of_type_string_or_nil_with_nil {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_initialize_x_of_type_string_or_nil_with_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: string? = nil
        local a = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_typestates_invalidate_type_refinements_upon_assignments {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_invalidate_type_refinements_upon_assignments() {
    use alloc::string::String;

    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ok<T> = { tag: "ok", val: T }
        type Err<E> = { tag: "err", err: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function f<T, E>(res: Result<T, E>)
            assert(res.tag == "ok")
            local tag: "ok", val: T = res.tag, res.val
            res = { tag = "err" :: "err", err = (5 :: any) :: E }
            local tag: "err", err: E = res.tag, res.err
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_local_assigned_in_either_branches_that_falls_through {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_local_assigned_in_either_branches_that_falls_through() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        if math.random() > 0.5 then
            x = 5
        else
            x = "hello"
        end
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number | string",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_typestates_local_assigned_in_only_one_branch_that_falls_through {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_local_assigned_in_only_one_branch_that_falls_through() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        if math.random() > 0.5 then
            x = 5
        end
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_typestates_local_t_is_assigned_a_fresh_table_with_x_assigned_a_union_and_then_assert_restricts_actual_outflow_of_types {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_local_t_is_assigned_a_fresh_table_with_x_assigned_a_union_and_then_assert_restricts_actual_outflow_of_types()
   {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = nil

        if math.random() > 0.5 then
            t = {}
            t.x = if math.random() > 0.5 then 5 else "hello"
            assert(typeof(t.x) == "string")
        else
            t = {}
            t.x = if math.random() > 0.5 then 7 else true
            assert(typeof(t.x) == "boolean")
        end

        local x = t.x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean | number | string",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_typestates_local_that_will_be_assigned_later {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_local_that_will_be_assigned_later() {
    use alloc::string::String;

    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: string
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_modify_captured_table_field {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_modify_captured_table_field() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local state = { x = 0 }
        function incr()
            state.x = state.x + 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let rand_ty = fixture
      .get_type(&String::from("state"), false)
      .expect("expected state type");
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "{ x: number }",
        to_string_type_id_to_string_options(rand_ty, &mut opts)
      );
    } else {
      assert_eq!(
        "{| x: number |}",
        to_string_type_id_to_string_options(rand_ty, &mut opts)
      );
    }
  }
}

mod type_infer_typestates_multiple_assignments_in_loops {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_multiple_assignments_in_loops() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil

        for i = 1, 10 do
            x = 5
            x = "hello"
        end

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_typestates_oss_1547 {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_oss_1547() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local rand = 0

        function a()
            rand = (rand % 4) + 1;
        end

        function b()
            rand = math.max(rand - 1, 0);
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let rand_ty = fixture
      .base
      .get_type(&String::from("rand"), false)
      .expect("expected rand type");
    assert_eq!("number", to_string_type_id(rand_ty));
  }
}

mod type_infer_typestates_oss_1547_simple {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_oss_1547_simple() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local rand = 0

        function a()
            rand = (rand % 4) + 1;
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let rand_ty = fixture
      .base
      .get_type(&String::from("rand"), false)
      .expect("expected rand type");
    assert_eq!("number", to_string_type_id(rand_ty));
  }
}

mod type_infer_typestates_oss_1561 {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_oss_1561() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Vector3 with
            X: number
            Y: number
            Z: number
        end

        declare Vector3: {
            new: (number?, number?, number?) -> Vector3
        }
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local targetVelocity: Vector3 = Vector3.new()
        function set2D(X: number, Y: number)
            targetVelocity = Vector3.new(X, Y, targetVelocity.Z)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number, number) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("set2D")))
    );
  }
}

mod type_infer_typestates_oss_1575 {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_oss_1575() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local flag = true
        local function Flip()
            flag = not flag
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_parameter_x_is_some_type_or_optional_then_assigned_with_alternate_value {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_parameter_x_is_some_type_or_optional_then_assigned_with_alternate_value()
  {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: number?)
            x = x or 5
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number?) -> number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_typestates_parameter_x_was_constrained_by_two_types {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_parameter_x_was_constrained_by_two_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x): string?
            local y: string | number = x
            return y
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(!result.errors.is_empty(), "{:?}", result.errors);

      let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("string?", to_string_type_id(err.wanted_type));
      assert_eq!("number | string", to_string_type_id(err.given_type));
      assert_eq!(
        "(number | string) -> string?",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "(string) -> string?",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
      );
    }
  }
}

mod type_infer_typestates_parameter_x_was_constrained_by_two_types_2 {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_parameter_x_was_constrained_by_two_types_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x): number?
            local y: string? = nil  -- 'y <: string?
            y = x                   -- 'y ~ 'x
            return y                -- 'y <: number?

                                    -- We therefore infer 'y <: (string | nil) & (number | nil)
                                    -- or 'y <: nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(nil) -> number?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_typestates_prototyped_recursive_functions {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_prototyped_recursive_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f
        function f()
            if math.random() > 0.5 then
                f()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(() -> ())?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_typestates_prototyped_recursive_functions_but_has_future_assignments {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_prototyped_recursive_functions_but_has_future_assignments() {
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
        local f
        function f()
            if math.random() > 0.5 then
                f()
            end
        end
        f = 5
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "((() -> ()) | number)?",
      to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_typestates_prototyped_recursive_functions_but_has_previous_assignments {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_prototyped_recursive_functions_but_has_previous_assignments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f
        f = 5
        function f()
            if math.random() > 0.5 then
                f()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "((() -> ()) | number)?",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_typestates_recursive_function {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_recursive_function() {
    use alloc::string::String;

    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x)
            f(5)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_recursive_local_function {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_recursive_local_function() {
    use alloc::string::String;

    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x)
            f(5)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_refine_a_local_and_then_assign_it {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_refine_a_local_and_then_assign_it() {
    use alloc::string::String;

    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            if typeof(x) == "string" then
                x = nil
            end

            local y: nil = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_refinement_through_erroring {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_refinement_through_erroring() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Payload = { payload: number }

        local function decode(s: string): Payload?
            return (nil :: any)
        end

        local function decodeEx(s: string): Payload
            local p = decode(s)
            if not p then
                error("failed to decode payload!!!")
            end
            return p
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_refinement_through_erroring_in_loop {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_refinement_through_erroring_in_loop() {
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
        --!strict

        local x = nil

        while math.random() > 0.5 do
            x = 42
            return
        end

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 14
      }))
    );
  }
}

mod type_infer_typestates_table_freeze_in_conditional {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_table_freeze_in_conditional() {
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
        local t = { x = 42 }
        if math.random() > 0.5 and table.freeze(t) then
        end
        t.y = 13
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_then_branch_assigns_and_else_branch_also_assigns_but_is_met_with_return {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_then_branch_assigns_and_else_branch_also_assigns_but_is_met_with_return()
  {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        if math.random() > 0.5 then
            x = 5
        else
            x = "hello"
            return
        end
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_typestates_then_branch_assigns_but_is_met_with_return_and_else_branch_assigns {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_then_branch_assigns_but_is_met_with_return_and_else_branch_assigns() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = nil
        if math.random() > 0.5 then
            x = 5
            return
        else
            x = "hello"
        end
        local y = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_typestates_throw_in_else_branch {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_throw_in_else_branch() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local x
        local coinflip : () -> boolean = (nil :: any)

        if coinflip () then
            x = "I win."
        else
            error("You lose.")
        end

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 14
      }))
    );
  }
}

mod type_infer_typestates_throw_in_if_branch {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_throw_in_if_branch() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local x
        local coinflip : () -> boolean = (nil :: any)

        if coinflip () then
            error("You lose.")
        else
            x = "I win."
        end

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 14
      }))
    );
  }
}

mod type_infer_typestates_throw_in_if_branch_and_do_nothing_in_else {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_throw_in_if_branch_and_do_nothing_in_else() {
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
        --!strict
        local x
        local coinflip : () -> boolean = (nil :: any)

        if coinflip () then
            error("You lose.")
        else
        end

        print(x)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 14
      }))
    );
  }
}

mod type_infer_typestates_type_refinement_in_loop {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_type_refinement_in_loop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local function onEachString(t: { string | number })
            for _, v in t do
                if type(v) ~= "string" then
                    continue
                end
                print(v)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number | string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 4,
        column: 24
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 22
      }))
    );
  }
}

mod type_infer_typestates_typestate_globals {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_typestate_globals() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare foo: string | number
        declare function f(x: string): ()
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        foo = "a"
        f(foo)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_typestates_typestate_unknown_global {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_typestate_unknown_global() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        x = 5
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      result.errors[0].data,
      TypeErrorData::UnknownSymbol(_)
    ));
  }
}

mod type_infer_typestates_typestates_do_not_apply_to_the_initial_local_definition {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_typestates_do_not_apply_to_the_initial_local_definition() {
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

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MyType = number | string
        local foo: MyType = 5
        print(foo)
        foo = 7
        print(foo)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "number | string",
      to_string_type_id_to_string_options(
        fixture.base.require_type_at_position_position(Position {
          line: 3,
          column: 14
        }),
        &mut opts,
      )
    );
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "number",
      to_string_type_id_to_string_options(
        fixture.base.require_type_at_position_position(Position {
          line: 5,
          column: 14
        }),
        &mut opts,
      )
    );
  }
}

mod type_infer_typestates_typestates_preserve_error_suppression {
  //! Ported from `tests/TypeInfer.typestates.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_typestates_typestates_preserve_error_suppression() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::type_state_fixture::TypeStateFixture;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(r#"
        local a: any = 51
        a = "pickles" -- We'll have a new DefId for this iteration of `a`.  Its type must also be error-suppressing
        print(a)
    "#),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "*error-type* | string",
      to_string_type_id_to_string_options(
        fixture
          .base
          .base
          .require_type_at_position_position(Position {
            line: 3,
            column: 14
          }),
        &mut opts,
      )
    );
  }
}
