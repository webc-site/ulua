extern crate alloc;

mod type_infer_extern_types_assign_to_prop_of_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:185:type_infer_extern_types_assign_to_prop_of_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_assign_to_prop_of_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_assign_to_prop_of_class() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local v = Vector2.New(0, 5)
        v.X = 55
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_call_base_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:89:type_infer_extern_types_call_base_method`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_call_base_method

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_call_base_method() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local i = ChildClass.New()
        i:BaseMethod(41)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_call_instance_method {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:77:type_infer_extern_types_call_instance_method`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_call_instance_method

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_call_instance_method() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local i = ChildClass.New()
        local result = i:Method()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_string(&String::from("result"))
      )
    );
  }
}

mod type_infer_extern_types_call_method_of_a_child_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:66:type_infer_extern_types_call_method_of_a_child_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_call_method_of_a_child_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_call_method_of_a_child_class() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local m = ChildClass.StaticMethod()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("m")))
    );
  }
}

mod type_infer_extern_types_call_method_of_a_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:55:type_infer_extern_types_call_method_of_a_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_call_method_of_a_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_call_method_of_a_class() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local m = BaseClass.StaticMethod()
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("m")))
    );
  }
}

mod type_infer_extern_types_callable_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:594:type_infer_extern_types_callable_extern_types`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_callable_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_callable_extern_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x : CallableClass
        local y = x("testing")
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_extern_types_can_assign_to_prop_of_base_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:205:type_infer_extern_types_can_assign_to_prop_of_base_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_can_assign_to_prop_of_base_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_can_assign_to_prop_of_base_class() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c = ChildClass.New()
        c.BaseField = 444
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_can_assign_to_prop_of_base_class_using_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:225:type_infer_extern_types_can_assign_to_prop_of_base_class_using_string`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_can_assign_to_prop_of_base_class_using_string

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_can_assign_to_prop_of_base_class_using_string() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c = ChildClass.New()
        c["BaseField"] = 444
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_can_read_prop_of_base_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:195:type_infer_extern_types_can_read_prop_of_base_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_can_read_prop_of_base_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_can_read_prop_of_base_class() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c = ChildClass.New()
        local x = 1 + c.BaseField
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_can_read_prop_of_base_class_using_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:215:type_infer_extern_types_can_read_prop_of_base_class_using_string`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_can_read_prop_of_base_class_using_string

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_can_read_prop_of_base_class_using_string() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c = ChildClass.New()
        local x = 1 + c["BaseField"]
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_cannot_call_method_of_child_on_base_instance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:108:type_infer_extern_types_cannot_call_method_of_child_on_base_instance`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_cannot_call_method_of_child_on_base_instance

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_cannot_call_method_of_child_on_base_instance() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local i = BaseClass.New()
        i:Method()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_cannot_call_unknown_method_of_a_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:99:type_infer_extern_types_cannot_call_unknown_method_of_a_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_cannot_call_unknown_method_of_a_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_cannot_call_unknown_method_of_a_class() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local m = BaseClass.Nope()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_cannot_index_a_class_with_no_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:846:type_infer_extern_types_cannot_index_a_class_with_no_indexer`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record DynamicPropertyLookupOnExternTypesUnsafe (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_extern_types_cannot_index_a_class_with_no_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_cannot_index_a_class_with_no_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error,
      records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    };
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = BaseClass.New()

        local c = a[1]
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<DynamicPropertyLookupOnExternTypesUnsafe>(&result.errors[0]);
    assert!(
      err.is_some(),
      "expected DynamicPropertyLookupOnExternTypesUnsafe but got {:?}",
      result.errors[0]
    );
    assert_eq!(
      unsafe { (*fixture.base.base.builtin_types).error_type },
      fixture.base.base.require_type_string(&String::from("c"))
    );
  }
}

mod type_infer_extern_types_cannot_unify_class_instance_with_primitive {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:235:type_infer_extern_types_cannot_unify_class_instance_with_primitive`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_cannot_unify_class_instance_with_primitive

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_cannot_unify_class_instance_with_primitive() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local v = Vector2.New(0, 5)
        v = 444
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_class_type_mismatch_with_name_conflict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:477:type_infer_extern_types_class_type_mismatch_with_name_conflict`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_class_type_mismatch_with_name_conflict

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_class_type_mismatch_with_name_conflict() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local i = ChildClass.New()
type ChildClass = { x: number }
local a: ChildClass = i
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'ChildClass' from 'MainModule', but got 'ChildClass' from 'Test'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_extern_types_class_unification_type_mismatch_is_correct_order {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:413:type_infer_extern_types_class_unification_type_mismatch_is_correct_order`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_class_unification_type_mismatch_is_correct_order

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_class_unification_type_mismatch_is_correct_order() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local p: BaseClass
        local foo: number = p
        local foo2: BaseClass = 1
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'number', but got 'BaseClass'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Expected this to be 'BaseClass', but got 'number'",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:864:type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record Subtyping (Analysis/include/Luau/Subtyping.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c = BaseClass.New()

        function requiresNothing() end

        function onTouch(other)
            requiresNothing(other:BaseMethod(0))
            print(other.BaseField)
        end

        c.Touched:Connect(onTouch)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_detailed_class_unification_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:443:type_infer_extern_types_detailed_class_unification_error`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_extern_types_detailed_class_unification_error

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_detailed_class_unification_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function foo(v)
    return v.X :: number + string.len(v.Y)
end

local a: Vector2
local b = foo
b(a)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be '{ read X: unknown, read Y: string }', but got 'Vector2'; \n\
accessing `Y` results in `number` in the latter type and `string` in the former type, \
and `number` is not a subtype of `string`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = "Expected this to be '{- X: number, Y: string -}', but got 'Vector2'\n\
caused by:\n  Property 'Y' is not compatible.\n\
Expected this to be 'string', but got 'number'";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_extern_types_extern_type_check_key_becomes_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1037:type_infer_extern_types_extern_type_check_key_becomes_intersection`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_key_becomes_intersection

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_check_key_becomes_intersection() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            IsEnabled: string | boolean
        end
    "#,
      ),
      false,
    );

    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function update(foo: Foobar)
            assert(type(foo.IsEnabled) == "string")
            return foo
        end
    "#,
      ),
      None,
    );

    assert!(results.errors.is_empty(), "{:?}", results.errors);
    assert_eq!(
      "(Foobar) -> Foobar & { read IsEnabled: string }",
      to_string_type_id(fixture.base.require_type_string(&String::from("update")))
    );
  }
}

