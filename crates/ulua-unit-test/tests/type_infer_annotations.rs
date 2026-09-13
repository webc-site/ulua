extern crate alloc;

mod type_infer_annotations_as_expr_does_not_propagate_type_info {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_as_expr_does_not_propagate_type_info() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 55 :: any
        local b = a :: number
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_annotations_as_expr_is_bidirectional {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_as_expr_is_bidirectional() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 55 :: number?
        local b = a :: number
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_annotations_as_expr_warns_on_unrelated_cast {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_as_expr_warns_on_unrelated_cast() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      to_string_error::to_string_type_error, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 55 :: string
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot cast 'number' into 'string' because the types are unrelated",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_annotations_assignment_also_checks_subtyping {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_assignment_also_checks_subtyping() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(): number?
            return nil
        end
        local x: number = 1
        local y: number? = f()
        x = y
        y = x
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 6,
          column: 12,
        },
        end: Position {
          line: 6,
          column: 13,
        },
      },
      result.errors[0].location
    );
  }
}

mod type_infer_annotations_assignment_cannot_transform_a_table_property_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_assignment_cannot_transform_a_table_property_type() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = {x=0}
        a.x = "one"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 14,
        },
        end: Position {
          line: 2,
          column: 19,
        },
      },
      result.errors[0].location
    );
  }
}

