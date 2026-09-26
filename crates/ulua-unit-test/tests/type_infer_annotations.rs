use ulua_analysis::type_aliases::module_name_type::ModuleName;
extern crate alloc;
use alloc::string::String;
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_analysis::{
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_builtin_definitions, first::first,
    follow_type, get_error::get_type_error, get_type, to_string_error::to_string_type_error,
    to_string_to_string::to_string_type_id,
  },
  records::{
    function_type::FunctionType, internal_compiler_error::InternalCompilerError,
    occurs_check_failed::OccursCheckFailed, table_type::TableType, type_mismatch::TypeMismatch,
  },
};
use ulua_ast::records::{location::Location, position::Position};
use ulua_common::fflag;
use ulua_unit_test::{
  functions::{is_in_arena::is_in_arena, type_error_data_ref::type_error_data_ref},
  records::{builtins_fixture::BuiltinsFixture, fixture::Fixture},
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_as_expr_does_not_propagate_type_info() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a = 55 :: any
        local b = a :: number
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!("any", to_string_type_id(fixture.require_type_string("a")));
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("b"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_as_expr_is_bidirectional() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a = 55 :: number?
        local b = a :: number
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("b"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_as_expr_warns_on_unrelated_cast() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a = 55 :: string
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Cannot cast 'number' into 'string' because the types are unrelated",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_assignment_also_checks_subtyping() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(): number?
            return nil
        end
        local x: number = 1
        local y: number? = f()
        x = y
        y = x
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_assignment_cannot_transform_a_table_property_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a = {x=0}
        a.x = "one"
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_assignments_are_checked_against_annotations() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: number = 1
        x = "two"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_assignments_to_annotated_parameters_are_checked() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(x: string)
            x = 0
            return x
        end
    "#,
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
    to_string_type_id(fixture.require_type_string("f"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_assignments_to_unannotated_parameters_can_transform_the_type() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(x)
            x = 0
            return x
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(unknown) -> number",
    to_string_type_id(fixture.require_type_string("f"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_builtin_types_are_not_exported() {
  let mut fixture = BuiltinsFixture::default();
  let any_type = {
    let frontend = fixture.get_frontend();
    frontend.builtin_types_ref().any_type
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
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/Main"), None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_cannot_use_nonexported_type() {
  let mut fixture = BuiltinsFixture::default();
  let any_type = {
    let frontend = fixture.get_frontend();
    frontend.builtin_types_ref().any_type
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
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/Main"), None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_check_multi_initialize() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: number, b: string = "one", 2
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(get_type_error::<TypeMismatch>(&result.errors[0]).is_some());
  assert!(get_type_error::<TypeMismatch>(&result.errors[1]).is_some());
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_cloned_interface_maintains_pointers_between_definitions() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        export type Record = { name: string, location: string }
        local a: Record = { name="Waldo", location="?????" }
        local b: Record = { name="Santa Claus", location="Maui" } -- FIXME

        return {a=a, b=b}
    "#,
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
  let exports_table = get_type::get::<TableType>(exports_type).expect("expected return table");

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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_corecursive_types_error_on_tight_loop() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = B
        type B = A

        local aa:A
        local bb:B
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
    "expected OccursCheckFailed: {:?}",
    result.errors[0]
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_define_generic_type_alias() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Array<T> = {[number]: T}
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_duplicate_type_param_name() {
  use ulua_analysis::records::duplicate_generic_parameter::DuplicateGenericParameter;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Oopsies<T, T> = {a: T, b: T}
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let dgp = type_error_data_ref::<DuplicateGenericParameter>(&result.errors[0])
    .expect("expected DuplicateGenericParameter");
  assert_eq!("T", dgp.parameter_name());
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_for_loop_counter_annotation() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options(" for i: number = 0, 50 do end ", None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_for_loop_counter_annotation_is_checked() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options(" for i: string = 0, 10 do end ", None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_annotation() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: (number, string) -> number
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let f_type = fixture.require_type_string("f");
  let ftv = get_type::get::<FunctionType>(follow_type::follow(f_type));
  assert!(ftv.is_some(), "expected function type");
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_annotation_with_a_defined_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local f: (number, number) -> string = function(a: number, b: number) return "" end
    "#,
    None,
  );

  let f_type = fixture.require_type_string("f");
  let ftv = get_type::get::<FunctionType>(follow_type::follow(f_type));
  assert!(ftv.is_some(), "expected function type");
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_parameter_annotations_are_checked() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function double(x: number)
            return 2
        end

        local four = double("two")
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_parameters_can_have_annotations() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function double(x: number)
            return 2
        end

        local four = double(2)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_return_annotation_should_continuously_parse_return_annotation_and_checked()
 {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(): (number, string) -> (number) -> nil
            return function(a: number, b: string): (number) -> nil
                return function(a: number): nil
                    return 1
                end
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_return_annotation_should_disambiguate_into_function_type_return_and_checked()
 {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(): (number, string) -> nil
            return function(a: number, b: string): number return 1 end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_function_return_multret_annotations_are_checked() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(): (number, string)
            return 1, 2
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_generic_aliases_are_cloned_properly() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        export type Array<T> = { [number]: T }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = unsafe { &*fixture.get_main_module(false) };
  let array = module
    .exported_type_bindings
    .get("Array")
    .expect("expected exported type Array");

  assert_eq!(1, array.type_params().len());

  let array_table = get_type::get::<TableType>(array.r#type()).expect("expected table type");

  assert_eq!(0, array_table.props.len());
  let indexer = array_table
    .indexer
    .as_ref()
    .expect("expected table indexer");

  assert!(is_in_arena(array.r#type(), &module.interface_types));
  assert_eq!(array.type_params()[0].ty(), indexer.index_result_type);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_infer_type_of_value_a_via_typeof_with_assignment() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a
        local b: typeof(a) = 1

        a = "foo"
    "#,
    None,
  );

  let (expected_location, expected_wanted, expected_given) =
    if !fflag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_string("a"))
      );
      assert_eq!("nil", to_string_type_id(fixture.require_type_string("b")));

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
        to_string_type_id(fixture.require_type_string("a"))
      );
      assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string("b"))
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_initializers_are_checked_against_annotations() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options("local a: number = \"Hello Types!\"", None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_instantiate_type_fun_should_not_trip_rbxassert() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Foo<T> = typeof(function(x) return x end)
        local foo: Foo<number>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_instantiation_clone_has_to_follow() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        export type t8<t8> = (t0)&(<t0...>((true)|(any))->"")
        export type t0<t0> = ({})&({_:{[any]:number},})
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_interface_types_belong_to_interface_arena() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        export type A = {field: number}

        local n: A = {field = 551}

        return {n=n}
    "#,
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
  let exports_table = get_type::get::<TableType>(exports_type).expect("expected return table");

  let n = exports_table
    .props
    .get("n")
    .and_then(|prop| prop.read_ty)
    .expect("expected n property read type");

  assert!(is_in_arena(n, &module.interface_types));
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_luau_ice_is_not_special_without_the_flag() {
  let _sffs = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, false);
  let mut fixture = Fixture::fixture_bool(false);

  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local a: _luau_ice = 55
    "#,
    None,
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag() {
  use ulua_common::assert_call_handler;
  use ulua_unit_test::records::assertion_catcher::AssertionCatcher;

  let _sffs = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let _ac = AssertionCatcher::new();
  let mut fixture = Fixture::fixture_bool(false);

  let result = catch_unwind(AssertUnwindSafe(|| {
    fixture.check_string_optional_frontend_options(
      r#"
        local a: _luau_ice = 55
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag_handler() {
  use alloc::rc::Rc;
  use core::cell::Cell;

  let _sffs = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let caught = Rc::new(Cell::new(false));
  let caught_for_handler = caught.clone();
  let mut fixture = Fixture::fixture_bool(false);

  fixture.get_frontend().ice_handler.on_internal_error = Some(Rc::new(move |_| {
    caught_for_handler.set(true);
  }));

  let result = catch_unwind(AssertUnwindSafe(|| {
    fixture.check_string_optional_frontend_options(
      r#"
        local a: _luau_ice = 55
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_luau_print_incomplete() {
  let _sffs = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: _luau_print
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "_luau_print requires one generic parameter",
    to_string_type_error(&result.errors[0])
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_luau_print_is_magic_if_the_flag_is_set() {
  use core::sync::atomic::{AtomicUsize, Ordering};

  use ulua_analysis::functions::{
    reset_print_line::reset_print_line, set_print_line::set_print_line,
  };

  static OUTPUT_COUNT: AtomicUsize = AtomicUsize::new(0);

  fn capture_print_line(_line: &str) {
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

  let _sffs = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local a: _luau_print<typeof(math.abs)>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(1, OUTPUT_COUNT.load(Ordering::SeqCst));
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_luau_print_is_not_special_without_the_flag() {
  let _sffs = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: _luau_print<number>
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_multi_assign_checks_against_annotations() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: number, b: string = 1, "two"
        a, b = "one", 2
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_occurs_check_on_cyclic_intersection_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type T = T & T
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
    "expected OccursCheckFailed: {:?}",
    result.errors[0]
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_occurs_check_on_cyclic_union_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type T = T | T
        local x : T
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
    "expected OccursCheckFailed: {:?}",
    result.errors[0]
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_pulling_a_type_from_value_dont_falsely_create_occurs_check_failed() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f(x)
            type T = typeof(x)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_react_use_state_partial_annotation() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type BasicStateAction<S> = ((S) -> S) | S
        type Dispatch<A> = (A) -> ()

        local useState: <S>( (() -> S) | S ) -> (S, Dispatch<BasicStateAction<S>>) = nil :: any

        local v: number, setV = useState(0)
        local w, setW = useState(0 :: number?)
        local x, setX = useState(0)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(((number) -> number) | number) -> ()",
    to_string_type_id(fixture.base.require_type_string("setV"))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.base.require_type_string("w"))
  );
  assert_eq!(
    "((((number?) -> number?) | number)?) -> ()",
    to_string_type_id(fixture.base.require_type_string("setW"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_string("x"))
  );
  assert_eq!(
    "(((number) -> number) | number) -> ()",
    to_string_type_id(fixture.base.require_type_string("setX"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_respect_partially_annotated_type_packs_1() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function f(): (number, string)
            return 42, "huh"
        end

        local a: number, b = f()

        print(math.abs(b))
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(err.wanted_type));
  assert_eq!("string", to_string_type_id(err.given_type));
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_respect_partially_annotated_type_packs_2() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function f(): (number, boolean, string)
            return 42, true, "huh"
        end

        local a: number, b, c: string = f()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "boolean",
    to_string_type_id(fixture.base.require_type_string("b"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_self_referential_type_alias() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type O = { x: number, incr: (O) -> number }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let o_type = fixture.lookup_type("O").expect("expected O alias");
  let o_type = follow_type::follow(o_type);
  let o_table = get_type::get::<TableType>(o_type).expect("expected O table type");

  let incr = o_table.props.get("incr").expect("expected incr property");
  let incr_read_ty = incr.read_ty.expect("expected incr read type");

  let incr_func = get_type::get::<FunctionType>(incr_read_ty).expect("expected incr function type");
  let first_arg = first(incr_func.arg_types(), false).expect("expected first argument");

  assert_eq!(o_type, follow_type::follow(first_arg));
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_successful_check() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local a: number, b: string = 1, "two"
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_table_annotation() {
  use ulua_analysis::records::primitive_type::PrimitiveType;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: {a: number, b: string} = {a=2, b="three"}
        local y = x.a
        local z = x.b
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let y_type = fixture.require_type_string("y");
  assert_eq!(
    Some(PrimitiveType::NUMBER),
    fixture.get_primitive_type(y_type)
  );
  let z_type = fixture.require_type_string("z");
  assert_eq!(
    Some(PrimitiveType::STRING),
    fixture.get_primitive_type(z_type)
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_too_many_type_params() {
  use ulua_analysis::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Callback<A, R> = (A) -> (boolean, R)
        local a: Callback<number, number, string> = function(i) return true, 4 end
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_two_type_params() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Map<K, V> = {[K]: V}
        local m: Map<string, number> = {}
        local a = m['foo']
        local b = m[9]                  -- error here
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(4, result.errors[0].location.begin.line);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_type_alias_always_resolve_to_a_real_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = B
        type B = C
        type C = number

        local aa:A
    "#,
    None,
  );

  let f_type = fixture.require_type_string("aa");
  let number_type = fixture.get_builtins().number_type;
  assert_eq!(number_type, follow_type::follow(f_type));
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_type_alias_b_should_check_with_another_aliases_until_a_non_aliased_type()
{
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = number
        type B = A
        local b: B = 10
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_type_alias_should_alias_to_number() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = number
        local a: A = 10
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_type_aliasing_to_number_should_not_check_given_a_string() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type A = number
        local a: A = "fail"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_type_annotations_inside_function_bodies() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function get_message()
            local message = 'That smarts!' :: string
            return message
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_type_assertion_expr() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options("local a = 55 :: any", None);

  assert_eq!("any", to_string_type_id(fixture.require_type_string("a")));
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_typeof_expr() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function id(i) return i end

        local m: typeof(id(77))
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("m"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_typeof_variable_type_annotation_should_return_its_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local foo = { bar = "baz" }

        type Foo = typeof(foo)

        local foo2: Foo
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    fixture.require_type_string("foo"),
    fixture.require_type_string("foo2")
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_unifier_3_supertail_covariant_with_sub() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local function fib(n)
            return n + fib(n)
        end
    "#,
    None,
  );

  assert_eq!(
    "<a>(a) -> t1 where t1 = add<a, t1>",
    to_string_type_id(fixture.require_type_string("fib"))
  );
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_unknown_type_reference_generates_error() {
  use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: IDoNotExist
    "#,
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

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_use_generic_type_alias() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type Array<T> = {[number]: T}   -- 1
        local p: Array<number> = {}     -- 2
        p[1] = 5                        -- 3 OK
        p[2] = 'hello'                  -- 4 Error.
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(4, result.errors[0].location.begin.line);
  assert!(get_type_error::<TypeMismatch>(&result.errors[0]).is_some());
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_use_type_required_from_another_file() {
  let mut fixture = BuiltinsFixture::default();
  let any_type = {
    let frontend = fixture.get_frontend();
    frontend.builtin_types_ref().any_type
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
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/Main"), None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.annotations.test.cpp`.
#[test]
fn type_infer_annotations_variable_type_is_supertype() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x: number = 1
        local y: number? = x
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.annotations.test.cpp:175`
#[test]
fn type_infer_annotations_function_return_annotations_are_checked() {
  use ulua_analysis::{
    functions::{flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_pack, get_type},
    records::function_type::FunctionType,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function fifty(): any
            return 55
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let fifty_type = get_type::get::<FunctionType>(fixture.require_type_string("fifty"))
    .expect("expected FunctionType");
  let ret_pack = follow_type_pack::follow(fifty_type.ret_types());
  let (head, _tail) = flatten_type_pack_id(ret_pack);
  assert_eq!(1, head.len());

  let any_type = {
    let frontend = fixture.get_frontend();
    frontend.builtin_types_ref().any_type
  };
  assert_eq!(any_type, follow_type::follow(head[0]));
}

// 缺口（未移植，对照 `tests/TypeInfer.annotations.test.cpp`，共 4 例）：
// - unknown_generic_type_pack_reference_generates_one_error（:254）、
//   unknown_generic_type_pack_vararg_generates_one_error（:271）、
//   unknown_generic_type_pack_in_explicit_instantiation_generates_one_error（:288）
//   ——依赖 FFlag `LuauStrictVisitInstantiatedType`，ulua 未同步该 flag（与
//   aliases/modules 组阻清单同类）。
// - unifier3_supertail_covariant_with_sub（:1026）——cpp CHECK 逐字串
//   `"<T>(T) -> t1 where t1 = add<T, t1>"`；rust 求解器具名泛型变量输出为
//   `"<a>(a) -> t1 where t1 = add<a, t1>"`（命名策略分歧），逐字一致断言不可
//   表达，改生产码超出补缺票授权范围。
