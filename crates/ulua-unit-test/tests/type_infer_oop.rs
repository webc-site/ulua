use ulua_analysis::type_aliases::module_name_type::ModuleName;

extern crate alloc;

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_assign_to_prop_of_intersection_of_metatables() {
  use ulua_analysis::{functions::get_error::get_type_error, records::type_mismatch::TypeMismatch};
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _fix_prop_reads = ScopedFastFlag::new(&fflag::LuauFixPropReadsOnMetatableTypes, true);
  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict

        local Base = {}
        Base.__index = Base

        type BaseStructure = { BaseString: string }

        export type Base = setmetatable<BaseStructure, typeof(Base)>

        function Base.new() : Base
            return nil :: any
        end

        local Sub = {}
        Sub.__index = Sub

        type SubStructure = { SubString: string }

        type Sub = setmetatable<SubStructure, typeof(Sub)> & Base

        function Sub.new() : Sub
            local self: Sub = setmetatable(Base.new(), Sub) :: any

            self.SubString = 5 -- Line 24
            self.BaseString = 5 -- Line 25

            return self
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!(24, result.errors[0].location.begin.line);
  get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!(25, result.errors[1].location.begin.line);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_augmenting_an_unsealed_table_with_a_metatable() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        local A = {number = 8}

        local B = setmetatable({}, A)

        function B:method()
            return "hello!!"
        end
    "#,
    None,
  );

  let mut opts = ToStringOptions::new(true);
  let b_type = fixture.base.require_type_string("B");
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "setmetatable<{ method: (unknown) -> string }, { number: number }>"
  } else {
    "setmetatable<{| method: <a>(a) -> string |}, {| number: number |}>"
  };
  assert_eq!(
    expected,
    to_string_type_id_to_string_options(b_type, &mut opts)
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_check_methods_of_sealed() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
local x: {prop: number} = {prop=9999}
function x:y(z: number)
    local s: string = z
end
"#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_class_decl() {
  use ulua_analysis::{
    functions::{get_type, to_string_to_string::to_string_type_id},
    records::extern_type::ExternType,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number
            public y: number
        end

        local p = Point.new { x = 2, y = 3 }

        local x = p.x
        local y = p.y
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let t = fixture.require_type_alias("Point");
  assert_eq!("Point", to_string_type_id(t));

  assert!(
    get_type::get::<ExternType>(t).is_some(),
    "expected Point alias to have ExternType"
  );

  assert_eq!("Point", to_string_type_id(fixture.require_type_string("p")));
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("x"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("y"))
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_class_that_shadows_a_type_alias() {
  use ulua_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type AAA = { x: number }
        class AAA end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
    .expect("expected DuplicateTypeDefinition");
  assert_eq!("AAA", err.name());
  assert!(err.previous_location().is_some());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_classes_arent_in_old_solver() {
  use ulua_analysis::records::generic_error::GenericError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(" class Point end ", None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
  assert_eq!("class keyword is illegal here", err.message());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_cross_module_metatable() {
  use alloc::string::String;

  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        --!strict
        local cls = {}
        cls.__index = cls
        function cls:abc() return 4 end
        return cls
    "#,
    ),
  );
  fixture.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        --!strict
        local cls = require(game.A)
        local tbl = {}
        setmetatable(tbl, cls)
    "#,
    ),
  );

  let module_b_name = ModuleName::from("game/B");
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b_name, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_b_name);
  let module_scope = module_b.get_module_scope();
  let cls_binding = module_scope
    .linear_search_for_binding("tbl", false)
    .expect("expected binding for tbl");

  assert_eq!(
    "setmetatable<tbl, cls>",
    to_string_type_id(cls_binding.type_id)
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_cycle_between_object_constructor_and_alias() {
  use ulua_analysis::{
    functions::{follow_type, get_type, to_string_to_string::to_string_type_id},
    records::metatable_type::MetatableType,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local T = {}
        T.__index = T

        function T.new(): T
            return setmetatable({}, T)
        end

        export type T = typeof(T.new())

        return T
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = unsafe { &*fixture.base.get_main_module(false) };
  let alias_type = module
    .exported_type_bindings
    .get("T")
    .expect("expected exported type T")
    .r#type();
  let followed = follow_type::follow(alias_type);
  let metatable = get_type::get::<MetatableType>(followed);
  assert!(
    metatable.is_some(),
    "expected metatable type, got {}",
    to_string_type_id(alias_type)
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_dont_bind_free_tables_to_themselves() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local T = {}
        local b: any

        function T:m()
            local a = b[i]
            if a then
                self:n()
                if self:p(a) then
                    self:n()
                end
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        type T = {method: ((T, number) -> number) & ((number) -> number)}
        local T: T

        T.method(4)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2() {
  use ulua_analysis::{
    functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local someTable = {}

        local function abs(x: number)
            if x < 0 then
                return -x
            else
                return x
            end
        end

        someTable.Function2 = function(Arg1, Arg2)
            abs(Arg1)
            abs(Arg2)
        end

        someTable.Function2() -- Argument count mismatch
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<CountMismatch>(&result.errors[0]);
  assert!(
    err.is_some(),
    "expected CountMismatch, got {:?}",
    result.errors[0]
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_not_defined_with_colon() {
  use ulua_analysis::{
    functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local someTable = {}

        local function abs(x: number)
            if x < 0 then
                return -x
            else
                return x
            end
        end

        someTable.Function1 = function(Arg1)
            abs(Arg1)
        end

        someTable.Function1() -- Argument count mismatch
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<CountMismatch>(&result.errors[0]);
  assert!(
    err.is_some(),
    "expected CountMismatch, got {:?}",
    result.errors[0]
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_empty_class() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(" class Point end ", None);

  assert!(result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_export_class_isnt_in_old_solver() {
  use ulua_analysis::records::generic_error::GenericError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(" export class Point end ", None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
  assert_eq!("class keyword is illegal here", err.message());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_flag_when_index_metamethod_returns_0_values() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local T = {}
        function T.__index()
        end

        local a = setmetatable({}, T)
        local p = a.prop
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "nil",
    to_string_type_id(fixture.base.require_type_string("p"))
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_fuzzer_duplicate_class_definition() {
  use ulua_analysis::records::syntax_error::SyntaxError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        class l0
        end
        class l0
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!(
    "A class named 'l0' has already been declared in this module",
    err.message()
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_fuzzer_self_referential_class_definition() {
  use ulua_analysis::{functions::get_type, records::extern_type::ExternType};
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        class l0
            public _:typeof(l0)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let l0 = fixture.require_type_string("l0");
  assert!(
    get_type::get::<ExternType>(l0).is_some(),
    "expected l0 to have ExternType"
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        function Base64FileReader(data)
            local reader = {}
            local index: number = 0

            function reader:PeekByte()
                return data:byte(index)
            end

            function reader:Byte()
                return data:byte(index - 1)
            end

            return reader
        end

        Base64FileReader()

        function ReadMidiEvents(data)

            local reader = Base64FileReader(data)

            while reader:HasMore() do
                (reader:Byte() % 128)
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory() {
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        ("foo")
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
    "#,
    None,
  );

  let module = fixture.get_main_module(false);
  let type_count = unsafe { (*module).internal_types.types.size() };
  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(80 >= type_count, "type count was {}", type_count);
  } else {
    assert!(50 >= type_count, "type count was {}", type_count);
  }
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_instantiate_duplicate_class() {
  use ulua_analysis::records::{
    cannot_call_non_function::CannotCallNonFunction, syntax_error::SyntaxError,
    unknown_symbol::UnknownSymbol,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
class l0
end
class l0
end
_ = l0 {  }
"#,
    None,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!(
    "A class named 'l0' has already been declared in this module",
    err.message()
  );
  type_error_data_ref::<UnknownSymbol>(&result.errors[1]).expect("expected UnknownSymbol");
  type_error_data_ref::<CannotCallNonFunction>(&result.errors[2])
    .expect("expected CannotCallNonFunction");
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_metatable_field_allows_upcast() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Foobar = {}
        Foobar.__index = Foobar
        Foobar.const = 42

        local foobar = setmetatable({}, Foobar)

        local _: { read const: number } = foobar
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_metatable_field_disallows_invalid_upcast() {
  use ulua_analysis::{
    functions::{
      get_error::get_type_error,
      to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options},
    },
    records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Foobar = {}
        Foobar.__index = Foobar
        Foobar.const = 42

        local foobar = setmetatable({}, Foobar)

        local _: { const: number } = foobar
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("{ const: number }", to_string_type_id(err.wanted_type));
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "setmetatable<{  }, t1> where t1 = { __index: t1, const: number }",
    to_string_type_id_to_string_options(err.given_type, &mut opts)
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_metatable_field_precedence_for_subtyping() {
  use ulua_analysis::{
    functions::{
      get_error::get_type_error, to_string_to_string::to_string_type_id_to_string_options,
    },
    records::{to_string_options::ToStringOptions, type_mismatch::TypeMismatch},
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function foobar1(_: { read foo: number }) end
        local function foobar2(_: { read bar: boolean }) end
        local function foobar3(_: { read foo: string }) end

        local t = { foo = 4 }
        setmetatable(t, { __index = { foo = "heh", bar = true }})
        foobar1(t)
        foobar2(t)
        foobar3(t)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{ read foo: string }",
    to_string_type_id_to_string_options(err.wanted_type, &mut opts)
  );
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "setmetatable<{ foo: number }, { __index: { bar: boolean, foo: string } }>",
    to_string_type_id_to_string_options(err.given_type, &mut opts)
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_method_depends_on_table() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        -- This catches a bug where x:m didn't count as a use of x
        -- so toposort would happily reorder a definition of
        -- function x:m before the definition of x.
        function g() f() end
        local x = {}
        function x:m() end
        function f() x:m() end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_method_should_not_create_cyclic_type() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local Component = {}

        function Component:__resolveUpdate(incomingState)
            local oldState = self.state
            incomingState = oldState
            self.state = incomingState
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_methods_are_topologically_sorted() {
  use ulua_analysis::records::primitive_type::PrimitiveType;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local T = {}

        function T:foo()
            return T:bar(999), T:bar("hi")
        end

        function T:bar(i)
            return i
        end

        local a, b = T:foo()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let a_type = fixture.require_type_string("a");
  let b_type = fixture.require_type_string("b");
  assert_eq!(
    Some(PrimitiveType::NUMBER),
    fixture.get_primitive_type(a_type)
  );
  assert_eq!(
    Some(PrimitiveType::STRING),
    fixture.get_primitive_type(b_type)
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_nonstrict_self_mismatch_tail() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        local f = {}
        function f:foo(a: number, b: number) end

        function bar(...)
            f.foo(f, 1, ...)
        end

        bar(2)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_object_constructor_can_refer_to_method_of_self() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict

        type Foo = {
            fooConn: () -> () | nil
        }

        local Foo = {}
        Foo.__index = Foo

        function Foo.new()
            local self: Foo = {
                fooConn = nil,
            }
            setmetatable(self, Foo)

            self.fooConn = function()
                self:method() -- Key 'method' not found in table self
            end

            return self
        end

        function Foo:method()
            print("foo")
        end

        local foo = Foo.new()

        -- TODO This is the best our current refinement support can offer :(
        local bar = foo.fooConn
        if bar then bar() end

        -- foo.fooConn()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_oop_invoke_with_inferred_self_and_property() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local ItemContainer = {}
        ItemContainer.__index = ItemContainer

        function ItemContainer.new(name)
            local self = {name = name}
            setmetatable(self, ItemContainer)
            return self
        end

        function ItemContainer:removeItem(itemId, itemType)
            print(self.name)
            self:getItem(itemId, itemType)
        end

        function ItemContainer:getItem(itemId, itemType): ()
        end

        local container = ItemContainer.new("library")

        container:removeItem(0, "magic")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_oop_invoke_with_inferred_self_type() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local ItemContainer = {}
        ItemContainer.__index = ItemContainer

        function ItemContainer.new()
            local self = {}
            setmetatable(self, ItemContainer)
            return self
        end

        function ItemContainer:removeItem(itemId, itemType)
            self:getItem(itemId, itemType)
        end

        function ItemContainer:getItem(itemId, itemType): ()
        end

        local container = ItemContainer.new()

        container:removeItem(0, "magic")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_pass_too_many_arguments() {
  use ulua_analysis::{
    functions::get_error::get_type_error, records::count_mismatch::CountMismatch,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        type T = {
            method: (T, number) -> number
        }

        function makeT(): T
            return {
                method=function(self, number)
                    return number * 2
                end
            }
        end

        local a = makeT()
        a:method(5, 7)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let count_mismatch =
    get_type_error::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
  assert_eq!(2, count_mismatch.expected());
  assert_eq!(3, count_mismatch.actual());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_point_class() {
  use ulua_analysis::{
    functions::{get_type, to_string_to_string::to_string_type_id},
    records::extern_type::ExternType,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number
            public y: number

            function length(self): number
                return 100
            end

            function __init(self, x: number, y: number)
                self.x = x
                self.y = y
            end
        end

        local p = Point.new(2, 3)
        local len = p:length()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let p = fixture.require_type_string("p");
  assert!(
    get_type::get::<ExternType>(p).is_some(),
    "expected p to have ExternType"
  );

  assert_eq!("Point", to_string_type_id(p));
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("len"))
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_promise_type_error_too_complex() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend().options.retain_full_type_graphs = false;

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict

        local Promise = {}
        Promise.prototype = {}
        Promise.__index = Promise.prototype

        function Promise._new(traceback, callback, parent)
            if parent ~= nil and not Promise.is(parent)then
            end

            local self = {
                _parent = parent,
            }

            parent._consumers[self] = true
            setmetatable(self, Promise)
            self:_reject()

            return self
        end

        function Promise.resolve(...)
            return Promise._new(debug.traceback(nil, 2), function(resolve)
            end)
        end

        function Promise.reject(...)
            return Promise._new(debug.traceback(nil, 2), function(_, reject)
            end)
        end

        function Promise._try(traceback, callback, ...)
            return Promise._new(traceback, function(resolve)
            end)
        end

        function Promise.try(callback, ...)
            return Promise._try(debug.traceback(nil, 2), callback, ...)
        end

        function Promise._all(traceback, promises, amount)
            if #promises == 0 or amount == 0 then
                return Promise.resolve({})
            end
            return Promise._new(traceback, function(resolve, reject, onCancel)
            end)
        end

        function Promise.all(promises)
            return Promise._all(debug.traceback(nil, 2), promises)
        end

        function Promise.allSettled(promises)
            return Promise.resolve({})
        end

        function Promise.race(promises)
            return Promise._new(debug.traceback(nil, 2), function(resolve, reject, onCancel)
            end)
        end

        function Promise.each(list, predicate)
            return Promise._new(debug.traceback(nil, 2), function(resolve, reject, onCancel)
                local predicatePromise = Promise.resolve(predicate(value, index))
                local success, result = predicatePromise:await()
            end)
        end

        function Promise.is(object)
        end

        function Promise.prototype:_reject(...)
            self:_finalize()
        end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_prop_with_typeof_reassigned_class() {
  use ulua_analysis::records::syntax_error::SyntaxError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _export_value = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
class Animal end
Animal = nil
class l0
public _:typeof(Animal)
end
"#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!(
    // cpp TypeInfer.classes.test.cpp:1227 类名全局不可作为变量名赋值。
    "'Animal' refers to a class and cannot be used as a variable name (defined on line 2)",
    err.message()
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_quantify_methods_defined_using_dot_syntax_and_explicit_self_parameter() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local T = {}

        function T.method(self)
            self:method()
        end

        function T.method2(self)
            self:method()
        end

        T:method2()
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_react_style_oo() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Prototype = {}

        local ClassMetatable = {
            __index = Prototype
        }

        local BaseClass = (setmetatable({}, ClassMetatable))

        function BaseClass:extend(name)
            local class = {
                name=name
            }

            class.__index = class

            function class.ctor(props)
                return setmetatable({props=props}, class)
            end

            return setmetatable(class, getmetatable(self))
        end

        local C = BaseClass:extend('C')
        local i = C.ctor({hello='world'})

        local iName = i.name
        local cName = C.name
        local hello = i.props.hello
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_string("iName"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_string("cName"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_string("hello"))
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_read_unknown_property_from_class_object_or_instance() {
  use ulua_analysis::records::unknown_property::UnknownProperty;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);
  let _access_violation = ScopedFastFlag::new(&fflag::LuauTweakAccessViolationReporting, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number
            public y: number

            function zero(): Point
                return Point.new {x=0, y=0}
            end
        end

        local p = Point.zero()
        local a = p.z
        local b = Point.z
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  let up0 =
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  assert_eq!("z", up0.key());
  let up1 =
    type_error_data_ref::<UnknownProperty>(&result.errors[1]).expect("expected UnknownProperty");
  assert_eq!("z", up1.key());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_repeat_class_methods() {
  use ulua_analysis::records::syntax_error::SyntaxError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
class l0
    function foo()
    end
    function foo()
    end
end
"#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!("Duplicate class member 'foo'", err.message());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_repeat_nameless_class_methods() {
  use ulua_analysis::records::syntax_error::SyntaxError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
class l0
    function  ()
    end
    function ()
    end
end
"#,
    None,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  let err1 = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!(
    "Expected identifier when parsing method name, got '('",
    err1.message()
  );
  let err2 = type_error_data_ref::<SyntaxError>(&result.errors[1]).expect("expected SyntaxError");
  assert_eq!(
    "Expected identifier when parsing method name, got '('",
    err2.message()
  );
  let err3 = type_error_data_ref::<SyntaxError>(&result.errors[2]).expect("expected SyntaxError");
  assert_eq!("Duplicate class member '%error-id%'", err3.message());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_repeat_props() {
  use ulua_analysis::records::syntax_error::SyntaxError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
class l0
    public foo
    public foo
end
"#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!("Duplicate class member 'foo'", err.message());
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_self_argument_has_self_type() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        class I
            function m(self): I
                return self
            end
        end

        local i = I.new{}
        local i2 = i:m()
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!("I", to_string_type_id(fixture.require_type_string("i2")));
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_set_prop_of_intersection_containing_metatable() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        export type Set<T> = typeof(setmetatable(
            {} :: {
                add: (self: Set<T>, T) -> Set<T>,
            },
            {}
        ))

        local Set = {} :: Set<any> & {}

        function Set:add(t)
            return self
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_table_oop() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
   --!strict
local Class = {}
Class.__index = Class

type Class = typeof(setmetatable({} :: { x: number }, Class))

function Class.new(x: number): Class
    return setmetatable({x = x}, Class)
end

function Class.getx(self: Class)
    return self.x
end

function test()
    local c = Class.new(42)
    local n = c:getx()
    local nn = c.x

    print(string.format("%d %d", n, nn))
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_textbook_class_pattern() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Account = {}
        Account.__index = Account

        type AccountData = {
            name: string,
            balance: number,
        }

        export type Account = setmetatable<AccountData, typeof(Account)>

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance

            return setmetatable(self, Account)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_textbook_class_pattern_2() {
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Account = {}
        Account.__index = Account

        type AccountData = {
            name: string,
            balance: number,
        }

        export type Account = setmetatable<AccountData, typeof(Account)>

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance

            return setmetatable(self, Account)
        end

        function Account.deposit(self: Account, credit: number)
            self.balance += credit
        end

        function Account.withdraw(self: Account, debit: number)
            self.balance -= debit
        end

        function Account.hasBalance(self: Account, amount: number): boolean
            return self.balance >= amount
        end

        local account = Account.new("Hina", 500)

        if account:hasBalance(123) then -- TypeError: Value of type 'unknown' could be nil
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_typecheck_class_annotations() {
  use ulua_analysis::records::{type_mismatch::TypeMismatch, type_pack_mismatch::TypePackMismatch};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number
            public y: number
            public name: string
            function magnitude(self): string
                -- self.name is not a number
                self.name = self.x

                -- This function is declared to return string.
                return math.sqrt(self.x * self.x + self.y * self.y)
            end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(
    result
      .errors
      .iter()
      .any(|err| type_error_data_ref::<TypeMismatch>(err).is_some())
  );
  assert!(
    result
      .errors
      .iter()
      .any(|err| type_error_data_ref::<TypePackMismatch>(err).is_some())
  );
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_typecheck_class_method_field_access() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::uninhabited_type_function::UninhabitedTypeFunction,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number?
            public y: number?
            function magnitude(self): number
                return math.sqrt(self.x * self.x + self.y * self.y)
            end
        end
    "#,
    None,
  );

  assert_eq!(4, result.errors.len(), "{:?}", result.errors);
  for err in &result.errors {
    let utf = type_error_data_ref::<UninhabitedTypeFunction>(err)
      .expect("expected UninhabitedTypeFunction");
    assert_eq!("mul<number?, number?>", to_string_type_id(utf.ty()));
  }
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_writes_to_class_object_properties_are_forbidden() {
  use ulua_analysis::records::property_access_violation::{self, PropertyAccessViolation};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);
  let _access_violation = ScopedFastFlag::new(&fflag::LuauTweakAccessViolationReporting, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number
            public y: number

            function zero(): Point
                return Point.new {x=0, y=0}
            end

            function magnitude(self): number
                return 5 -- stochastic approximation for performance
            end
        end

        Point.magnitude = function(p: Point) return 3 end
        Point.zero = function() return Point.new { x = 1, y = 1 } end
        Point.one = function() return Point.new { x = 1, y = 1 } end
    "#,
    None,
  );

  let expected = ["magnitude", "zero", "one"];
  assert_eq!(expected.len(), result.errors.len(), "{:?}", result.errors);
  for (err, key) in result.errors.iter().zip(expected) {
    let pav = type_error_data_ref::<PropertyAccessViolation>(err)
      .expect("expected PropertyAccessViolation");
    assert_eq!(key, pav.key());
    assert_eq!(
      property_access_violation::Context::CannotWrite,
      pav.context()
    );
  }
}

// Source: `tests/TypeInfer.metatableOOP.test.cpp`
#[test]
fn type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden() {
  use ulua_analysis::records::property_access_violation::{self, PropertyAccessViolation};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _tidy = ScopedFastFlag::new(&fflag::LuauTidyTypePrototyping, true);
  let _access_violation = ScopedFastFlag::new(&fflag::LuauTweakAccessViolationReporting, true);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Point
            public x: number
            public y: number

            function zero(): Point
                return Point.new {x=0, y=0}
            end

            function magnitude(self): number
                return 5 -- stochastic approximation for performance
            end
        end

        local p = Point.zero()
        p.magnitude = function(p: Point) return 3 end
        p.zero = function() return Point.new { x = 1, y = 1 } end
        p.one = function() return Point.new { x = 1, y = 1 } end

        p.__index = {}
    "#,
    None,
  );

  let expected = ["magnitude", "zero", "one", "__index"];
  assert_eq!(expected.len(), result.errors.len(), "{:?}", result.errors);
  for (err, key) in result.errors.iter().zip(expected) {
    let pav = type_error_data_ref::<PropertyAccessViolation>(err)
      .expect("expected PropertyAccessViolation");
    assert_eq!(key, pav.key());
    assert_eq!(
      property_access_violation::Context::CannotWrite,
      pav.context()
    );
  }
}
// Ported from `tests/TypeInfer.metatableOOP.test.cpp` ("fuzzer_setmetatable_invalid_types").
// 回归：构造 `MetatableType` 时 table 成员为非表类型不得 panic，且必须报错。
#[test]
fn type_infer_oop_fuzzer_setmetatable_invalid_types() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        return setmetatable(_ < _,setmetatable(setmetatable(_,_),{"",},math.abs))
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(

      r#"
        return setmetatable(if _ then setmetatable(_,_) else {""}, {""}, _) setmetatable(_,_) else {""}, {""}, _)
    "#
,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