mod type_infer_annotations_assignments_are_checked_against_annotations {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_assignments_are_checked_against_annotations() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: number = 1
        x = "two"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_assignments_to_annotated_parameters_are_checked {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_assignments_to_annotated_parameters_are_checked() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: string)
            x = 0
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 16,
        },
        end: Position {
          line: 2,
          column: 17,
        },
      },
      result.errors[0].location
    );
    assert_eq!(
      "(string) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_annotations_assignments_to_unannotated_parameters_can_transform_the_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_assignments_to_unannotated_parameters_can_transform_the_type() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x)
            x = 0
            return x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(unknown) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_annotations_builtin_types_are_not_exported {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_builtin_types_are_not_exported() {
    use alloc::string::String;

    use ulua_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let any_type = {
      let frontend = fixture.get_frontend();
      unsafe { (*frontend.builtin_types).any_type }
    };
    add_global_binding_builtin_definitions(
      &mut fixture.get_frontend().globals,
      "script",
      any_type,
      "@test",
    );

    fixture.base.file_resolver.source.insert(
      String::from("Modules/Main"),
      String::from(
        r#"
        --!strict
        local Test = require(script.Parent.Thing)

        export type Foo = { [any]: Test.number }

        return Test
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("Modules/Thing"),
      String::from(
        r#"
        --!strict

        return {}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/Main"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_cannot_use_nonexported_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_cannot_use_nonexported_type() {
    use alloc::string::String;

    use ulua_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let any_type = {
      let frontend = fixture.get_frontend();
      unsafe { (*frontend.builtin_types).any_type }
    };
    add_global_binding_builtin_definitions(
      &mut fixture.get_frontend().globals,
      "script",
      any_type,
      "@test",
    );

    fixture.base.file_resolver.source.insert(
      String::from("Modules/Main"),
      String::from(
        r#"
        --!strict
        local Test = require(script.Parent.Thing)

        export type Foo = { [any]: Test.TestType }

        return Test
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("Modules/Thing"),
      String::from(
        r#"
        --!strict

        type TestType = {bar: boolean}

        return {}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/Main"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_check_multi_initialize {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_check_multi_initialize() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number, b: string = "one", 2
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(get_type_error::<TypeMismatch>(&result.errors[0]).is_some());
    assert!(get_type_error::<TypeMismatch>(&result.errors[1]).is_some());
  }
}

mod type_infer_annotations_cloned_interface_maintains_pointers_between_definitions {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_cloned_interface_maintains_pointers_between_definitions() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        first::first, get_type_alt_j::get_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{table_type::TableType, to_string_options::ToStringOptions},
    };
    use ulua_unit_test::{functions::is_in_arena::is_in_arena, records::fixture::Fixture};

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Record = { name: string, location: string }
        local a: Record = { name="Waldo", location="?????" }
        local b: Record = { name="Santa Claus", location="Maui" } -- FIXME

        return {a=a, b=b}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let record_type = module
      .exported_type_bindings
      .get("Record")
      .expect("expected exported Record")
      .r#type();

    let exports_type = first(module.return_type, true).expect("expected module return type");
    let exports_table = get_type_id::<TableType>(exports_type).expect("expected return table");

    let a_type = exports_table
      .props
      .get("a")
      .and_then(|prop| prop.read_ty)
      .expect("expected a property read type");
    let b_type = exports_table
      .props
      .get("b")
      .and_then(|prop| prop.read_ty)
      .expect("expected b property read type");

    assert!(is_in_arena(record_type, &module.interface_types));
    assert!(is_in_arena(a_type, &module.interface_types));
    assert!(is_in_arena(b_type, &module.interface_types));

    let mut record_opts = ToStringOptions::new(true);
    let mut a_opts = ToStringOptions::new(true);
    let mut b_opts = ToStringOptions::new(true);
    let record_string = to_string_type_id_to_string_options(record_type, &mut record_opts);

    assert_eq!(
      record_string,
      to_string_type_id_to_string_options(a_type, &mut a_opts)
    );
    assert_eq!(
      record_string,
      to_string_type_id_to_string_options(b_type, &mut b_opts)
    );
  }
}

mod type_infer_annotations_corecursive_types_error_on_tight_loop {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_corecursive_types_error_on_tight_loop() {
    use alloc::string::String;

    use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = B
        type B = A

        local aa:A
        local bb:B
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
      "expected OccursCheckFailed: {:?}",
      result.errors[0]
    );
  }
}

mod type_infer_annotations_define_generic_type_alias {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_define_generic_type_alias() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Array<T> = {[number]: T}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let main_module = fixture.get_main_module(false);
    assert!(!main_module.is_null(), "expected main module");
    let main_module = unsafe { &*main_module };
    assert!(main_module.has_module_scope());

    let scope = main_module.get_module_scope();
    let tf = scope
      .private_type_bindings
      .get("Array")
      .expect("expected Array private type binding");
    assert_eq!(1, tf.type_params().len());
  }
}

mod type_infer_annotations_duplicate_type_param_name {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_duplicate_type_param_name() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_generic_parameter::DuplicateGenericParameter;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Oopsies<T, T> = {a: T, b: T}
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let dgp = type_error_data_ref::<DuplicateGenericParameter>(&result.errors[0])
      .expect("expected DuplicateGenericParameter");
    assert_eq!("T", dgp.parameter_name());
  }
}

mod type_infer_annotations_for_loop_counter_annotation {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_for_loop_counter_annotation() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(" for i: number = 0, 50 do end "),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_for_loop_counter_annotation_is_checked {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_for_loop_counter_annotation_is_checked() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(" for i: string = 0, 10 do end "),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_function_annotation {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_annotation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: (number, string) -> number
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    let ftv = get_type_id::<FunctionType>(follow_type_id(f_type));
    assert!(ftv.is_some(), "expected function type");
  }
}

mod type_infer_annotations_function_annotation_with_a_defined_function {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_annotation_with_a_defined_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: (number, number) -> string = function(a: number, b: number) return "" end
    "#,
      ),
      None,
    );

    let f_type = fixture.require_type_string(&String::from("f"));
    let ftv = get_type_id::<FunctionType>(follow_type_id(f_type));
    assert!(ftv.is_some(), "expected function type");
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_function_parameter_annotations_are_checked {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_parameter_annotations_are_checked() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function double(x: number)
            return 2
        end

        local four = double("two")
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_function_parameters_can_have_annotations {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_parameters_can_have_annotations() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function double(x: number)
            return 2
        end

        local four = double(2)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_function_return_annotation_should_continuously_parse_return_annotation_and_checked {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_return_annotation_should_continuously_parse_return_annotation_and_checked()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(): (number, string) -> (number) -> nil
            return function(a: number, b: string): (number) -> nil
                return function(a: number): nil
                    return 1
                end
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_function_return_annotation_should_disambiguate_into_function_type_return_and_checked {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_return_annotation_should_disambiguate_into_function_type_return_and_checked()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(): (number, string) -> nil
            return function(a: number, b: string): number return 1 end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_function_return_multret_annotations_are_checked {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_function_return_multret_annotations_are_checked() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(): (number, string)
            return 1, 2
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_generic_aliases_are_cloned_properly {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_generic_aliases_are_cloned_properly() {
    use alloc::string::String;

    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::table_type::TableType};
    use ulua_unit_test::{functions::is_in_arena::is_in_arena, records::fixture::Fixture};

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Array<T> = { [number]: T }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let array = module
      .exported_type_bindings
      .get("Array")
      .expect("expected exported type Array");

    assert_eq!(1, array.type_params().len());

    let array_table = get_type_id::<TableType>(array.r#type()).expect("expected table type");

    assert_eq!(0, array_table.props.len());
    let indexer = array_table
      .indexer
      .as_ref()
      .expect("expected table indexer");

    assert!(is_in_arena(array.r#type(), &module.interface_types));
    assert_eq!(array.type_params()[0].ty(), indexer.index_result_type);
  }
}

mod type_infer_annotations_infer_type_of_value_a_via_typeof_with_assignment {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_infer_type_of_value_a_via_typeof_with_assignment() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a
        local b: typeof(a) = 1

        a = "foo"
    "#,
      ),
      None,
    );

    let (expected_location, expected_wanted, expected_given) =
      if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
          "string?",
          to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
          "nil",
          to_string_type_id(fixture.require_type_string(&String::from("b")))
        );

        (
          Location {
            begin: Position {
              line: 2,
              column: 29,
            },
            end: Position {
              line: 2,
              column: 30,
            },
          },
          fixture.get_builtins().nil_type,
          fixture.get_builtins().number_type,
        )
      } else {
        assert_eq!(
          "number",
          to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
          "number",
          to_string_type_id(fixture.require_type_string(&String::from("b")))
        );

        (
          Location {
            begin: Position {
              line: 4,
              column: 12,
            },
            end: Position {
              line: 4,
              column: 17,
            },
          },
          fixture.get_builtins().number_type,
          fixture.get_builtins().string_type,
        )
      };

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(expected_location, result.errors[0].location);

    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(expected_wanted, tm.wanted_type);
    assert_eq!(expected_given, tm.given_type);
  }
}

mod type_infer_annotations_initializers_are_checked_against_annotations {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_initializers_are_checked_against_annotations() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local a: number = \"Hello Types!\""),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_instantiate_type_fun_should_not_trip_rbxassert {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_instantiate_type_fun_should_not_trip_rbxassert() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Foo<T> = typeof(function(x) return x end)
        local foo: Foo<number>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_instantiation_clone_has_to_follow {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_instantiation_clone_has_to_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type t8<t8> = (t0)&(<t0...>((true)|(any))->"")
        export type t0<t0> = ({})&({_:{[any]:number},})
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_interface_types_belong_to_interface_arena {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_interface_types_belong_to_interface_arena() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{first::first, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::{functions::is_in_arena::is_in_arena, records::fixture::Fixture};

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type A = {field: number}

        local n: A = {field = 551}

        return {n=n}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let a = module
      .exported_type_bindings
      .get("A")
      .expect("expected exported type A");

    assert!(is_in_arena(a.r#type(), &module.interface_types));
    assert!(!is_in_arena(
      a.r#type(),
      fixture.get_frontend().globals.global_types_mut()
    ));

    let exports_type = first(module.return_type, true).expect("expected module return type");
    let exports_table = get_type_id::<TableType>(exports_type).expect("expected return table");

    let n = exports_table
      .props
      .get("n")
      .and_then(|prop| prop.read_ty)
      .expect("expected n property read type");

    assert!(is_in_arena(n, &module.interface_types));
  }
}

mod type_infer_annotations_luau_ice_is_not_special_without_the_flag {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_luau_ice_is_not_special_without_the_flag() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, false);
    let mut fixture = Fixture::fixture_bool(false);

    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: _luau_ice = 55
    "#,
      ),
      None,
    );
  }
}

mod type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag() {
    use alloc::string::String;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::{FFlag, assert_call_handler};
    use ulua_unit_test::{
      records::{assertion_catcher::AssertionCatcher, fixture::Fixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let _ac = AssertionCatcher::new();
    let mut fixture = Fixture::fixture_bool(false);

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        local a: _luau_ice = 55
    "#,
        ),
        None,
      );
    }));

    let panic = result.expect_err("expected InternalCompilerError");
    assert!(
      panic.downcast_ref::<InternalCompilerError>().is_some(),
      "expected InternalCompilerError panic payload"
    );

    if AssertionCatcher::tripped() != 1 {
      unsafe {
        assert_call_handler(
          c"1 == AssertionCatcher::tripped".as_ptr(),
          c"TypeInfer.annotations.test.cpp".as_ptr(),
          line!() as i32,
          c"type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag".as_ptr(),
        );
      }
    }
  }
}

