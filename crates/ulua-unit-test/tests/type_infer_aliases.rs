extern crate alloc;

mod type_infer_aliases_alias_expands_to_bare_reference_to_imported_type {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_alias_expands_to_bare_reference_to_imported_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        export type Object = {[string]: any}
        return {}
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local A = require(script.Parent.A)

        type Object = A.Object
        type ReadOnly<T> = T

        local function f(): ReadOnly<Object>
            return nil :: any
        end
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_another_thing_from_roact {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_another_thing_from_roact() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Map<K, V> = { [K]: V }
        type Set<T> = { [T]: boolean }

        type FiberRoot = {
            pingCache: Map<Wakeable, (Set<any> | Map<Wakeable, Set<any>>)> | nil,
        }

        type Wakeable = {
            andThen: (self: Wakeable) -> nil | Wakeable,
        }

        local function attachPingListener(root: FiberRoot, wakeable: Wakeable, lanes: number)
            local pingCache: Map<Wakeable, (Set<any> | Map<Wakeable, Set<any>>)> | nil = root.pingCache
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_basic_alias {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_basic_alias() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = number
        local x: T = 1
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_aliases_bound_type_in_alias_segfault {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_bound_type_in_alias_segfault() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        type Map<T, V> = {[K]: V}
        function foo:bar(): Config<any, any> end
        type Config<TSource, TContext> = Map<TSource, TContext> & { fields: FieldConfigMap<any, any>}
        export type FieldConfig<TSource, TContext, TArgs> = {[string]: any}
        export type FieldConfigMap<TSource, TContext> = Map<string, FieldConfig<TSource, TContext>>
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_cannot_create_cyclic_type_with_unknown_module {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_cannot_create_cyclic_type_with_unknown_module() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type AAA = B.AAA
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Unknown type 'B.AAA'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_aliases_cannot_steal_hoisted_type_alias {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_cannot_steal_hoisted_type_alias() {
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
        local x: T = "foo"
        type T = number
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected_location = if !FFlag::DebugLuauForceOldSolver.get() {
      Location {
        begin: Position {
          line: 1,
          column: 21,
        },
        end: Position {
          line: 1,
          column: 26,
        },
      }
    } else {
      Location {
        begin: Position { line: 1, column: 8 },
        end: Position {
          line: 1,
          column: 26,
        },
      }
    };
    assert_eq!(expected_location, result.errors[0].location);

    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.wanted_type));
    assert_eq!("string", to_string_type_id(tm.given_type));
  }
}

mod type_infer_aliases_cli_38393_recursive_intersection_oom {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_cli_38393_recursive_intersection_oom() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function _(l0:(t0)&((t0)&(((t0)&((t0)->()))->(typeof(_),typeof(# _)))),l39,...):any
        end
        type t0<t0> = ((typeof(_))&((t0)&(((typeof(_))&(t0))->typeof(_))),{n163:any,})->(any,typeof(_))
        _(_)
    "#,
        ),
        None,
    );
  }
}

mod type_infer_aliases_corecursive_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_corecursive_aliases() {
    use alloc::string::String;

    use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo<T> = Bar<T>
type Bar<T> = Foo<T>
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<OccursCheckFailed>(&result.errors[0])
      .expect("expected OccursCheckFailed");
  }
}

mod type_infer_aliases_corecursive_function_types {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_corecursive_function_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = () -> (number, B)
        type B = () -> (string, A)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "t1 where t1 = () -> (number, () -> (string, t1))",
      to_string_type_id(fixture.require_type_alias(&String::from("A")))
    );
    assert_eq!(
      "t1 where t1 = () -> (string, () -> (number, t1))",
      to_string_type_id(fixture.require_type_alias(&String::from("B")))
    );
  }
}

mod type_infer_aliases_corecursive_types_generic {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_corecursive_types_generic() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(
      r#"
        type A<T> = {v:T, b:B<T>}
        type B<T> = {v:T, a:A<T>}

        function f(a: A<number>)
            return a
        end
    "#,
    );
    let expected = String::from(
      r#"
        type A<T> = {v:T, b:B<T>}
        type B<T> = {v:T, a:A<T>}

        function f(a: A<number>): A<number>
            return a
        end
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
    let result = fixture.check_string_optional_frontend_options(&code, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_cyclic_function_type_in_type_alias {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_cyclic_function_type_in_type_alias() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type F = () -> F?
        local function f()
            return f
        end

        local g: F = f
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "t1 where t1 = () -> t1?",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod type_infer_aliases_cyclic_types_of_named_table_fields_do_not_expand_when_stringified {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_cyclic_types_of_named_table_fields_do_not_expand_when_stringified() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type Node = { Parent: Node?; }

        function f(node: Node)
            node.Parent = 1
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Node?", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_aliases_default_pack_parameter {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_default_pack_parameter() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<A... = (number, string)> = { fn: (A...) -> () }
        local x: T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T<number, string>",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_aliases_default_type_parameter {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_default_type_parameter() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<A = number, B = string> = { a: A, b: B }
        local x: T<string> = { a = "foo", b = "bar" }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T<string, string>",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_aliases_dependent_generic_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_dependent_generic_aliases() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type T<a> = { v: a }
        type U<a> = { t: T<a> }
        local x: U<number> = { t = { v = 123 } }
        local bad: U<number> = { t = { v = "foo" } }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 4,
          column: 43,
        },
        end: Position {
          line: 4,
          column: 48,
        },
      },
      result.errors[0].location
    );
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_aliases_do_not_quantify_unresolved_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_do_not_quantify_unresolved_aliases() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict

        local KeyPool = {}

        local function newkey(pool: KeyPool, index)
            return {}
        end

        function newKeyPool()
            local pool = {
                available = {} :: {Key},
            }

            return setmetatable(pool, KeyPool)
        end

        export type KeyPool = typeof(newKeyPool())
        export type Key = typeof(newkey(newKeyPool(), 1))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_dont_allow_redefining_builtin_types {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_dont_allow_redefining_builtin_types() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _disallow_redefining =
      ScopedFastFlag::new(&FFlag::LuauDisallowRedefiningBuiltinTypes, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type number = string
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
      .expect("expected DuplicateTypeDefinition");
  }
}

mod type_infer_aliases_dont_lose_track_of_pending_expansion_types_after_substitution {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_dont_lose_track_of_pending_expansion_types_after_substitution() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/ReactCurrentDispatcher"),
      String::from(
        r#"
        export type BasicStateAction<S> = ((S) -> S) | S
        export type Dispatch<A> = (A) -> ()

        export type Dispatcher = {
            useState: <S>(initialState: (() -> S) | S) -> (S, Dispatch<BasicStateAction<S>>),
        }

        return {}
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("game/React/React/ReactHooks"),
      String::from(
        r#"
        local RCD = require(script.Parent.Parent.Parent.ReactCurrentDispatcher)

        local function resolveDispatcher(): RCD.Dispatcher
            return (nil :: any) :: RCD.Dispatcher
        end

        function useState<S>(
            initialState: (() -> S) | S
        ): (S, RCD.Dispatch<RCD.BasicStateAction<S>>)
            local dispatcher = resolveDispatcher()
            return dispatcher.useState(initialState)
        end
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(
        &String::from("game/React/React/ReactHooks"),
        None,
      );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_dont_stop_typechecking_after_reporting_duplicate_type_definition {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_dont_stop_typechecking_after_reporting_duplicate_type_definition() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = number
        type A = string -- Redefinition of type 'A', previously defined at line 1
        local foo: string = 1 -- "Type 'number' could not be converted into 'string'"
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_evaluating_generic_default_type_for_symbol_before_definition_is_an_error {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_evaluating_generic_default_type_for_symbol_before_definition_is_an_error() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type B<T = typeof(A)> = unknown
local A = {}
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  }
}

mod type_infer_aliases_evaluating_generic_default_type_pack_for_symbol_before_definition_is_an_error {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_evaluating_generic_default_type_pack_for_symbol_before_definition_is_an_error()
   {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type B<T... = ...typeof(A)> = unknown
local A = {}
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  }
}

mod type_infer_aliases_evaluating_generic_default_type_pack_shouldnt_ice {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_evaluating_generic_default_type_pack_shouldnt_ice() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local A = {}
type B<T... = ...typeof(A)> = unknown
"#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    }
  }
}

mod type_infer_aliases_evaluating_generic_default_type_shouldnt_ice {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_evaluating_generic_default_type_shouldnt_ice() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local A = {}
type B<T = typeof(A)> = unknown
"#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    }
  }
}

mod type_infer_aliases_export_type_and_type_alias_are_duplicates {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_export_type_and_type_alias_are_duplicates() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Foo = number
        type Foo = number
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let dtd = type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
      .expect("expected DuplicateTypeDefinition");
    assert_eq!("Foo", dtd.name());
  }
}

mod type_infer_aliases_exported_alias_location_is_accessible_on_module {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_exported_alias_location_is_accessible_on_module() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type Value = string
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let module = unsafe { &*fixture.get_main_module(false) };
    let tfun = module
      .exported_type_bindings
      .get("Value")
      .expect("expected exported type Value");
    assert_eq!(
      Some(Location {
        begin: Position { line: 1, column: 8 },
        end: Position {
          line: 1,
          column: 34,
        },
      }),
      tfun.definition_location()
    );
  }
}

mod type_infer_aliases_exported_type_function_location_is_accessible_on_module {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_exported_type_function_location_is_accessible_on_module() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type function Apply()
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let module = unsafe { &*fixture.get_main_module(false) };
    let tfun = module
      .exported_type_bindings
      .get("Apply")
      .expect("expected exported type function Apply");
    assert_eq!(
      Some(Location {
        begin: Position { line: 1, column: 8 },
        end: Position {
          line: 2,
          column: 11,
        },
      }),
      tfun.definition_location()
    );
  }
}

mod type_infer_aliases_forward_declared_alias_is_not_clobbered_by_prior_unification_with_any {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_forward_declared_alias_is_not_clobbered_by_prior_unification_with_any() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function x()
            local y: FutureType = {}::any
            return 1
        end
        type FutureType = { foo: typeof(x()) }
        local d: FutureType = { smth = true } -- missing error, 'd' is resolved to 'any'
    "#,
      ),
      None,
    );

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ foo: number }",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("d")),
        &mut opts
      )
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_free_variables_from_typeof_in_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_free_variables_from_typeof_in_aliases() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x) return x[1] end
        -- x has type X? for a free type variable X
        local x = f ({})
        type ContainsFree<a> = { this: a, that: typeof(x) }
        type ContainsContainsFree = { that: ContainsFree<number> }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_fuzzer_bug_doesnt_crash {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_fuzzer_bug_doesnt_crash() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type t0 = (t0<t0...>)
"#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_fuzzer_cursed_type_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_fuzzer_cursed_type_aliases() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type t1<t0...> = t4<t0...>
        export type t4<t2, t0...> = t4<t0...>
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_fuzzer_more_cursed_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_fuzzer_more_cursed_aliases() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
export type t138 = t0<t138>
export type t0<t0,t10,t10,t109> = t0
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_general_require_multi_assign {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_general_require_multi_assign() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("workspace/A"),
      String::from(
        r#"
        export type myvec2 = {x: number, y: number}
        return {}
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("workspace/B"),
      String::from(
        r#"
        export type myvec3 = {x: number, y: number, z: number}
        return {}
    "#,
      ),
    );
    fixture.base.file_resolver.source.insert(
      String::from("workspace/C"),
      String::from(
        r#"
        local Foo, Bar = require(workspace.A), require(workspace.B)

        local a: Foo.myvec2
        local b: Bar.myvec3
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("workspace/C"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let a_type_id = fixture
      .base
      .require_type_module_name_string("workspace/C", &String::from("a"));
    let a_type =
      get_type_id::<TableType>(follow_type_id(a_type_id)).expect("expected table type for a");
    assert_eq!(2, a_type.props.len());

    let b_type_id = fixture
      .base
      .require_type_module_name_string("workspace/C", &String::from("b"));
    let b_type =
      get_type_id::<TableType>(follow_type_id(b_type_id)).expect("expected table type for b");
    assert_eq!(3, b_type.props.len());
  }
}

mod type_infer_aliases_generic_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_generic_aliases() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
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
        type T<a> = { v: a }
        local x: T<number> = { v = 123 }
        local y: T<string> = { v = "foo" }
        local bad: T<number> = { v = "foo" }
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 4,
          column: 37,
        },
        end: Position {
          line: 4,
          column: 42,
        },
      },
      result.errors[0].location
    );
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_aliases_generic_param_remap {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_generic_param_remap() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(
      r#"
        -- An example of a forwarded use of a type that has different type arguments than parameters
        type A<T,U> = {t:T, u:U, next:A<U,T>?}
        local aa:A<number,string> = { t = 5, u = 'hi', next = { t = 'lo', u = 8 } }
        local bb = aa
    "#,
    );
    let expected = String::from(
      r#"

        type A<T,U> = {t:T, u:U, next:A<U,T>?}
        local aa:A<number,string> = { t = 5, u = 'hi', next = { t = 'lo', u = 8 } }
        local bb:A<number,string>=aa
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
    let result = fixture.check_string_optional_frontend_options(&code, None);
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_generic_typevars_are_not_considered_to_escape_their_scope_if_they_are_reused_in_multiple_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_generic_typevars_are_not_considered_to_escape_their_scope_if_they_are_reused_in_multiple_aliases()
   {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Array<T> = {T}
        type Exclude<T, V> = T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_gh_1632_no_infinite_recursion_in_normalization {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_gh_1632_no_infinite_recursion_in_normalization() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Node<T> = {
            value: T,
            next: Node<T>?,
            -- remove `prev`, solves issue
            prev: Node<T>?,
        };

        type List<T> = {
            head: Node<T>?
        }

        local function IsFront(list: List<any>, nodeB: Node<any>)
            -- remove if statement below, solves issue
            if (list.head == nodeB) then
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_it_is_ok_to_shadow_user_defined_alias {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_it_is_ok_to_shadow_user_defined_alias() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = number

        do
            type T = string
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mismatched_generic_pack_type_param {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mismatched_generic_pack_type_param() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<A...> = (A) -> ()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Variadic type parameter 'A...' is used as a regular generic type; consider changing 'A...' to 'A' in the generic argument list",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 24,
        },
        end: Position {
          line: 1,
          column: 25,
        },
      },
      result.errors[0].location
    );
  }
}

mod type_infer_aliases_mismatched_generic_type_param {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mismatched_generic_type_param() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<A> = (A...) -> ()
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Generic type 'A' is used as a variadic type parameter; consider changing 'A' to 'A...' in the generic argument list",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 21,
        },
        end: Position {
          line: 1,
          column: 25,
        },
      },
      result.errors[0].location
    );
  }
}

