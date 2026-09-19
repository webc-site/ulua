extern crate alloc;

mod type_infer_anyerror_any_type_propagates {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_any_type_propagates() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: any
        local bar = foo:method("argument")
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("bar")))
    );
  }
}

mod type_infer_anyerror_assign_prop_to_table_by_calling_any_yields_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_assign_prop_to_table_by_calling_any_yields_any() {
    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: any
        local T = {}

        T.prop = f()

        return T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let table_ty = follow_type_id(fixture.require_type_string(&String::from("T")));
    let table = get_type_id::<TableType>(table_ty).expect("expected table");
    let prop = table.props.get("prop").expect("expected prop");
    let read_ty = prop.read_ty.expect("expected readable prop");
    assert_eq!("any", to_string_type_id(read_ty));
  }
}

mod type_infer_anyerror_call_to_any_yields_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_call_to_any_yields_any() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: any
        local b = a()
    "#,
      ),
      None,
    );

    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_anyerror_calling_error_type_yields_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_calling_error_type_yields_error() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::unknown_symbol::UnknownSymbol,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{functions::find_error::find_error, records::fixture::Fixture};

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = unknown.Parent.Reward.GetChildren()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = find_error::<UnknownSymbol>(&result).expect("expected UnknownSymbol error");
    assert_eq!("unknown", err.name());

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "any"
    } else {
      "*error-type*"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_can_get_length_of_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_can_get_length_of_any() {
    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = ({} :: any)
        local bar = #foo
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let bar_type = fixture.require_type_string(&String::from("bar"));
    assert_eq!(
      Some(PrimitiveType::NUMBER),
      fixture.get_primitive_type(bar_type)
    );
  }
}

mod type_infer_anyerror_can_subscript_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_can_subscript_any() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: any
        local bar = foo[5]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("bar")))
    );
  }
}

mod type_infer_anyerror_cast_to_table_of_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_cast_to_table_of_any() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local v = {true} :: {any}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_chain_calling_error_type_yields_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_chain_calling_error_type_yields_error() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = Utility.Create "Foo" {}
    "#,
      ),
      None,
    );

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "any"
    } else {
      "*error-type*"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_check_methods_of_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_check_methods_of_any() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x: any = {}
function x:y(z: number)
    local s: string = z
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_check_methods_of_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_check_methods_of_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local x = (true).foo
function x:y(z: number)
    local s: string = z
end
"#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_dot_on_error_type_does_not_produce_an_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_dot_on_error_type_does_not_produce_an_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = (true).x
        foo.x = foo.y
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_for_in_loop_iterator_is_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_is_any() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local bar = nil :: any

        local a
        for b in bar do
            a = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "(*error-type* | ~nil)?"
    } else {
      "any"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_for_in_loop_iterator_is_any_2 {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_is_any2() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local bar = nil :: any

        local a
        for b in bar() do
            a = b
        end
    "#,
      ),
      None,
    );

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "(*error-type* | ~nil)?"
    } else {
      "any"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_for_in_loop_iterator_is_any_pack {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_is_any_pack() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function bar(): ...any end

        local a
        for b in bar() do
            a = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "(*error-type* | ~nil)?"
    } else {
      "any"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_for_in_loop_iterator_is_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_is_error() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a
        for b in bar do
            a = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "*error-type*?"
    } else {
      "*error-type*"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_for_in_loop_iterator_is_error_2 {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_is_error2() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function bar(c) return c end

        local a
        for b in bar() do
            a = b
        end
    "#,
      ),
      None,
    );

    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "*error-type*?",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    }
  }
}

mod type_infer_anyerror_for_in_loop_iterator_returns_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_returns_any() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function bar(): any
            return true
        end

        local a
        for b in bar do
            a = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "(*error-type* | ~nil)?",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    } else {
      let any_type = fixture.get_builtins().any_type;
      let a_type = fixture.require_type_string(&String::from("a"));
      assert_eq!(any_type, a_type);
    }
  }
}

mod type_infer_anyerror_for_in_loop_iterator_returns_any_2 {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_for_in_loop_iterator_returns_any2() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::fflag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function bar(): any
            return true
        end

        local a
        for b in bar() do
            a = b
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !fflag::DebugLuauForceOldSolver.get() {
      "(*error-type* | ~nil)?"
    } else {
      "any"
    };

    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_anyerror_indexing_error_type_does_not_produce_an_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_indexing_error_type_does_not_produce_an_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local originalReward = unknown.Parent.Reward:GetChildren()[1]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_intersection_of_any_can_have_props {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_intersection_of_any_can_have_props() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function foo(x: any, y)
    if x then
        return x._status
    end
    return y
end
"#,
      ),
      None,
    );

    assert_eq!(
      "(any, any) -> any",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_anyerror_length_of_error_type_does_not_produce_an_error {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_length_of_error_type_does_not_produce_an_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local l = #this_is_not_defined
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_metatable_of_any_can_be_a_table {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_metatable_of_any_can_be_a_table() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local T: any
T = {}
T.__index = T
function T.new(...)
    local self = {}
    setmetatable(self, T)
    self:construct(...)
    return self
end
function T:construct(index)
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_prop_access_on_any_with_other_options {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_prop_access_on_any_with_other_options() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(thing: any | string)
            local foo = thing.SomeRandomKey
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_quantify_any_does_not_bind_to_itself {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_quantify_any_does_not_bind_to_itself() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local A : any
        function A.B() end
        A:C()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let a_type = fixture.require_type_string(&String::from("A"));
    assert_eq!(a_type, fixture.get_builtins().any_type);
  }
}

mod type_infer_anyerror_replace_every_free_type_when_unifying_a_complex_function_with_any {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_replace_every_free_type_when_unifying_a_complex_function_with_any() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: any
        local b
        for _, i in pairs(a) do
            b = i
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_anyerror_table_of_any_calls {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_table_of_any_calls() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function testFunc(input: {any})
        end

        local v = {true}

        testFunc(v)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_anyerror_type_error_addition {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_type_error_addition() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local foo = makesandwich()
local bar = foo.nutrition + 100
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown global 'makesandwich'; consider assigning to it first",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_anyerror_union_of_types_regression_test {
  //! Source: `tests/TypeInfer.anyerror.test.cpp`

  #[test]
  fn type_infer_anyerror_union_of_types_regression_test() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local stat
stat = stat and tonumber(stat) or stat
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
