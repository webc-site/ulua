extern crate alloc;

mod type_infer_cfa_do_assert_x {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_do_assert_x() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            do
                assert(x)
            end

            local foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_do_if_not_x_return {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_do_if_not_x_return() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            do
                if not x then
                    return
                end
            end

            local foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 8,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_early_return_in_a_loop_which_is_guaranteed_to_run_first {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_early_return_in_a_loop_which_is_guaranteed_to_run_first() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            repeat
                if not x then
                    return
                end

                local foo = x
            until math.random() > 0.5

            local bar = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 28,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_early_return_in_a_loop_which_is_guaranteed_to_run_first_2 {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_early_return_in_a_loop_which_is_guaranteed_to_run_first_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            for i = 1, 10 do
                if not x then
                    return
                end

                local foo = x
            end

            local bar = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 28,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_early_return_in_a_loop_which_isnt_guaranteed_to_run_first {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_early_return_in_a_loop_which_isnt_guaranteed_to_run_first() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            while math.random() > 0.5 do
                if not x then
                    return
                end

                local foo = x
            end

            local bar = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 28,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_for_record_do_if_not_x_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_for_record_do_if_not_x_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                do
                    if not record.value then
                        break
                    end
                end

                local foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_for_record_do_if_not_x_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_for_record_do_if_not_x_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                do
                    if not record.value then
                        continue
                    end
                end

                local foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                if not record.value then
                    break
                end

                local foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 34,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_elif_not_y_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_elif_not_y_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    break
                elseif not recordY.value then
                    break
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_elif_not_y_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_elif_not_y_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    break
                elseif not recordY.value then
                    continue
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_elif_not_y_fallthrough_elif_not_z_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_elif_not_y_fallthrough_elif_not_z_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}}, z: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                local recordZ = y[i]
                if not recordX.value then
                    break
                elseif not recordY.value then

                elseif not recordZ.value then
                    break
                end

                local foo = recordX.value
                local bar = recordY.value
                local baz = recordZ.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 14,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 15,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_elif_rand_break_elif_not_y_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_elif_rand_break_elif_not_y_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    break
                elseif math.random() > 0.5 then
                    break
                elseif not recordY.value then
                    break
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_elif_rand_break_elif_not_y_fallthrough {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_elif_rand_break_elif_not_y_fallthrough() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    break
                elseif math.random() > 0.5 then
                    break
                elseif not recordY.value then

                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_if_not_y_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_if_not_y_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    break
                end

                if not recordY.value then
                    break
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_break_if_not_y_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_break_if_not_y_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    break
                end

                if not recordY.value then
                    continue
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                if not record.value then
                    continue
                end

                local foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 7,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_elif_not_y_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_elif_not_y_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    continue
                elseif not recordY.value then
                    continue
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_elif_not_y_fallthrough_elif_not_z_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_elif_not_y_fallthrough_elif_not_z_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}}, z: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                local recordZ = y[i]
                if not recordX.value then
                    continue
                elseif not recordY.value then

                elseif not recordZ.value then
                    continue
                end

                local foo = recordX.value
                local bar = recordY.value
                local baz = recordZ.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 14,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 15,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_elif_not_y_throw_elif_not_z_fallthrough {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_elif_not_y_throw_elif_not_z_fallthrough() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}}, z: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                local recordZ = y[i]
                if not recordX.value then
                    continue
                elseif not recordY.value then
                    error("Y value not defined")
                elseif not recordZ.value then

                end