mod type_infer_extern_types_extern_type_check_key_becomes_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1012:type_infer_extern_types_extern_type_check_key_becomes_never`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_key_becomes_never

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_check_key_becomes_never() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            IsEnabled: string
        end

        declare extern type Bing with
            IsEnabled: number
        end
    "#,
      ),
      false,
    );

    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function update(foo: Foobar | Bing)
            assert(type(foo.IsEnabled) == "number")
            return foo
        end
    "#,
      ),
      None,
    );

    assert!(results.errors.is_empty(), "{:?}", results.errors);
    assert_eq!(
      "(Bing | Foobar) -> Bing",
      to_string_type_id(fixture.base.require_type_string(&String::from("update")))
    );
  }
}

mod type_infer_extern_types_extern_type_check_key_idempotent {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1079:type_infer_extern_types_extern_type_check_key_idempotent`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_key_idempotent

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_check_key_idempotent() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            IsEnabled: string
        end
    "#,
      ),
      false,
    );

    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function update(foo: Foobar)
            assert(type(foo.IsEnabled) == "string")
            return foo
        end
    "#,
      ),
      None,
    );

    assert!(results.errors.is_empty(), "{:?}", results.errors);
    assert_eq!(
      "(Foobar) -> Foobar",
      to_string_type_id(fixture.base.require_type_string(&String::from("update")))
    );
  }
}