mod type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag_handler {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag_handler() {
    use alloc::{rc::Rc, string::String};
    use core::cell::Cell;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let caught = Rc::new(Cell::new(false));
    let caught_for_handler = caught.clone();
    let mut fixture = Fixture::fixture_bool(false);

    fixture.get_frontend().ice_handler.on_internal_error = Some(Rc::new(move |_| {
      caught_for_handler.set(true);
    }));

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture.check_string_optional_frontend_options(
        &String::from(
          r#"
        local a: _luau_ice = 55
    "#,
        ),
        None,
      );
    }));

    let panic = result.expect_err("expected InternalCompilerError");
    assert!(
      panic.downcast_ref::<InternalCompilerError>().is_some(),
      "expected InternalCompilerError panic payload"
    );
    assert!(caught.get());
  }
}

mod type_infer_annotations_luau_print_incomplete {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_luau_print_incomplete() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: _luau_print
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "_luau_print requires one generic parameter",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_annotations_luau_print_is_magic_if_the_flag_is_set {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_luau_print_is_magic_if_the_flag_is_set() {
    use alloc::string::String;
    use core::sync::atomic::{AtomicUsize, Ordering};

    use ulua_analysis::functions::{
      reset_print_line::reset_print_line, set_print_line::set_print_line,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    static OUTPUT_COUNT: AtomicUsize = AtomicUsize::new(0);

    extern "C-unwind" fn capture_print_line(_line: &String) {
      OUTPUT_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    struct ResetPrintLineGuard;

    impl Drop for ResetPrintLineGuard {
      fn drop(&mut self) {
        reset_print_line();
      }
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    OUTPUT_COUNT.store(0, Ordering::SeqCst);
    set_print_line(Some(capture_print_line));
    let _guard = ResetPrintLineGuard;

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: _luau_print<typeof(math.abs)>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(1, OUTPUT_COUNT.load(Ordering::SeqCst));
  }
}

mod type_infer_annotations_luau_print_is_not_special_without_the_flag {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_luau_print_is_not_special_without_the_flag() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: _luau_print<number>
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_multi_assign_checks_against_annotations {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_multi_assign_checks_against_annotations() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number, b: string = 1, "two"
        a, b = "one", 2
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 15,
        },
        end: Position {
          line: 2,
          column: 20,
        },
      },
      result.errors[0].location
    );
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 22,
        },
        end: Position {
          line: 2,
          column: 23,
        },
      },
      result.errors[1].location
    );
  }
}

