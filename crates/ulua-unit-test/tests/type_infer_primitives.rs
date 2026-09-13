use ulua_common::FFlag;
extern crate alloc;

mod type_infer_primitives_cannot_call_primitives {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:14:type_infer_primitives_cannot_call_primitives`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_primitives_cannot_call_primitives

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_cannot_call_primitives() {
    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result =
      fixture.check_string_optional_frontend_options(&String::from("local foo = 5    foo()"), None);

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
}

mod type_infer_primitives_check_methods_of_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:71:type_infer_primitives_check_methods_of_number`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_primitives_check_methods_of_number
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_check_methods_of_number() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: number = 9999
        function x:y(z: number)
            local s: string = z
        end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
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
}

mod type_infer_primitives_properties_of_vectors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:118:type_infer_primitives_properties_of_vectors`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_primitives_properties_of_vectors

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_properties_of_vectors() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_primitives_property_of_buffers {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:108:type_infer_primitives_property_of_buffers`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_primitives_property_of_buffers

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_property_of_buffers() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local b = buffer.create(100)
        print(b.foo)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_primitives_singleton_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:94:type_infer_primitives_singleton_types`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record BuiltinsFixture (tests/Fixture.h)
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_primitives_singleton_types

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_singleton_types() {
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture_a = BuiltinsFixture::default();
    fixture_a.get_frontend();

    {
      let mut fixture_b = BuiltinsFixture::default();
      fixture_b.get_frontend();
    }

    let result = fixture_a.base.check_string_optional_frontend_options(
      &String::from("local s: string = 'hello' local t = s:lower()"),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_primitives_string_function_indirect {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:59:type_infer_primitives_string_function_indirect`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
  //!   - translates_to -> rust_item type_infer_primitives_string_function_indirect

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_string_function_indirect() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s:string
        local l = s.lower
        local p = l(s)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("p")))
    );
  }
}

mod type_infer_primitives_string_index {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:33:type_infer_primitives_string_index`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record NotATable (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_primitives_string_index

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_string_index() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "Hello, World!"
        local t = s[4]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let TypeErrorData::NotATable(not_a_table) = &result.errors[0].data else {
      panic!("expected NotATable, got {:?}", result.errors[0]);
    };
    assert_eq!("string", to_string_type_id(not_a_table.ty));

    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
  }
}

mod type_infer_primitives_string_length {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:22:type_infer_primitives_string_length`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_primitives_string_length

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_string_length() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local s = "Hello, World!"
        local t = #s
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let number_type = fixture.get_builtins().number_type;
    assert_eq!(number_type, fixture.require_type_string(&String::from("t")));
  }
}

mod type_infer_primitives_string_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:49:type_infer_primitives_string_method`
  //! Source: `tests/TypeInfer.primitives.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_primitives_string_method

  #[cfg(test)]
  #[test]
  fn type_infer_primitives_string_method() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local p = ("tacos"):len()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.require_type_string(&String::from("p"));
    assert_eq!("number", to_string_type_id(ty));
  }
}