mod type_infer_extern_types_extern_type_check_key_superset {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1058:type_infer_extern_types_extern_type_check_key_superset`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_key_superset

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_check_key_superset() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            IsEnabled: string
        end
    "#,
      ),
      false,
    );

    let results = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function update(foo: Foobar)
            assert(type(foo.IsEnabled) == "string" or type(foo.IsEnabled) == "number")
            return foo
        end
    "#,
      ),
      None,
    );

    assert!(results.errors.is_empty(), "{:?}", results.errors);
    assert_eq!(
      "(Foobar) -> Foobar",
      to_string_type_id(fixture.base.require_type_string(&String::from("update")))
    );
  }
}

mod type_infer_extern_types_extern_type_check_missing_key {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:946:type_infer_extern_types_extern_type_check_missing_key`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_missing_key

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_check_missing_key() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::unknown_property::UnknownProperty,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            Enabled: boolean
            function Disable(self): ()
        end
    "#,
      ),
      false,
    );

    let results = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local isUsingGamepad = false
        local isModalVisible = false

        local function updateGamepadCursor(foo: Foobar)
            local shouldEnableCursor = isUsingGamepad and isModalVisible

            if foo.IsEnabled == shouldEnableCursor then
                return
            end

            if not shouldEnableCursor then
                foo:Disable()
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    let err =
      get_type_error::<UnknownProperty>(&results.errors[0]).expect("expected UnknownProperty");
    assert_eq!("IsEnabled", err.key());
  }
}

mod type_infer_extern_types_extern_type_check_present_key_in_superclass {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:979:type_infer_extern_types_extern_type_check_present_key_in_superclass`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_present_key_in_superclass

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_check_present_key_in_superclass() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type FoobarParent with
            IsEnabled: boolean
        end
        declare extern type Foobar extends FoobarParent with
            function Disable(self): ()
        end
    "#,
      ),
      false,
    );

    let results = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local isUsingGamepad = false
        local isModalVisible = false

        local function updateGamepadCursor(foo: Foobar)
            local shouldEnableCursor = isUsingGamepad and isModalVisible

            if foo.IsEnabled == shouldEnableCursor then
                return
            end

            if not shouldEnableCursor then
                foo:Disable()
            end
        end
    "#,
      ),
      None,
    );

    assert!(results.errors.is_empty(), "{:?}", results.errors);
  }
}

mod type_infer_extern_types_extern_type_indexer_interactions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1167:type_infer_extern_types_extern_type_indexer_interactions`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_indexer_interactions

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_indexer_interactions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Container with
            [string | number]: boolean | string
        end

        declare extern type Point with
            X: number
            Y: number
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c: Container
        local p: Point
        local _: { [ string | number ]: boolean | string } = c -- OK
        local _: { [string]: boolean | string } = c -- not OK
        local _: { [ string | number ]: boolean } = c -- not OK
        local _: { [string]: number } = p -- not OK
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    for err in &result.errors {
      assert!(
        get_type_error::<TypeMismatch>(err).is_some(),
        "expected TypeMismatch, got {:?}",
        err
      );
    }
  }
}

mod type_infer_extern_types_extern_type_intersect_with_table_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1100:type_infer_extern_types_extern_type_intersect_with_table_indexer`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_intersect_with_table_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_intersect_with_table_indexer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
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
        local function f(obj: { [any]: any }, function_name: string)
            if typeof(obj) == "userdata" then
                local _ = obj[function_name]
            end
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "userdata & { [any]: any }",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 3,
        column: 28
      }))
    );
  }
}

mod type_infer_extern_types_extern_type_intersection_with_table_type_1 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1195:type_infer_extern_types_extern_type_intersection_with_table_type_1`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_intersection_with_table_type_1

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_intersection_with_table_type_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _normalize = ScopedFastFlag::new(&FFlag::LuauExternTypesNormalizeWithShapes, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Instance with
            name: string
        end

        declare extern type WithBrushes extends Instance with
            brushes: Instance
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function take(thing: WithBrushes & { brushes: Instance })
            print(thing)
            print(thing.brushes.name)
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "WithBrushes & { brushes: Instance }",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 2,
        column: 18
      }))
    );
  }
}

