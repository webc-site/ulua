use ulua_common::fflag;
extern crate alloc;

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_cannot_call_primitives() {
  use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options("local foo = 5    foo()", None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    matches!(
      &result.errors[0].data,
      TypeErrorData::CannotCallNonFunction(_)
    ),
    "{:?}",
    result.errors[0]
  );
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_check_methods_of_number() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: number = 9999
        function x:y(z: number)
            local s: string = z
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Expected type table, got 'number' instead",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[1])
    );
  } else {
    assert_eq!(
      "Cannot add method to non-table type 'number'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[1])
    );
  }
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_properties_of_vectors() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local a = vector.create(1, 2, 3)
        local b = vector.create(4, 5, 6)

        local t1 = {
            a + b,
            a - b,
            a * 3,
            a * b,
            3 * b,
            a / 3,
            a / b,
            3 / b,
            a // 4,
            a // b,
            4 // b,
            -a,
        }
        local t2 = {
            a.x,
            a.y,
            a.z,
        }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_property_of_buffers() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local b = buffer.create(100)
        print(b.foo)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_singleton_types() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture_a = BuiltinsFixture::default();
  fixture_a.get_frontend();

  {
    let mut fixture_b = BuiltinsFixture::default();
    fixture_b.get_frontend();
  }

  let result = fixture_a
    .base
    .check_string_optional_frontend_options("local s: string = 'hello' local t = s:lower()", None);

  assert!(result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_string_function_indirect() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local s:string
        local l = s.lower
        local p = l(s)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_string("p"))
  );
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_string_index() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, type_aliases::type_error_data::TypeErrorData,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local s = "Hello, World!"
        local t = s[4]
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let TypeErrorData::NotATable(not_a_table) = &result.errors[0].data else {
    panic!("expected NotATable, got {:?}", result.errors[0]);
  };
  assert_eq!("string", to_string_type_id(not_a_table.ty));

  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.require_type_string("t"))
  );
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_string_length() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local s = "Hello, World!"
        local t = #s
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let number_type = fixture.get_builtins().number_type;
  assert_eq!(number_type, fixture.require_type_string("t"));
}

// Source: `tests/TypeInfer.primitives.test.cpp`
#[test]
fn type_infer_primitives_string_method() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local p = ("tacos"):len()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let ty = fixture.require_type_string("p");
  assert_eq!("number", to_string_type_id(ty));
}