mod type_infer_aliases_module_export_free_type_leak {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_module_export_free_type_leak() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function get()
    return function(obj) return true end
end

export type f = typeof(get())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_module_export_wrapped_free_type_leak {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_module_export_wrapped_free_type_leak() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function get()
    return {a = 1, b = function(obj) return true end}
end

export type f = typeof(get())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_aliases() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type T = { f: number, g: U }
        type U = { h: number, i: T? }
        local x: T = { f = 37, g = { h = 5, i = nil } }
        x.g.i = x
        local y: T = { f = 3, g = { h = 5, i = nil } }
        y.g.i = y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_generic_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_generic_aliases() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type T<a> = { f: a, g: U<a> }
        type U<a> = { h: a, i: T<a>? }
        local x: T<number> = { f = 37, g = { h = 5, i = nil } }
        x.g.i = x
        local y: T<string> = { f = "hi", g = { h = "lo", i = nil } }
        y.g.i = y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_types_errors {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_types_errors() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      copy_errors::copy_errors, freeze::freeze, to_string_error::to_string_type_error,
      unfreeze::unfreeze,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        type T<a> = { f: a, g: U<a> }
        type U<b> = { h: b, i: T<b>? }
        local x: T<number> = { f = 37, g = { h = 5, i = nil } }
        x.g.i = x
        local y: T<string> = { f = "hi", g = { h = 5, i = nil } }
        y.g.i = y
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let module = unsafe { &mut *fixture.get_main_module(false) };
    unfreeze(&mut module.interface_types);
    copy_errors(&mut module.errors, &mut module.interface_types, unsafe {
      &*fixture.builtin_types
    });
    freeze(&mut module.interface_types);
    module.internal_types.clear();
    module.ast_types.clear();

    for error in &module.errors {
      let error_string = to_string_type_error(error);
      assert!(
        !error_string.contains("VALUELESS"),
        "unexpected VALUELESS in {error_string}"
      );
    }
  }
}

mod type_infer_aliases_mutually_recursive_types_restriction_not_ok_1 {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_types_restriction_not_ok_1() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        -- OK because forwarded types are used with their parameters.
        type Tree<T> = { data: T, children: Forest<T> }
        type Forest<T> = {Tree<{T}>}
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_types_restriction_not_ok_2 {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_types_restriction_not_ok_2() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        -- Not OK because forwarded types are used with different types than their parameters.
        type Forest<T> = {Tree<{T}>}
        type Tree<T> = { data: T, children: Forest<T> }
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_types_restriction_ok {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_types_restriction_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Tree<T> = { data: T, children: Forest<T> }
        type Forest<T> = {Tree<T>}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_types_swapsies_not_ok {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_types_swapsies_not_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Tree1<T,U> = { data: T, children: {Tree2<U,T>} }
        type Tree2<T,U> = { data: U, children: {Tree1<T,U>} }
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_mutually_recursive_types_swapsies_ok {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_mutually_recursive_types_swapsies_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Tree1<T,U> = { data: T, children: {Tree2<U,T>} }
        type Tree2<U,T> = { data: U, children: {Tree1<T,U>} }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_names_are_ascribed {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_names_are_ascribed() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { x: number }
        local x: T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
  }
}

mod type_infer_aliases_non_recursive_aliases_that_reuse_a_generic_name {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_non_recursive_aliases_that_reuse_a_generic_name() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Array<T> = { [number]: T }
        type Tuple<T, V> = Array<T | V>

        local p: Tuple<number, string>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{number | string}",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("p")),
        &mut opts
      )
    );
  }
}

mod type_infer_aliases_recursive_type_alias_bad_pack_use_warns {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_recursive_type_alias_bad_pack_use_warns() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error,
      records::{
        generic_error::GenericError, occurs_check_failed::OccursCheckFailed,
        swapped_generic_type_parameter::SwappedGenericTypeParameter,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo<T> = Foo<T...>
"#,
      ),
      None,
    );