mod type_infer_extern_types_extern_type_intersection_with_table_type_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1226:type_infer_extern_types_extern_type_intersection_with_table_type_2`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_intersection_with_table_type_2

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_intersection_with_table_type_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _normalize = ScopedFastFlag::new(&FFlag::LuauExternTypesNormalizeWithShapes, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Instance with
            name: string
        end

        declare extern type WithBrushes extends Instance with
            brushes: Instance
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function take(thing: Instance & { brushes: Instance })
            print(thing)
            print(thing.brushes.name)
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Instance & { brushes: Instance }",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 2,
        column: 18
      }))
    );
  }
}

mod type_infer_extern_types_extern_type_is_not_subtype_of_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1135:type_infer_extern_types_extern_type_is_not_subtype_of_table`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_is_not_subtype_of_table

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_is_not_subtype_of_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Color3 with
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(c: Color3): { Color3 }
            return c
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Color3", to_string_type_id(err.given_type));
    assert_eq!("{Color3}", to_string_type_id(err.wanted_type));
  }
}

mod type_infer_extern_types_extern_type_overload {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1153:type_infer_extern_types_extern_type_overload`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_overload

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_overload() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Color3 with
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f : ((Color3) -> ()) & (({Color3}) -> ())
        local c: Color3
        f(c)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_extern_type_with_indexer_intersect_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1115:type_infer_extern_types_extern_type_with_indexer_intersect_table`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_type_with_indexer_intersect_table

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_type_with_indexer_intersect_table() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Foobar with
            [string]: unknown
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function update(obj: Foobar)
            assert(typeof(obj.Baz) == "number")
            return obj
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "(Foobar) -> Foobar & { read Baz: number }",
      to_string_type_id(fixture.base.require_type_string(&String::from("update")))
    );
  }
}

mod type_infer_extern_types_extern_types_can_have_overloaded_operators {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:263:type_infer_extern_types_extern_types_can_have_overloaded_operators`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_types_can_have_overloaded_operators

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_types_can_have_overloaded_operators() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = Vector2.New(1, 2)
        local b = Vector2.New(3, 4)
        local c = a + b
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "Vector2",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_extern_types_extern_types_without_overloaded_operators_cannot_be_added {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:276:type_infer_extern_types_extern_types_without_overloaded_operators_cannot_be_added`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_extern_types_without_overloaded_operators_cannot_be_added

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_extern_types_without_overloaded_operators_cannot_be_added() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = BaseClass.New()
        local b = BaseClass.New()
        local c = a + b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_function_arguments_are_covariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:287:type_infer_extern_types_function_arguments_are_covariant`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_function_arguments_are_covariant

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_function_arguments_are_covariant() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(b: BaseClass) end

        f(ChildClass.New())
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_higher_order_function_arguments_are_contravariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:298:type_infer_extern_types_higher_order_function_arguments_are_contravariant`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_higher_order_function_arguments_are_contravariant

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_higher_order_function_arguments_are_contravariant() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function apply(f: (BaseClass) -> ())
            f(ChildClass.New()) -- 2
        end

        apply(function (c: ChildClass) end) -- 5
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_higher_order_function_return_type_is_not_contravariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:326:type_infer_extern_types_higher_order_function_return_type_is_not_contravariant`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_higher_order_function_return_type_is_not_contravariant

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_higher_order_function_return_type_is_not_contravariant() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function apply(f: () -> BaseClass)
            return f()
        end

        apply(function ()
            return ChildClass.New()
        end)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_higher_order_function_return_values_are_covariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:311:type_infer_extern_types_higher_order_function_return_values_are_covariant`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_higher_order_function_return_values_are_covariant

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_higher_order_function_return_values_are_covariant() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function apply(f: () -> BaseClass)
            return f()
        end