mod type_infer_annotations_occurs_check_on_cyclic_intersection_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_occurs_check_on_cyclic_intersection_type() {
    use alloc::string::String;

    use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = T & T
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
      "expected OccursCheckFailed: {:?}",
      result.errors[0]
    );
  }
}

mod type_infer_annotations_occurs_check_on_cyclic_union_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_occurs_check_on_cyclic_union_type() {
    use alloc::string::String;

    use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = T | T
        local x : T
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
      type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
      "expected OccursCheckFailed: {:?}",
      result.errors[0]
    );
  }
}

mod type_infer_annotations_pulling_a_type_from_value_dont_falsely_create_occurs_check_failed {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_pulling_a_type_from_value_dont_falsely_create_occurs_check_failed() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x)
            type T = typeof(x)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_react_use_state_partial_annotation {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_react_use_state_partial_annotation() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        type BasicStateAction<S> = ((S) -> S) | S
        type Dispatch<A> = (A) -> ()

        local useState: <S>( (() -> S) | S ) -> (S, Dispatch<BasicStateAction<S>>) = nil :: any

        local v: number, setV = useState(0)
        local w, setW = useState(0 :: number?)
        local x, setX = useState(0)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(((number) -> number) | number) -> ()",
      to_string_type_id(fixture.base.require_type_string(&String::from("setV")))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.base.require_type_string(&String::from("w")))
    );
    assert_eq!(
      "((((number?) -> number?) | number)?) -> ()",
      to_string_type_id(fixture.base.require_type_string(&String::from("setW")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "(((number) -> number) | number) -> ()",
      to_string_type_id(fixture.base.require_type_string(&String::from("setX")))
    );
  }
}