    assert_eq!(5, result.errors.len(), "{:?}", result.errors);
    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<GenericError>(error).is_some()),
      "expected GenericError in {:?}",
      result.errors
    );
    assert_eq!(
      "Generic type 'Foo<T>' expects 1 type argument, but none are specified",
      to_string_type_error(&result.errors[4])
    );
    type_error_data_ref::<OccursCheckFailed>(&result.errors[1])
      .expect("expected OccursCheckFailed");
    let swapped = type_error_data_ref::<SwappedGenericTypeParameter>(&result.errors[2])
      .expect("expected SwappedGenericTypeParameter");
    assert_eq!("T", swapped.name);
  }
}

mod type_infer_aliases_recursive_type_alias_warns {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_recursive_type_alias_warns() {
    use alloc::string::String;

    use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo<T> = Foo<T>
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<OccursCheckFailed>(&result.errors[0])
      .expect("expected OccursCheckFailed");
  }
}

mod type_infer_aliases_recursive_types_restriction_not_ok {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_recursive_types_restriction_not_ok() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        -- this would be an infinite type if we allowed it
        type Tree<T> = { data: T, children: {Tree<{T}>} }
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_recursive_types_restriction_ok {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_recursive_types_restriction_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Tree<T> = { data: T, children: {Tree<T>} }
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_report_shadowed_aliases {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_report_shadowed_aliases() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_error::to_string_type_error, records::primitive_type::PrimitiveType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type MyString = string
        type string = number
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Redefinition of type 'string'",
      to_string_type_error(&result.errors[0])
    );

    let t1 = fixture
      .lookup_type(&String::from("MyString"))
      .expect("expected MyString");
    assert_eq!(Some(PrimitiveType::STRING), fixture.get_primitive_type(t1));

    let t2 = fixture
      .lookup_type(&String::from("string"))
      .expect("expected string");
    assert_eq!(Some(PrimitiveType::STRING), fixture.get_primitive_type(t2));
  }
}