        apply(function ()
            return ChildClass.New()
        end)
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:923:type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic() {
    use alloc::string::String;

    use ulua_analysis::enums::solver_mode::SolverMode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _luau_solver_off = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend().set_luau_solver_mode(SolverMode::New);

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local function ExitSeat(player, character, seat, weld)
    --Find vehicle model
    local model
    local newParent = seat
    repeat
        model = newParent
        newParent = model.Parent
    until newParent.ClassName ~= "Model"
    local part, _ = Raycast(seat.Position, dir, dist, {character, model})
end
"#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_index_instance_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:513:type_infer_extern_types_index_instance_property`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_extern_types_index_instance_property

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_index_instance_property() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function execute(object: BaseClass, name: string)
            print(object[name])
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Attempting a dynamic property access on type 'BaseClass' is unsafe and may cause exceptions at runtime",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_extern_types_index_instance_property_nonstrict {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:525:type_infer_extern_types_index_instance_property_nonstrict`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_index_instance_property_nonstrict

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_index_instance_property_nonstrict() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!nonstrict

        local function execute(object: BaseClass, name: string)
            print(object[name])
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_indexable_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:605:type_infer_extern_types_indexable_extern_types`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Index (Analysis/include/Luau/TypePath.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_indexable_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_indexable_extern_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let mut check = |source: &str| {
      fixture
        .base
        .base
        .check_string_optional_frontend_options(&String::from(source), None)
    };

    let result = check(
      r#"
            local x : IndexableClass
            local y = x.stringKey
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            local y = x["stringKey"]
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            local str : string
            local y = x[str]            -- Index with a non-const string
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            local y = x[7]              -- Index with a numeric key
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            x.stringKey = 42
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            x["stringKey"] = 42
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            local str : string
            x[str] = 42                 -- Index with a non-const string
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            x[1] = 42                   -- Index with a numeric key
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
      r#"
            local x : IndexableClass
            local y = x[true]
        "#,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'number | string', but got 'boolean';\n\
this is because\n\
\t* the 1st component of the union is `string`, and `boolean` is not a subtype of `string`\n\
\t* the 2nd component of the union is `number`, and `boolean` is not a subtype of `number`\n";
      ulua_unit_test::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
      assert_eq!(
        "Expected this to be 'number | string', but got 'boolean'; none of the union options are compatible",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = check(
      r#"
            local x : IndexableClass
            x[true] = 42
        "#,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'number | string', but got 'boolean';\n\
this is because\n\
\t * the 1st component of the union is `string`, and `boolean` is not a subtype of `string`\n\
\t * the 2nd component of the union is `number`, and `boolean` is not a subtype of `number`\n";
      ulua_unit_test::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
      assert_eq!(
        "Expected this to be 'number | string', but got 'boolean'; none of the union options are compatible",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = check(
      r#"
            local x : IndexableClass
            x.key = "string value"
        "#,
    );

    if FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = check(
      r#"
            local x : IndexableClass
            local str : string = x.key
        "#,
    );

    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[0])
    );

    let result = check(
      r#"
            local x : IndexableNumericKeyClass
            x.key = 1
        "#,
    );
    assert_eq!(
      "Key 'key' not found in external type 'IndexableNumericKeyClass'",
      to_string_type_error(&result.errors[0])
    );

    let result = check(
      r#"
            local x : IndexableNumericKeyClass
            x["key"] = 1
        "#,
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Key 'key' not found in external type 'IndexableNumericKeyClass'",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = check(
      r#"
            local x : IndexableNumericKeyClass
            local str : string
            x[str] = 1                  -- Index with a non-const string
        "#,
    );

    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );

    let result = check(
      r#"
            local x : IndexableNumericKeyClass
            local y = x.key
        "#,
    );
    assert_eq!(
      "Key 'key' not found in external type 'IndexableNumericKeyClass'",
      to_string_type_error(&result.errors[0])
    );

    let result = check(
      r#"
            local x : IndexableNumericKeyClass
            local y = x["key"]
        "#,
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Key 'key' not found in external type 'IndexableNumericKeyClass'",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
    }

    let result = check(
      r#"
            local x : IndexableNumericKeyClass
            local str : string
            local y = x[str]            -- Index with a non-const string
        "#,
    );

    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_extern_types_intersections_of_unions_of_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:489:type_infer_extern_types_intersections_of_unions_of_extern_types`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_intersections_of_unions_of_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_intersections_of_unions_of_extern_types() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x : (BaseClass | Vector2) & (ChildClass | AnotherChild)
        local y : (ChildClass | AnotherChild)
        x = y
        y = x
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_luau_analyze_cl_i_crashes_on_this_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:22:type_infer_extern_types_luau_analyze_cl_i_crashes_on_this_test`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_luau_analyze_cl_i_crashes_on_this_test

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_luau_analyze_cli_crashes_on_this_test() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local CircularQueue = {}
CircularQueue.__index = CircularQueue

function CircularQueue:new()
	local newCircularQueue = {
		head = nil,
	}
	setmetatable(newCircularQueue, CircularQueue)

	return newCircularQueue
end

function CircularQueue:push()
	local newListNode

	if self.head then
		newListNode = {
			prevNode = self.head.prevNode,
			nextNode = self.head,
		}
		newListNode.prevNode.nextNode = newListNode
		newListNode.nextNode.prevNode = newListNode
	end
end

return CircularQueue

    "#,
      ),
      None,
    );
  }
}

mod type_infer_extern_types_optional_class_casts_work_in_new_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:569:type_infer_extern_types_optional_class_casts_work_in_new_solver`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_extern_types_optional_class_casts_work_in_new_solver

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_optional_class_casts_work_in_new_solver() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: ChildClass }
        type B = { x: BaseClass }