                local foo = recordX.value
                local bar = recordY.value
                local baz = recordZ.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 14,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 15,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_elif_rand_continue_elif_not_y_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_elif_rand_continue_elif_not_y_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    continue
                elseif math.random() > 0.5 then
                    continue
                elseif not recordY.value then
                    continue
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_elif_rand_continue_elif_not_y_fallthrough {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_elif_rand_continue_elif_not_y_fallthrough() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    continue
                elseif math.random() > 0.5 then
                    continue
                elseif not recordY.value then

                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_if_not_y_continue {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_if_not_y_continue() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    continue
                end

                if not recordY.value then
                    continue
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_continue_if_not_y_throw {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_continue_if_not_y_throw() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    continue
                end

                if not recordY.value then
                    error("Y value not defined")
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            if not x then
                return
            end

            local foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_elif_not_rand_return_elif_not_y_fallthrough {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_elif_not_rand_return_elif_not_y_fallthrough() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?, y: string?)
            if not x then
                return
            elseif math.random() > 0.5 then
                return
            elseif not y then

            end

            local foo = x
            local bar = y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_elif_not_y_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_elif_not_y_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                if not recordX.value then
                    return
                elseif not recordY.value then
                    break
                end

                local foo = recordX.value
                local bar = recordY.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 38,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_elif_not_y_fallthrough_elif_not_z_break {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_elif_not_y_fallthrough_elif_not_z_break() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}}, y: {{value: string?}}, z: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                local recordZ = y[i]
                if not recordX.value then
                    return
                elseif not recordY.value then

                elseif not recordZ.value then
                    break
                end

                local foo = recordX.value
                local bar = recordY.value
                local baz = recordZ.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 13,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 14,
        column: 38,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 15,
        column: 38,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_elif_not_y_fallthrough_elif_not_z_return {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_elif_not_y_fallthrough_elif_not_z_return() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?, y: string?, z: string?)
            if not x then
                return
            elseif not y then

            elseif not z then
                return
            end

            local foo = x
            local bar = y
            local baz = z
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 24,
      }))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 12,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_elif_not_y_return {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_elif_not_y_return() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?, y: string?)
            if not x then
                return
            elseif not y then
                return
            end

            local foo = x
            local bar = y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 8,
        column: 24,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_elif_rand_return_elif_not_y_return {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_elif_rand_return_elif_not_y_return() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?, y: string?)
            if not x then
                return
            elseif math.random() > 0.5 then
                return
            elseif not y then
                return
            end

            local foo = x
            local bar = y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_return_if_not_y_return {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_return_if_not_y_return() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?, y: string?)
            if not x then
                return
            end

            if not y then
                return
            end

            local foo = x
            local bar = y
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 10,
        column: 24,
      }))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 11,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_then_assert_false {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_then_assert_false() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            if not x then
                assert(false)
            end

            local foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_if_not_x_then_error {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_if_not_x_then_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            if not x then
                error("oops")
            end

            local foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 6,
        column: 24,
      }))
    );
  }
}

mod type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            type Foo = number

            if typeof(x) == "string" then
                return
            end

            local foo: Foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 8,
        column: 29,
      }))
    );
  }
}

mod type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope_breaking {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope_breaking() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                type Foo = number

                if typeof(record.value) == "string" then
                    break
                end

                local foo: Foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 43,
      }))
    );
  }
}

mod type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope_continuing {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope_continuing() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                type Foo = number

                if typeof(record.value) == "string" then
                    continue
                end

                local foo: Foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 43,
      }))
    );
  }
}

mod type_infer_cfa_tagged_unions {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_tagged_unions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function map<T, U, E>(result: Result<T, E>, f: (T) -> U): Result<U, E>
            if result.tag == "ok" then
                local tag = result.tag
                local val = result.value

                return { tag = "ok", value = f(result.value) }
            end

            local tag = result.tag
            local err = result.error

            return result
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 8,
        column: 35,
      }))
    );
    assert_eq!(
      "E",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 14,
        column: 31,
      }))
    );
    assert_eq!(
      "Err<E>",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 16,
        column: 19,
      }))
    );
  }
}

mod type_infer_cfa_tagged_unions_breaking {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_tagged_unions_breaking() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function process<T, E>(results: {Result<T, E>})
            for _, result in results do
                if result.tag == "ok" then
                    local tag = result.tag
                    local val = result.value

                    break
                end

                local tag = result.tag
                local err = result.error
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 39,
      }))
    );
    assert_eq!(
      "E",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 15,
        column: 35,
      }))
    );
  }
}

mod type_infer_cfa_tagged_unions_continuing {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_tagged_unions_continuing() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function process<T, E>(results: {Result<T, E>})
            for _, result in results do
                if result.tag == "ok" then
                    local tag = result.tag
                    local val = result.value

                    continue
                end

                local tag = result.tag
                local err = result.error
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 39,
      }))
    );
    assert_eq!(
      "E",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 15,
        column: 35,
      }))
    );
  }
}

mod type_infer_cfa_type_alias_does_not_leak_out {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_type_alias_does_not_leak_out() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: string?)
            if typeof(x) == "string" then
                return
            else
                type Foo = number
            end

            local foo: Foo = x
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 8,
        column: 29,
      }))
    );
  }
}

mod type_infer_cfa_type_alias_does_not_leak_out_breaking {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_type_alias_does_not_leak_out_breaking() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                if typeof(record.value) == "string" then
                    break
                else
                    type Foo = number
                end

                local foo: Foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 43,
      }))
    );
  }
}

mod type_infer_cfa_type_alias_does_not_leak_out_continuing {
  //! Source: `tests/TypeInfer.cfa.test.cpp`

  #[test]
  fn type_infer_cfa_type_alias_does_not_leak_out_continuing() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                if typeof(record.value) == "string" then
                    continue
                else
                    type Foo = number
                end

                local foo: Foo = record.value
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "nil",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 9,
        column: 43,
      }))
    );
  }
}