mod type_infer_aliases_reported_location_is_correct_when_type_alias_are_duplicates {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_reported_location_is_correct_when_type_alias_are_duplicates() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = string
        type B = number
        type C = string
        type B = number
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let dtd = type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
      .expect("expected DuplicateTypeDefinition");
    assert_eq!("B", dtd.name());
    let previous_location = dtd
      .previous_location()
      .expect("expected previous duplicate location");
    assert_eq!(3, previous_location.begin.line + 1);
  }
}

mod type_infer_aliases_saturate_to_first_type_pack {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_saturate_to_first_type_pack() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T<A, B, C...> = { fn: (A, B) -> C... }
        local x: T<string, number, string, boolean>
        local f = x.fn
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "T<string, number, string, boolean>",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "(string, number) -> (string, boolean)",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_aliases_should_also_occurs_check {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_should_also_occurs_check() {
    use alloc::string::String;

    use ulua_analysis::records::occurs_check_failed::OccursCheckFailed;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo<T> = Foo<T> | string
"#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<OccursCheckFailed>(&result.errors[0])
      .expect("expected OccursCheckFailed");
  }
}

mod type_infer_aliases_stringify_optional_parameterized_alias {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_stringify_optional_parameterized_alias() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Node<T> = { value: T, child: Node<T>? }