        local a = { x = ChildClass.New() } :: A
        local opt_a = a :: A?
        local b = { x = BaseClass.New() } :: B
        local opt_b = b :: B?
        local b_from_a = a :: B
        local b_from_opt_a = opt_a :: B
        local opt_b_from_a = a :: B?
        local opt_b_from_opt_a = opt_a :: B?
        local a_from_b = b :: A
        local a_from_opt_b = opt_b :: A
        local opt_a_from_b = b :: A?
        local opt_a_from_opt_b = opt_b :: A?
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_optional_class_field_access_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:427:type_infer_extern_types_optional_class_field_access_error`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_extern_types_optional_class_field_access_error

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_optional_class_field_access_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local b: Vector2? = nil
local a = b.X + b.Z

b.X = 2 -- real Vector2.X is also read-only
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Value of type 'Vector2?' could be nil",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Value of type 'Vector2?' could be nil",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Key 'Z' not found in external type 'Vector2'",
      to_string_type_error(&result.errors[2])
    );
    assert_eq!(
      "Value of type 'Vector2?' could be nil",
      to_string_type_error(&result.errors[3])
    );
  }
}

mod type_infer_extern_types_read_write_class_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:795:type_infer_extern_types_read_write_class_properties`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_extern_types_read_write_class_properties

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_read_write_class_properties() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
        freeze::freeze, get_error::get_type_error, get_mutable_type::get_mutable_type_id,
        unfreeze::unfreeze,
      },
      records::{
        binding::Binding, extern_type::ExternType, property_type::Property,
        type_mismatch::TypeMismatch,
      },
      type_aliases::type_id::TypeId,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let (string_type, number_type) = {
      let frontend = fixture.get_frontend();
      let string_type = unsafe { (*frontend.builtin_types).string_type };
      let number_type = unsafe { (*frontend.builtin_types).number_type };

      let globals = &mut frontend.globals;
      unfreeze(globals.global_types_mut());

      let (_instance_type, workspace_type, script_type, part_type) = {
        let arena = globals.global_types_mut();
        let make_extern = |name: &str, parent: Option<TypeId>| ExternType {
          name: String::from(name),
          props: Default::default(),
          parent,
          metatable: None,
          tags: Default::default(),
          user_data: None,
          definition_module_name: String::from("Test"),
          definition_location: None,
          indexer: None,
          relation: None,
        };

        let instance_type = arena.add_type(make_extern("Instance", None));
        let instance =
          get_mutable_type_id::<ExternType>(instance_type).expect("expected Instance extern type");
        instance
          .props
          .insert(String::from("Parent"), Property::rw_type_id(instance_type));

        let workspace_type = arena.add_type(make_extern("Workspace", None));

        let script_type = arena.add_type(make_extern("Script", Some(instance_type)));
        let script =
          get_mutable_type_id::<ExternType>(script_type).expect("expected Script extern type");
        script.props.insert(
          String::from("Parent"),
          Property::rw_type_id_type_id(workspace_type, instance_type),
        );

        let part_type = arena.add_type(make_extern("Part", Some(instance_type)));
        let part = get_mutable_type_id::<ExternType>(part_type).expect("expected Part extern type");
        part.props.insert(
          String::from("BrickColor"),
          Property::rw_type_id(string_type),
        );
        part.props.insert(
          String::from("Parent"),
          Property::rw_type_id_type_id(workspace_type, instance_type),
        );

        (instance_type, workspace_type, script_type, part_type)
      };

      let workspace =
        get_mutable_type_id::<ExternType>(workspace_type).expect("expected Workspace extern type");
      workspace
        .props
        .insert(String::from("Script"), Property::readonly(script_type));
      workspace
        .props
        .insert(String::from("Part"), Property::readonly(part_type));

      add_global_binding_builtin_definitions_alt_b(
        globals,
        "script",
        Binding {
          type_id: script_type,
          location: Location::default(),
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );

      freeze(globals.global_types_mut());
      (string_type, number_type)
    };

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        script.Parent.Part.BrickColor = 0xFFFFFF
        script.Parent.Part.Parent = script
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 40,
        },
        end: Position {
          line: 1,
          column: 48,
        },
      },
      result.errors[0].location
    );

    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(string_type, tm.wanted_type);
    assert_eq!(number_type, tm.given_type);
  }
}