mod type_infer_annotations_respect_partially_annotated_type_packs_1 {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_respect_partially_annotated_type_packs_1() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (number, string)
            return 42, "huh"
        end

        local a: number, b = f()

        print(math.abs(b))
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(err.wanted_type));
    assert_eq!("string", to_string_type_id(err.given_type));
  }
}

mod type_infer_annotations_respect_partially_annotated_type_packs_2 {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_respect_partially_annotated_type_packs_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (number, boolean, string)
            return 42, true, "huh"
        end

        local a: number, b, c: string = f()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
  }
}

mod type_infer_annotations_self_referential_type_alias {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_self_referential_type_alias() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{first::first, follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{function_type::FunctionType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type O = { x: number, incr: (O) -> number }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let o_type = fixture
      .lookup_type(&String::from("O"))
      .expect("expected O alias");
    let o_type = follow_type_id(o_type);
    let o_table = get_type_id::<TableType>(o_type).expect("expected O table type");

    let incr = o_table.props.get("incr").expect("expected incr property");
    let incr_read_ty = incr.read_ty.expect("expected incr read type");

    let incr_func = get_type_id::<FunctionType>(incr_read_ty).expect("expected incr function type");
    let first_arg = first(incr_func.arg_types(), false).expect("expected first argument");

    assert_eq!(o_type, follow_type_id(first_arg));
  }
}

mod type_infer_annotations_successful_check {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_successful_check() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number, b: string = 1, "two"
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_table_annotation {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_table_annotation() {
    use alloc::string::String;

    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: {a: number, b: string} = {a=2, b="three"}
        local y = x.a
        local z = x.b
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let y_type = fixture.require_type_string(&String::from("y"));
    assert_eq!(
      Some(PrimitiveType::NUMBER),
      fixture.get_primitive_type(y_type)
    );
    let z_type = fixture.require_type_string(&String::from("z"));
    assert_eq!(
      Some(PrimitiveType::STRING),
      fixture.get_primitive_type(z_type)
    );
  }
}

mod type_infer_annotations_too_many_type_params {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_too_many_type_params() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Callback<A, R> = (A) -> (boolean, R)
        local a: Callback<number, number, string> = function(i) return true, 4 end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(2, result.errors[0].location.begin.line);

    let igpc = type_error_data_ref::<IncorrectGenericParameterCount>(&result.errors[0])
      .expect("expected IncorrectGenericParameterCount");
    assert_eq!(3, igpc.actual_parameters());
    assert_eq!(2, igpc.type_fun().type_params().len());
    assert_eq!("Callback", igpc.name());