        local function visitor<T>(node: Node<T>?)
            local a: Node<T>

            if node then
                a = node.child -- Observe the output of the error message.
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let e = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Node<T>?", to_string_type_id(e.given_type));
    assert_eq!("Node<T>", to_string_type_id(e.wanted_type));
  }
}

mod type_infer_aliases_stringify_type_alias_of_recursive_template_table_type {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_stringify_type_alias_of_recursive_template_table_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Table<T> = { a: T }
        type Wrapped = Table<Wrapped>
        local l: Wrapped = 2
        "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Wrapped", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_aliases_stringify_type_alias_of_recursive_template_table_type_2 {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_stringify_type_alias_of_recursive_template_table_type_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Table<T> = { a: T }
        type Wrapped = (Table<Wrapped>) -> string
        local l: Wrapped = 2
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
      "t1 where t1 = ({ a: t1 }) -> string",
      to_string_type_id(tm.wanted_type)
    );
    assert_eq!("number", to_string_type_id(tm.given_type));
  }
}

mod type_infer_aliases_table_types_record_the_property_locations {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_table_types_record_the_property_locations() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Table = {
            create: () -> ()
        }

        local x: Table
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.require_type_alias(&String::from("Table"));
    let ttv = get_type_id::<TableType>(follow_type_id(ty)).expect("expected TableType");
    let prop = ttv.props.get("create").expect("expected create prop");