mod type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:387:type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(bar)
            bar.Y = 1 -- valid
            bar.x = 2 -- invalid, wanted 'X'
            bar.w = 2 -- invalid
        end

        local a: Vector2
        foo(a)
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Expected this to be '{ Y: number, w: number, x: number }', but got 'Vector2'",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Key 'w' not found in external type 'Vector2'",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Key 'x' not found in external type 'Vector2'.  Did you mean 'X'?",
        to_string_type_error(&result.errors[1])
      );
    }
  }
}

mod type_infer_extern_types_table_indexers_are_invariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:364:type_infer_extern_types_table_indexers_are_invariant`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_table_indexers_are_invariant

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_table_indexers_are_invariant() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: {[number]: BaseClass})
            a[1] = AnotherChild.New()
        end

        local t: {[number]: ChildClass}
        f(t) -- line 6.  Breaks soundness.

        function g(t: {[number]: ChildClass})
        end

        local t2: {[number]: BaseClass} = {BaseClass.New()}
        t2[1] = AnotherChild.New()
        g(t2) -- line 13.  Breaks soundness
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(6, result.errors[0].location.begin.line);
    assert_eq!(13, result.errors[1].location.begin.line);
  }
}

mod type_infer_extern_types_table_properties_are_invariant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:341:type_infer_extern_types_table_properties_are_invariant`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_table_properties_are_invariant

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_table_properties_are_invariant() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(a: {foo: BaseClass})
            a.foo = AnotherChild.New()
        end

        local t: {foo: ChildClass}
        f(t) -- line 6.  Breaks soundness.

        function g(t: {foo: ChildClass})
        end

        local t2: {foo: BaseClass} = {foo=BaseClass.New()}
        t2.foo = AnotherChild.New()
        g(t2) -- line 13.  Breaks soundness
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(6, result.errors[0].location.begin.line);
    assert_eq!(13, result.errors[1].location.begin.line);
  }
}