    assert_eq!(
      "Generic type 'Callback<A, R>' expects 2 type arguments, but 3 are specified",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_annotations_two_type_params {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_two_type_params() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Map<K, V> = {[K]: V}
        local m: Map<string, number> = {}
        local a = m['foo']
        local b = m[9]                  -- error here
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(4, result.errors[0].location.begin.line);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_annotations_type_alias_always_resolve_to_a_real_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_type_alias_always_resolve_to_a_real_type() {
    use alloc::string::String;

    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = B
        type B = C
        type C = number

        local aa:A
    "#,
      ),
      None,
    );

    let f_type = fixture.require_type_string(&String::from("aa"));
    let number_type = fixture.get_builtins().number_type;
    assert_eq!(number_type, follow_type_id(f_type));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_type_alias_b_should_check_with_another_aliases_until_a_non_aliased_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_type_alias_b_should_check_with_another_aliases_until_a_non_aliased_type()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = number
        type B = A
        local b: B = 10
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_type_alias_should_alias_to_number {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_type_alias_should_alias_to_number() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = number
        local a: A = 10
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_type_aliasing_to_number_should_not_check_given_a_string {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_type_aliasing_to_number_should_not_check_given_a_string() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = number
        local a: A = "fail"
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_type_annotations_inside_function_bodies {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_type_annotations_inside_function_bodies() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function get_message()
            local message = 'That smarts!' :: string
            return message
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_type_assertion_expr {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_type_assertion_expr() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result =
      fixture.check_string_optional_frontend_options(&String::from("local a = 55 :: any"), None);

    assert_eq!(
      "any",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod type_infer_annotations_typeof_expr {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_typeof_expr() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id(i) return i end

        local m: typeof(id(77))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("m")))
    );
  }
}

mod type_infer_annotations_typeof_variable_type_annotation_should_return_its_type {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_typeof_variable_type_annotation_should_return_its_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo = { bar = "baz" }

        type Foo = typeof(foo)

        local foo2: Foo
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      fixture.require_type_string(&String::from("foo")),
      fixture.require_type_string(&String::from("foo2"))
    );
  }
}

mod type_infer_annotations_unifier_3_supertail_covariant_with_sub {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_unifier_3_supertail_covariant_with_sub() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function fib(n)
            return n + fib(n)
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<a>(a) -> t1 where t1 = add<a, t1>",
      to_string_type_id(fixture.require_type_string(&String::from("fib")))
    );
  }
}

mod type_infer_annotations_unknown_type_reference_generates_error {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_unknown_type_reference_generates_error() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: IDoNotExist
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 17,
        },
        end: Position {
          line: 1,
          column: 28,
        },
      },
      result.errors[0].location
    );
    assert_eq!(String::from("MainModule"), result.errors[0].module_name);

    let error =
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("IDoNotExist", error.name());
    assert_eq!(Context::Type, error.context());
  }
}

mod type_infer_annotations_use_generic_type_alias {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_use_generic_type_alias() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Array<T> = {[number]: T}   -- 1
        local p: Array<number> = {}     -- 2
        p[1] = 5                        -- 3 OK
        p[2] = 'hello'                  -- 4 Error.
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(4, result.errors[0].location.begin.line);
    assert!(get_type_error::<TypeMismatch>(&result.errors[0]).is_some());
  }
}

mod type_infer_annotations_use_type_required_from_another_file {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_use_type_required_from_another_file() {
    use alloc::string::String;

    use ulua_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let any_type = {
      let frontend = fixture.get_frontend();
      unsafe { (*frontend.builtin_types).any_type }
    };
    add_global_binding_builtin_definitions(
      &mut fixture.get_frontend().globals,
      "script",
      any_type,
      "@test",
    );

    fixture.base.file_resolver.source.insert(
      String::from("Modules/Main"),
      String::from(
        r#"
        --!strict
        local Test = require(script.Parent.Thing)

        export type Foo = { [any]: Test.TestType }

        return Test
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("Modules/Thing"),
      String::from(
        r#"
        --!strict

        export type TestType = {bar: boolean}

        return {}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/Main"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_annotations_variable_type_is_supertype {
  //! Ported from `tests/TypeInfer.annotations.test.cpp`.
  use super::*;

  #[cfg(test)]
  #[test]
  fn type_infer_annotations_variable_type_is_supertype() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: number = 1
        local y: number? = x
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