    assert_eq!(None, prop.location);
    assert_eq!(
      Some(Location {
        begin: Position {
          line: 2,
          column: 12,
        },
        end: Position {
          line: 2,
          column: 18,
        },
      }),
      prop.type_location
    );
  }
}

mod type_infer_aliases_type_alias_adds_reduce_constraint_for_type_function {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_adds_reduce_constraint_for_type_function() {
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
    type plus<T> = add<number, T>

    local sum: plus<number> = 10
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_type_alias_dont_crash_on_bad_name {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_dont_crash_on_bad_name() {
    use alloc::string::String;

    use ulua_analysis::records::reserved_identifier::ReservedIdentifier;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type typeof = typeof(nil :: any)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<ReservedIdentifier>(&result.errors[0])
      .expect("expected ReservedIdentifier");
  }
}

mod type_infer_aliases_type_alias_dont_crash_on_duplicate_with_typeof {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_dont_crash_on_duplicate_with_typeof() {
    use alloc::string::String;

    use ulua_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = typeof(nil :: any)
        type A = typeof(nil :: any)
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
      .expect("expected DuplicateTypeDefinition");
  }
}

mod type_infer_aliases_type_alias_fwd_declaration_is_precise {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_fwd_declaration_is_precise() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local foo: Id<number> = 1
        type Id<T> = T
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_aliases_type_alias_import_mutation {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_import_mutation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_global_binding::get_global_binding, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture
      .base
      .check_string_optional_frontend_options(&String::from("type t10<x> = typeof(table)"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = get_global_binding(&mut fixture.get_frontend().globals, "table");

    assert_eq!("typeof(table)", to_string_type_id(ty));

    let ttv = get_type_id::<TableType>(ty).expect("expected table type");
    assert!(ttv.instantiated_type_params.is_empty());
  }
}

mod type_infer_aliases_type_alias_local_mutation {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_local_mutation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Cool = { a: number, b: string }
        local c: Cool = { a = 1, b = "s" }
        type NotCool<x> = Cool
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("c"));
    assert_eq!("Cool", to_string_type_id(ty));

    let ty = follow_type_id(ty);
    let ttv = get_type_id::<TableType>(ty).expect("expected TableType");
    assert!(ttv.instantiated_type_params.is_empty());
  }
}

mod type_infer_aliases_type_alias_local_rename {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_local_rename() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type Cool = { a: number, b: string }
type NotCool = Cool
local c: Cool = { a = 1, b = "s" }
local d: NotCool = { a = 1, b = "s" }
"#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Cool",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "NotCool",
      to_string_type_id(fixture.require_type_string(&String::from("d")))
    );
  }
}