mod type_infer_extern_types_type_mismatch_invariance_required_for_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:538:type_infer_extern_types_type_mismatch_invariance_required_for_error`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_extern_types_type_mismatch_invariance_required_for_error

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_type_mismatch_invariance_required_for_error() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type A = { x: ChildClass }
type B = { x: BaseClass }

local a: A = { x = ChildClass.New() }
local b: B = a
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let expected = "Expected this to be 'B', but got 'A'; \n\
accessing `x` results in `ChildClass` in the latter type and `BaseClass` in the former type, \
and `ChildClass` is not exactly `BaseClass`";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = "Expected this to be exactly 'B', but got 'A'\n\
caused by:\n  Property 'x' is not compatible.\n\
Expected this to be exactly 'BaseClass', but got 'ChildClass'";
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_extern_types_unions_of_intersections_of_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:501:type_infer_extern_types_unions_of_intersections_of_extern_types`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_unions_of_intersections_of_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_unions_of_intersections_of_extern_types() {
    use alloc::string::String;

    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x : (BaseClass & ChildClass) | (BaseClass & AnotherChild) | (BaseClass & Vector2)
        local y : (ChildClass | AnotherChild)
        x = y
        y = x
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_extern_types_warn_when_prop_almost_matches {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:248:type_infer_extern_types_warn_when_prop_almost_matches`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_extern_types_warn_when_prop_almost_matches

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_warn_when_prop_almost_matches() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error,
      records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    };
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        Vector2.new(0, 0)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = get_type_error::<UnknownPropButFoundLikeProp>(&result.errors[0])
      .expect("expected UnknownPropButFoundLikeProp");
    assert_eq!(1, err.candidates().len());
    assert!(err.candidates().contains("New"));
  }
}

mod type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:118:type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function makeClone(o)
            return BaseClass.Clone(o)
        end

        local a = makeClone(ChildClass.New())
    "#,
      ),
      None,
    );

    assert_eq!(
      "BaseClass",
      to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:131:type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::extern_type_fixture::ExternTypeFixture;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function makeClone(o)
            return BaseClass.Clone(o)
        end

        type Oopsies = { BaseMethod: (Oopsies, number) -> ()}

        local oopsies: Oopsies = {
            BaseMethod = function (self: Oopsies, i: number)
                print('gadzooks!')
            end
        }

        makeClone(oopsies)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    assert_eq!("Oopsies", to_string_type_id(tm.given_type));
    assert_eq!("BaseClass", to_string_type_id(tm.wanted_type));
  }
}

mod type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class_using_new_solver {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:157:type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class_using_new_solver`
  //! Source: `tests/TypeInfer.externTypes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class_using_new_solver

  #[cfg(test)]
  #[test]
  fn type_infer_extern_types_we_can_report_when_someone_is_trying_to_use_a_table_rather_than_a_class_using_new_solver()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::extern_type_fixture::ExternTypeFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function makeClone(o)
            return BaseClass.Clone(o)
        end

        type Oopsies = { read BaseMethod: (Oopsies, number) -> ()}

        local oopsies: Oopsies = {
            BaseMethod = function (self: Oopsies, i: number)
                print('gadzooks!')
            end
        }

        makeClone(oopsies)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    assert_eq!("Oopsies", to_string_type_id(tm.given_type));
    assert_eq!("BaseClass", to_string_type_id(tm.wanted_type));
  }
}