mod type_infer_aliases_type_alias_local_synthetic_mutation {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_local_synthetic_mutation() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local c = { a = 1, b = "s" }
type Cool = typeof(c)
"#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("c"));
    let ty = follow_type_id(ty);
    let ttv = get_type_id::<TableType>(ty).expect("expected TableType");
    assert_eq!(Some(String::from("Cool")), ttv.name.clone());
  }
}

mod type_infer_aliases_type_alias_locations {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_locations() {
    use alloc::string::String;

    use ulua_analysis::functions::find_scope_at_position::find_scope_at_position;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = number

        do
            type T = string
            type X = boolean
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    assert!(!module.scopes.is_empty());

    let module_scope = module.scopes[0].1.clone();
    assert_eq!(
      Some(&Location {
        begin: Position {
          line: 1,
          column: 13,
        },
        end: Position {
          line: 1,
          column: 14,
        },
      }),
      module_scope.type_alias_name_locations.get("T")
    );

    let do_scope =
      find_scope_at_position(module, Position { line: 4, column: 0 }).expect("expected do scope");
    assert_eq!(
      Some(&Location {
        begin: Position {
          line: 4,
          column: 17,
        },
        end: Position {
          line: 4,
          column: 18,
        },
      }),
      do_scope.type_alias_name_locations.get("T")
    );
    assert_eq!(
      Some(&Location {
        begin: Position {
          line: 5,
          column: 17,
        },
        end: Position {
          line: 5,
          column: 18,
        },
      }),
      do_scope.type_alias_name_locations.get("X")
    );
  }
}

mod type_infer_aliases_type_alias_of_an_imported_recursive_generic_type {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_of_an_imported_recursive_generic_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        export type X<T, U> = { a: T, b: U, C: X<T, U>? }
        return {}
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Import = require(game.A)
        type X<T, U> = Import.X<T, U>
    "#,
      ),
      None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let ty1 = fixture
      .base
      .lookup_imported_type(&String::from("Import"), &String::from("X"))
      .expect("expected imported type Import.X");
    let ty2 = fixture
      .base
      .lookup_type(&String::from("X"))
      .expect("expected local type X");
    let mut opts = ToStringOptions::new(true);
    let ty1_string = to_string_type_id_to_string_options(ty1, &mut opts);
    let mut opts = ToStringOptions::new(true);
    let ty2_string = to_string_type_id_to_string_options(ty2, &mut opts);
    assert_eq!(ty1_string, ty2_string);

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Import = require(game.A)
        type X<T, U> = Import.X<U, T>
    "#,
      ),
      None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let ty1 = fixture
      .base
      .lookup_imported_type(&String::from("Import"), &String::from("X"))
      .expect("expected imported type Import.X");
    let ty2 = fixture
      .base
      .lookup_type(&String::from("X"))
      .expect("expected local type X");

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "t1 where t1 = { C: t1?, a: T, b: U }",
      to_string_type_id_to_string_options(ty1, &mut opts)
    );

    let expected_ty2 = if !FFlag::DebugLuauForceOldSolver.get() {
      "t1 where t1 = { C: t1?, a: U, b: T }"
    } else {
      "{ C: t1, a: U, b: T } where t1 = { C: t1, a: U, b: T }?"
    };
    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      expected_ty2,
      to_string_type_id_to_string_options(ty2, &mut opts)
    );
  }
}

mod type_infer_aliases_type_alias_of_an_imported_recursive_type {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_type_alias_of_an_imported_recursive_type() {
    use alloc::string::String;

    use ulua_analysis::functions::follow_type::follow_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type X = { a: number, b: X? }
return {}
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Import = require(game.A)
type X = Import.X
    "#,
      ),
      None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let ty1 = fixture
      .base
      .lookup_imported_type(&String::from("Import"), &String::from("X"))
      .expect("expected imported type Import.X");
    let ty2 = fixture
      .base
      .lookup_type(&String::from("X"))
      .expect("expected local type X");
    assert_eq!(follow_type_id(ty1), follow_type_id(ty2));
  }
}

mod type_infer_aliases_typeof_is_not_a_valid_alias_name {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_typeof_is_not_a_valid_alias_name() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type typeof = number
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "typeof cannot be used as an identifier for a type function or alias"
    } else {
      "Type aliases cannot be named typeof"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_aliases_use_table_name_and_generic_params_in_errors {
  //! Ported from `tests/TypeInfer.aliases.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_aliases_use_table_name_and_generic_params_in_errors() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_error::get_type_error, to_string_to_string_alt_c::to_string_type_id},
      records::type_mismatch::TypeMismatch,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Pair<T, U> = {first: T, second: U}
        local a: Pair<string, number>
        local b: Pair<string, string>

        a = b
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Pair<string, number>", to_string_type_id(tm.wanted_type));
    assert_eq!("Pair<string, string>", to_string_type_id(tm.given_type));
  }
}
