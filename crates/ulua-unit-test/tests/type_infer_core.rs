use ulua_analysis::type_aliases::module_name_type::ModuleName;
extern crate alloc;

// 277 处 test fn 体内逐例重复的 use 统一上提至此（借 tst-r16 pretty_printer/fragment_autocomplete 上提先例；源头 cpp 侧即为整文件共享的 using 声明）。
use alloc::{string::String, sync::Arc};
use core::ptr::null_mut;

use ulua_analysis::{
  enums::{solver_mode::SolverMode, table_state::TableState},
  functions::{
    find_node_at_position_ast_query::find_node_at_position_source_module_position,
    first::first,
    freeze::freeze,
    get_type,
    to_string_error::to_string_type_error,
    to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options},
    unfreeze::unfreeze,
  },
  records::{
    cannot_call_non_function::CannotCallNonFunction, code_too_complex::CodeTooComplex,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    count_mismatch::CountMismatch, deprecated_api_used::DeprecatedApiUsed,
    function_type::FunctionType, missing_properties::MissingProperties,
    occurs_check_failed::OccursCheckFailed, primitive_type::PrimitiveType, scope::Scope,
    table_type::TableType, to_string_options::ToStringOptions, type_fun::TypeFun,
    type_level::TypeLevel, type_mismatch::TypeMismatch, unknown_property::UnknownProperty,
    unknown_symbol::UnknownSymbol,
  },
};
use ulua_ast::{
  records::{ast_expr_function::AstExprFunction, location::Location, position::Position},
  rtti::ast_node_try_as_ptr,
};
use ulua_common::{dfint, fflag, fint};
use ulua_unit_test::{
  functions::{has_error::has_error, type_error_data_ref::type_error_data_ref},
  records::{builtins_fixture::BuiltinsFixture, fixture::Fixture},
  type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
};

// 样板收口助手：原逐例重复的 BuiltinsFixture 构造+get_frontend+默认选项检查
// 语句收口为宏，行为与原语句逐字一致（借 tst-r16 ir_lowering 助手先例）。
macro_rules! bs_check {
  ($src:expr) => {{
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options($src, None);
    (fixture, result)
  }};
}

// 样板收口助手：原逐例重复的 Fixture::fixture_bool(false) 构造+默认选项检查
// 语句收口为宏，行为与原语句逐字一致（借 tst-r16 ir_lowering 助手先例）。
macro_rules! fx_check {
  ($src:expr) => {{
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options($src, None);
    (fixture, result)
  }};
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_any_type_in_function_argument_should_not_error() {
  let (_fixture, result) = bs_check!(
    r#"
        --!strict
        local function f(u: string) end

        local t: {[any]: any} = {}

        for k in t do
            f(k)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_assert_allows_singleton_union_or_intersection() {
  let (_fixture, result) = fx_check!(
    r#"
        local x = 42 :: | number
        local y = 42 :: & number
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_assert_table_freeze_constraint_solving() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        local f = table.freeze
        f(table)
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_avoid_blocking_type_function() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        local function foo(a : string?)
            local b = a or ""
            return b:upper()
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_avoid_double_reference_to_free_type() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        local function wtf(name: string?)
            local message
            message = "invalid alternate fiber: " .. (name or "UNNAMED alternate")
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_bad_iter_metamethod() {
  let (_fixture, result) = bs_check!(
    r#"
        function iter(): unknown
            return nil
        end

        local a = {__iter = iter}
        setmetatable(a, a)

        for i in a do
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let ccnf = type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
      .expect("expected CannotCallNonFunction");
    assert_eq!("unknown", to_string_type_id(ccnf.ty()));
  } else {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (_fixture, result) = bs_check!(
    r#"
        local function concat<T>(target: {T}, ...: {T} | T): {T}
            return (nil :: any) :: {T}
        end

        local res = concat({"alic"}, 1, 2)
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  for error in &result.errors {
    assert_eq!(5, error.location.begin.line, "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_bidirectional_checking_of_higher_order_function() {
  let (_fixture, result) = fx_check!(
    r#"
        function higher(cb: (number) -> ()) end

        higher(function(n)      -- no error here.  n : number
            local e: string = n -- error here.  n /: string
        end)
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(4, result.errors[0].location.begin.line);
  assert_eq!(4, result.errors[0].location.end.line);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_bound_typepack_promote() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
local function p()
    local this = {}
    this.pf = foo()
    function this:IsActive() end
    function this:Start(o) end
    return this
end

local function h(tp, o)
    ep = tp
    tp:Start(o)
    tp.pf.Connect(function()
        ep:IsActive()
    end)
end

function on()
    local t = p()
    h(t)
end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_captured_globals_are_not_blocked() {
  let _forbid_internal_types = ScopedFastFlag::new(&fflag::DebugLuauForbidInternalTypes, true);

  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        local Cancelled: boolean = false

        function Start()
            if Cancelled then
                return
            end
            Selection = 42
            local _ = function ()
                if Selection then
                end
            end
        end

        function Cancel()
            Selection = nil
        end

        return {}
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_check_block_recursion_limit() {
  let limit: usize = if cfg!(debug_assertions) { 350 } else { 595 };

  let _luau_recursion_limit = ScopedFastInt::new(&fint::LuauRecursionLimit, limit as i32 * 2);
  let _luau_check_recursion_limit =
    ScopedFastInt::new(&fint::LuauCheckRecursionLimit, limit as i32 - 100);
  let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
    &dfint::LuauConstraintGeneratorRecursionLimit,
    limit as i32 - 100,
  );
  let _luau_subtyping_recursion_limit =
    ScopedFastInt::new(&dfint::LuauSubtypingRecursionLimit, limit as i32 - 100);

  let mut fixture = Fixture::fixture_bool(false);
  let code = "do ".repeat(limit) + "local a = 1" + &" end".repeat(limit);
  let result = fixture.check_string_optional_frontend_options(&code, None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<CodeTooComplex>(&result.errors[0]).expect("expected CodeTooComplex");
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_check_expr_recursion_limit() {
  let limit: usize = if cfg!(debug_assertions) { 250 } else { 500 };

  let _luau_recursion_limit = ScopedFastInt::new(&fint::LuauRecursionLimit, limit as i32 * 2);
  let _luau_check_recursion_limit =
    ScopedFastInt::new(&fint::LuauCheckRecursionLimit, limit as i32 - 100);
  let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
    &dfint::LuauConstraintGeneratorRecursionLimit,
    limit as i32 - 100,
  );
  let _luau_subtyping_recursion_limit =
    ScopedFastInt::new(&dfint::LuauSubtypingRecursionLimit, limit as i32 - 100);

  let mut fixture = Fixture::fixture_bool(false);
  let code = String::from(r#"("foo")"#) + &":lower()".repeat(limit);
  let result = fixture.check_string_optional_frontend_options(&code, None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<CodeTooComplex>(&result.errors[0]).is_some(),
    "Expected CodeTooComplex but got {}",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_check_type_infer_recursion_count() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let limit: usize = if cfg!(debug_assertions) { 350 } else { 600 };
  let _sfi = ScopedFastInt::new(&fint::LuauCheckRecursionLimit, limit as i32);

  let mut fixture = Fixture::fixture_bool(false);
  let code = String::from("function f() return ")
    + &"{a=".repeat(limit)
    + "'a'"
    + &"}".repeat(limit)
    + " end";
  let result = fixture.check_string_optional_frontend_options(&code, None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<CodeTooComplex>(&result.errors[0]).expect("expected CodeTooComplex");
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_checking_should_not_ice() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        f,g = ...
        f(g(...))[...] = nil
        f,xpcall = ...
        local value = g(...)(g(...))
    "#,
    None,
  );

  assert_eq!(
    "any",
    to_string_type_id(fixture.require_type_string("value"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_cli_39932_use_unifier_in_ensure_methods() {
  let (_fixture, result) = fx_check!(
    r#"
        local x: {number|number} = {1, 2, 3}
        local y = x[1] - x[2]
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_cli_50041_committing_txnlog_in_apollo_client_error() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        --!nolint

        type FieldSpecifier = {
            fieldName: string,
        }

        type ReadFieldOptions = FieldSpecifier & { from: number? }

        type Policies = {
            getStoreFieldName: (self: Policies, fieldSpec: FieldSpecifier) -> string,
        }

        local Policies = {}

        local function foo(p: Policies)
        end

        function Policies:getStoreFieldName(specifier: FieldSpecifier): string
            return ""
        end

        function Policies:readField(options: ReadFieldOptions)
            local _ = self:getStoreFieldName(options)
            foo(self)
        end
    "#,
    None,
  );

  if fflag::LuauInstantiateInSubtyping.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
      "Expected this to be exactly 'Policies' from 'MainModule', but got 'Policies' from 'MainModule'",
      "\ncaused by:\n",
      "  Property 'getStoreFieldName' is not compatible.\n",
      "Expected this to be exactly\n\t",
      "'(Policies, FieldSpecifier) -> string'",
      "\nbut got\n\t",
      "'(Policies, FieldSpecifier & { from: number? }) -> ('a, b...)'",
      "\ncaused by:\n",
      "  Argument #2 type is not compatible.\n",
      "Expected this to be exactly\n\t",
      "'FieldSpecifier & { from: number? }'",
      "\nbut got\n\t",
      "'FieldSpecifier'",
      "\ncaused by:\n",
      "  Not all intersection parts are compatible.\n",
      "Table type 'FieldSpecifier' not compatible with type '{ from: number? }' because the former has extra field 'fieldName'"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  } else {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_concat_string_with_string_union() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local function concat_stuff(x: string, y : string | number)
            return x .. y
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_config_reader_example() {
  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/ConfigReader"),
    String::from(
      r#"
        --!strict
        local ConfigReader = {}
        ConfigReader.Defaults = {}

        local Defaults = ConfigReader.Defaults
        local Config = ConfigReader.Defaults

        function ConfigReader:read(config_name: string)
            if Config[config_name] ~= nil then
                return Config[config_name]
            elseif Defaults[config_name] ~= nil then
                return Defaults[config_name]
            else
                error(config_name .. " must be defined in Config")
            end
        end

        function ConfigReader:getFullConfigWithDefaults()
            local config = {}
            for key, val in pairs(ConfigReader.Defaults) do
                config[key] = val
            end
            for key, val in pairs(Config) do
                config[key] = val
            end
            return config
        end

        return ConfigReader
    "#,
    ),
  );

  fixture.base.file_resolver.source.insert(
    String::from("game/Util"),
    String::from(
      r#"
        --!strict
        local ConfigReader = require(script.Parent.ConfigReader)
        local _ = ConfigReader:read("foobar")()
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Util"), None);
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_constraint_generation_recursion_limit() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _check_recursion_limit = ScopedFastInt::new(&fint::LuauCheckRecursionLimit, 5);
  let _constraint_generator_recursion_limit =
    ScopedFastInt::new(&dfint::LuauConstraintGeneratorRecursionLimit, 5);

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        if true then
        elseif true then
        elseif true then
        elseif true then
        else
        local x = 1
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        type React_Ref<ElementType> = { current: ElementType } | ((ElementType) -> ())

        type React_AbstractComponent<Config, Instance> = {
            render: ((ref: React_Ref<Instance>) -> nil)
        }

        local createElement : <P, T>(React_AbstractComponent<P, T>) -> ()

        function ScrollView:render()
            local one = table.unpack(
                if true then a else b
            )

            createElement(one)
            createElement(one)
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_correctly_scope_locals_do() {
  let (_fixture, result) = fx_check!(
    r#"
        do
            local a = 1
        end

        local b = a -- oops!
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let us = type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("a", us.name());
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_crazy_complexity() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        --!nonstrict
        A:A():A():A():A():A():A():A():A():A():A():A()
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_cyclic_follow() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
--!nonstrict
l0,table,_,_,_ = ...
_,_,_,_.time(...)._.n0,l0,_ = function(l0)
end,_.__index,(_),_.time(_.n0 or _,...)
for l0=...,_,"" do
end
_ += not _
do end
"#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_cyclic_follow_2() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
--!nonstrict
n13,_,table,_,l0,_,_ = ...
_,n0[(_)],_,_._(...)._.n39,l0,_._ = function(l84,...)
end,_.__index,"",_,l0._(nil)
for l0=...,table.n5,_ do
end
_:_(...).n1 /= _
do
_(_ + _)
do end
end
"#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_cyclic_unification_aborts_eventually() {
  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let _instantiate_in_subtyping = ScopedFastFlag::new(&fflag::LuauInstantiateInSubtyping, true);
  let _type_pack_loop_limit = ScopedFastInt::new(&fint::LuauTypeInferTypePackLoopLimit, 100);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture
    .base
    .check_string_optional_frontend_options(r#"pcall(table.unpack({pcall}))"#, None);

  assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_dcr_delays_expansion_of_function_containing_blocked_parameter_type() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local b: any

        function f(x)
            local a = b[1] or 'Cn'
            local c = x[1]

            if a:sub(1, #c) == c then
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_dont_ice_on_astexprerror() {
  let (_fixture, result) = fx_check!(
    r#"
        local foo = -;
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_dont_ice_when_failing_the_occurs_check() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        local s
        s(s, 'a')
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_dont_report_type_errors_within_an_ast_expr_error() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (_fixture, result) = fx_check!(
    r#"
        local a = foo:
    "#
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_dont_report_type_errors_within_an_ast_stat_error() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (_fixture, result) = fx_check!(
    r#"
        foo
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_exponential_blowup_from_copying_types() {
  let (mut fixture, result) = fx_check!(
    r#"
        --!strict
        -- An example of exponential blowup in number of types
        -- The problem is that if we define function f(a) return x end
        -- then this has type <t>(t)->T where x:T
        -- *but* it copies T each time f is applied
        -- so { left = f("hi"), right = f(5) }
        -- has type { left : T_L, right : T_R }
        -- where T_L and T_R are copies of T.
        -- x0 : T0 where T0 = {}
        local x0 = {}
        -- f0 : <t>(t)->T0
        local function f0(a) return x0 end
        -- x1 : T1 where T1 = { left : T0_L, right : T0_R }
        local x1 = { left = f0("hi"), right = f0(5) }
        -- f1 : <t>(t)->T1
        local function f1(a) return x1 end
        -- x2 : T2 where T2 = { left : T1_L, right : T1_R }
        local x2 = { left = f1("hi"), right = f1(5) }
        -- f2 : <t>(t)->T2
        local function f2(a) return x2 end
        -- etc etc
        local x3 = { left = f2("hi"), right = f2(5) }
        local function f3(a) return x3 end
        local x4 = { left = f3("hi"), right = f3(5) }
        return x4
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = unsafe { &*fixture.get_main_module(false) };
  assert!(
    5 >= module.interface_types.types.size(),
    "interface type count was {}",
    module.interface_types.types.size()
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_expr_statement() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options("local foo = 5    foo()", None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_follow_on_new_types_in_substitution() {
  let (_fixture, result) = fx_check!(
    r#"
        local obj = {}

        function obj:Method()
            self.fieldA = function(object)
                if object.a then
                    self.arr[object] = true
                elseif object.b then
                    self.fieldB[object] = object:Connect(function(arg)
                        self.arr[arg] = nil
                    end)
                end
            end
        end

        return obj
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level()
 {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        --!strict
        if _ then
            _[_], _ = nil
            _()
        end

        local aaa = function():typeof(_) return 1 end

        if aaa then
            while _() do
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_assert_table_freeze_constraint_solving() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        local function l0()
        end
        for l0 in false do
        _ = (if _ then table)
        repeat
        do end
        _:freeze(table)
        until if _ then {{n0=_,},(_:freeze()._[_]),}
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert!(
    !has_error::<ConstraintSolvingIncompleteError>(&result),
    "{:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
// Separate subsystem (NOT the for-in iterator cluster): on this fuzz input the
// type checker builds a *structurally self-referential* UnionType (`U = {T, X}`
// where one option follows back to `U` itself — a membership cycle, not a
// `BoundType` chain, so `follow`'s Floyd cycle-detector cannot see it). Any
// recursive type predicate that descends union options — e.g. `is_string`
// (Analysis/src/Type.cpp:199, a faithful 1:1 of C++ `isString`'s
// `std::all_of(begin(utv), end(utv), isString)`) — then recurses forever and
// overflows the stack. C++ avoids this only by never constructing such a union
// for this input; the defect is in the cyclic-union *construction* during the
// fuzzed `if/elseif`/`setmetatable` expression check, a different subsystem from
// for-in iteration. (Previously latent: the old-solver `check(AstStatForIn)` was
// a no-op stub, so iteration never reached `findMetatableEntry`/`isString` on the
// cyclic type; now that the for-in check is faithfully ported, the pre-existing
// hazard becomes reachable.) Re-ignored with precise cause per the task's
// separate-subsystem allowance; fixing it requires de-cycling union construction.
fn type_infer_fuzz_avoid_singleton_union() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        _ = if true then _ else {},if (_) then _ elseif "" then {} elseif _ then {} elseif _ then _ else {}
        for l0,l2 in setmetatable(_,_),l0,_ do
        end
    "#
    ,
      None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_dont_double_solve_compound_assignment() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local _ = {}
        _[function<t0...>(...)
            _[function(...)
                _[_] %= _
                _ = {}
                _ = (- _)()
            end] %= _
            _[_] %= _
        end] %= true
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert!(
    !has_error::<ConstraintSolvingIncompleteError>(&result),
    "{:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_free_table_type_change_during_index_check() {
  let (_fixture, result) = fx_check!(
    r#"
local _ = nil
while _["" >= _] do
end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_generalize_one_remove_type_assert() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local _ = {_ = _}, l0
        _ += _
        while _ do
            while _[_] do
                if _.n0 then
                    _ = _
                else
                    _ = _
                    return _
                end
                do
                    while _ do
                        _, _ = nil
                    end
                    return function()
                    end
                end
                while _[_] do
                    _ = _._VERSION, ""
                end
            end
            local _
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_generalize_one_remove_type_assert_2() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local _ = {n0 = _.n0}, -_, _
        _ += _.n0
        _ /= _[_]
        while _.n110 do
            while _._ do
                while _ do
                    while _ do
                        _ = _
                    end
                end
                while _[_] do
                    function _()
                    end
                end
            end
            while ... do
            end
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert!(
    !has_error::<ConstraintSolvingIncompleteError>(&result),
    "{:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_global_self_assignment() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options("_ = _", None);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_local_before_declaration_ice() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        local _
        table.freeze(_, _)
    "#
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let err0 = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("nil", to_string_type_id(err0.given_type));
  assert_eq!("table", to_string_type_id(err0.wanted_type));

  let err1 =
    type_error_data_ref::<CountMismatch>(&result.errors[1]).expect("expected CountMismatch");
  assert_eq!(1, err1.expected());
  assert_eq!(2, err1.actual());
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_missing_follow_table_freeze() {
  let (_fixture, result) = bs_check!(
    r#"
        if _:freeze(_)[_][_] then
        else
        do end
        end
        if _:freeze((nil))[_][_] then
        else
        do end
        end
        _ = table,true,_(lower)
        do end
        _:freeze()[_] += {} > _
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzz_simplify_combinatorial_explosion() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(

          r#"
_ = {[_[`{_ + ...}`]]=_,_,[{_=nil,[_._G]=false,}]={[_[_[_]][_][_ / ...]]=_,[...]=false,_,},[_[_][_][_]]=l255,},""
local _
    "#
    ,
      None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
_ = {[(_G)]=_,[_[_[_]][_[_]][nil][_]]={_G=_,},_[_[_]][_][_],n0={[_]=_,_G=_,},248,}
local _
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_allow_failing_to_bind_generic() {
  let (_fixture, result) = bs_check!(
    r#"
        function test(arg1, arg2)
            local fun1 = test(test)
            local fun2 = test(test())
            fun1(arg2, fun2)
        end

        test()
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_attach_polarity_to_ret_free_type() {
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        FOO =
            {
                [1 // setmetatable({}, FOO)] = 2,
                __idiv = function(lhs, rhs, ...) return ... end,
            }
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_avoid_double_negation() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
local _ = _
repeat
do end
while 0 do
do
_ = _[0]
_._ *= _
end
if _ then
elseif "" then
end
_ = _[0]
_ = ""
end
_ = ""
until _
while false do
do
_ = _[0]
do end
end
_ = ""
return if _ then _,_
end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        if if _ then _ else nil then
            local l0 = require(module0)
            _ = l0
        elseif _ then
            function _(l0:true,...)
            end
        else
        end
        _ = l0
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local l0 = require(module0)
        local l10 = require(module0)
        do end
        for l0=_,_,true do
        end
        do
        local l0 = require(module0)
        _ = l0
        local l10 = require(module0)
        function _()
        end
        end
        local l10 = require(module0)
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_bind_generic_sigsegv() {
  let (_fixture, result) = bs_check!(
    r#"
        function test(arg1, arg2)
            local fun = test()
            local fun2 = fun(nil, test(test()))
            fun2(test(test)())
        end

        local f = test()
        f(nil, test())
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_derived_unsound_loops() {
  let (_fixture, result) = fx_check!(
    r#"
        for _ in ... do
            repeat
                _ = 42
            until _
            repeat
                _ = _ + 2
            until _
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: tests/TypeInfer.test.cpp:3010（CHECK_NOTHROW：仅要求不 panic/ICE，无错误断言）
#[test]
fn type_infer_fuzzer_export_no_ice() {
  let (_fixture, _result) = fx_check!(
    r#"
        while true do
            export local _
        end
        do
            export local _
            _ = _
        end
    "#
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_found_this() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        l0, _ = nil

        local function p()
            _()
        end

        a = _(
            function():(typeof(p),typeof(_))
            end
        )[nil]
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_found_this_2() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        local _
        if _ then
            _ = _
            while _() do
                _ = # _
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_global_type_inference() {
  let (_fixture, result) = bs_check!(
    r#"
        A = A
        A = A
        function A()
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_has_indexer_can_create_cyclic_union() {
  let (_fixture, result) = bs_check!(
    r#"
        local _ = nil
        repeat
            _ = {[true] = _[_]}
            do
                repeat
                    _ = {[_[l0]] = _[_]}
                    return
                until #next(_) < _
            end
            local l0 = require(module0)
        until #_[_](_) < next(_)
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_infer_divergent_rw_props() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        return function(l0:{_:(any)&(any),write _:any,})
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_instantiate_iter_function() {
  let _polarity = ScopedFastFlag::new(&fflag::LuauInstantiationUsesPolarity, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        function iterfunc(l0)
            return l0()
        end
        for _, _ in setmetatable({}, { __iter = iterfunc }) do
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_missing_follow_in_assign_index_constraint() {
  let (_fixture, result) = fx_check!(
    r#"
        _._G = nil
        for _ in ... do
        break
        end
        for _ in function<t0,t0,t0>(l0)
        _,_._,l0 = l0,_,_._
        local _ = l0,{[_]=_,}
        _[{nil=_,}](_)
        end,{[_]=_,} do
        end
        _ -= _
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_missing_follow_in_checking_generic_mapping() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function _<U...,M...>(l0,l0,l0,l0,)
            l0(_(rshift),_()(_(if _ then _),))
            _()(_(_(_)))
        end
        _()(_()(_(true,_)),)
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        function _<Y...,U...,M...>(l0:any,l0,l0,...)
            _()(_,_()(_(_()),_))
            do end
        end
        do end
        _()(_(""),{})
        do end
        for _ in ... do
        end
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_missing_follow_in_function_call() {
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(

          r#"
        do end
        _ = if _ then true elseif _ then if _ then _ elseif _ then 2 .. {} elseif _._ then l0 else _ elseif _ then if ... then _ elseif {} then `` elseif _ then {_G=_,}
        type t0<),A,)...> = ({_G:any,write n0:any,write _:any<<A...>()->()>,write [any]:""""""""""""""""""""userda290013136ta:(0x000062900131369029001313690"""})|(l0.any)
    "#
    ,
      None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_missing_follow_in_instantiation2() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        _ = if {l0._,} then if _ then _ elseif rawset({[_]=_,[{_._,}]=_,}) then _ else {_._,} elseif rawset(_) then (true),""
    "#
    ,
      None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_missing_type_pack_follow() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local _ = {[0]=_,}
while _ do
do
local l2 = require(module0)
end
end
do end
function _(l0:typeof(_),l0,l0)
local l0 = require(module0)
_()(l0(),_,_(_())((_)))
do end
end
_()(_(if nil then _))("",_,_(_,(_)))
do end
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local _ = {_,}
while _ do
do
do end
end
end
_ = nil
function _(l0,l0,l0)
local l0 = require(module0)
_()(_(),_,_(_())(_,true)(_,_),l0)
do end
end
_()(_())("",_.n0,_,_(_,true,(_)))
do end
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_occurs_check_stack_overflow() {
  let (_fixture, result) = fx_check!(
    r#"
        _ = if _ then _
        for l0 in ... do
        type t0 = (()->((t0<t0...>)->())|(any))|(typeof(_))
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_pack_check_missing_follow() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
_ = n255
function _()
setmetatable(_)[_[xpcall(_,setmetatable(_,_()))]] /= xpcall(_,_)
_.n16(_,_)[_[_]] *= _
end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_simplify_crash() {
  let (_fixture, result) = fx_check!(
    r#"
        if _ then
            _ = nil
        else if _ and _ then
            _ = nil
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_simplify_is_check_on_bound_type() {
  let (_fixture, result) = bs_check!(
    r#"
        _[if _ then false],_,_._,log10 = {{[_]={_,},_G=not function():true
        _ = nil
        end,},[_[_ + true][_][_]]=_,sort=_,},_
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_simplify_table_indexer() {
  let (_fixture, result) = fx_check!(
    r#"
        _[_] += true
        _ = {
            [{
                [_] = _[_][if ... then _ else _](),
                [-1795162112] = function()
                end,
                [{
                    _G = function()
                    end
                }] = _(_(true)),
                _G = _
            }] = _,
            [_[not _][_]] = _(),
            _
        }

    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_fuzzer_unify_with_free_missing_follow() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
for _ in ... do
repeat
local function l0(l0)
end
_ = l0["aaaa"]
repeat
_ = true,_("")
_ = _[_]
until _
until _
repeat
_ = if _ then _,_()
_ = _[_]
until _
end
    "#,
    None,
  );
}

// Source: tests/TypeInfer.test.cpp:3024
#[test]
fn type_infer_generic_p_inference_with_optional_param_does_not_leak_nil() {
  use ulua_analysis::type_aliases::type_error_data::TypeErrorData;

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local function createElement<P>(component: (P) -> any, props: P?): any
            return nil
        end

        local function MyComponent(props: { x: number, y: number? })
            return nil
        end

        createElement(MyComponent, { x = 1 })
    "#
  );

  // cpp 侧先 ignoreMissingAnnotations（滤除 TypeAnnotationRequired）再断言无错误
  let errors: Vec<_> = result
    .errors
    .iter()
    .filter(|e| !matches!(&e.data, TypeErrorData::TypeAnnotationRequired(_)))
    .collect();
  assert!(errors.is_empty(), "{:?}", errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_getmetatable_infer_any_param() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function check(x): any
            return getmetatable(x)
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "(unknown) -> any",
      to_string_type_id(fixture.base.require_type_string("check"))
    );
  } else {
    assert_eq!(
      "(setmetatable<{+  +}, any>) -> any",
      to_string_type_id(fixture.base.require_type_string("check"))
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_getmetatable_infer_any_ret() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function spooky(x: any)
            return getmetatable(x)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(any) -> any",
    to_string_type_id(fixture.base.require_type_string("spooky"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_getmetatable_works_with_any() {
  let (_fixture, result) = bs_check!(
    r#"
        return {
            new = function(name: string)
                local self = newproxy(true) :: any

                getmetatable(self).__tostring = function()
                    return "Hello, I am " .. name
                end

                return self
            end,
        }
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_globals() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (mut fixture, result) = fx_check!(
    r#"
        --!nonstrict
        foo = true
        foo = "now i'm a string!"
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!("any", to_string_type_id(fixture.require_type_string("foo")));
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_globals_2() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (mut fixture, result) = fx_check!(
    r#"
        --!nonstrict
        foo = function() return 1 end
        foo = "now i'm a string!"
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("() -> (...any)", to_string_type_id(tm.wanted_type));
  assert_eq!("string", to_string_type_id(tm.given_type));
  assert_eq!(
    "() -> (...any)",
    to_string_type_id(fixture.require_type_string("foo"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_globals_are_banned_in_strict_mode() {
  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        foo = true
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let us = type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("foo", us.name());
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_handle_self_referential_has_prop_constraints() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local function calculateTopBarHeight(props)
        end
        local function isTopPage(props)
            local topMostOpaquePage
            if props.avatarRoute then
                topMostOpaquePage = props.avatarRoute.opaque.name
            else
                topMostOpaquePage = props.opaquePage
            end
        end

        function TopBarContainer:updateTopBarHeight(prevProps, prevState)
            calculateTopBarHeight(self.props)
            isTopPage(self.props)
            local topMostOpaquePage
            if self.props.avatarRoute then
                topMostOpaquePage = self.props.avatarRoute.opaque.name
                --                  ^--------------------------------^
            else
                topMostOpaquePage = self.props.opaquePage
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_if_statement() {
  let (mut fixture, result) = fx_check!(
    r#"
        local a
        local b

        if true then
            a = 'hello'
        else
            b = 999
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_string("a"))
    );
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_string("b"))
    );
  } else {
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string("a"))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string("b"))
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_if_then_else_bidirectional_inference() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        type foo = {
            bar: (() -> string)?,
        }
        local qux: foo = if false then {} else 10
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(err.given_type));
  assert_eq!("foo", to_string_type_id(err.wanted_type));
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_if_then_else_two_errors() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        type foo = {
            bar: () -> string,
        }
        local qux: foo = if false then {} else 10
    "#
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  let err1 = type_error_data_ref::<MissingProperties>(&result.errors[0])
    .expect("expected MissingProperties");
  assert_eq!("foo", to_string_type_id(err1.super_type()));
  assert_eq!("{  }", to_string_type_id(err1.sub_type()));

  let err2 = type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!("foo", to_string_type_id(err2.wanted_type));
  assert_eq!("number", to_string_type_id(err2.given_type));
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_index_expr_should_be_checked() {
  let (_fixture, result) = bs_check!(
    r#"
        local foo: any

        print(foo[(true).x])
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let up =
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  assert_eq!("boolean", to_string_type_id(up.table()));
  assert_eq!("x", up.key());
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_indexing_a_cyclic_intersection_does_not_crash() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local _
        if _ then
            while nil do
                _ = _
            end
        end
        if _[if _ then ""] then
            while nil do
                _ = if _ then ""
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_assignment_value_types() {
  let (_fixture, result) = fx_check!(
    r#"
local a: (number, number) -> number = function(a, b) return a - b end

a = function(a, b) return a + b end

local b: {number|string}
local c: {number|string}
b, c = {2, "s"}, {"b", 4}
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_assignment_value_types_mutable_lval() {
  let (_fixture, result) = bs_check!(
    r#"
local a = {}
a.x = 2
a = setmetatable(a, { __call = function(x) end })
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_in_nocheck_mode() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (mut fixture, result) = fx_check!(
    r#"
        --!nocheck
        function f(x)
            return x
        end
         -- we get type information even if there's type errors
        f(1, 2)
    "#
  );

  assert_eq!(
    "(any) -> (...any)",
    to_string_type_id(fixture.require_type_string("f"))
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_locals_via_assignment_from_its_call_site() {
  let (mut fixture, result) = fx_check!(
    r#"
        local a
        function f(x) a = x end
        f(1)
        f("foo")
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.require_type_string("a"))
    );
    assert_eq!(
      "(unknown) -> ()",
      to_string_type_id(fixture.require_type_string("f"))
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string("a"))
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_locals_with_nil_value() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options("local f = nil; f = 'hello world'", None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let f_type = fixture.require_type_string("f");
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!("string?", to_string_type_id(f_type));
  } else {
    assert_eq!(
      Some(PrimitiveType::STRING),
      fixture.get_primitive_type(f_type)
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_locals_with_nil_value_2() {
  let (mut fixture, result) = fx_check!(
    r#"
        local a = 2
        local b = a,nil
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("a"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("b"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_through_group_expr() {
  let (_fixture, result) = fx_check!(
    r#"
local function f(a: (number, number) -> number) return a(1, 3) end
f(((function(a, b) return a + b end)))
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_type_assertion_value_type() {
  let (_fixture, result) = fx_check!(
    r#"
local function f()
    return {4, "b", 3} :: {string|number}
end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_infer_types_of_globals() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (mut fixture, result) = bs_check!(
    r#"
        --!strict
        foo = 5
        print(foo)
    "#
  );

  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 3,
      column: 14,
    }))
  );
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Unknown global 'foo'; consider assigning to it first",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_invalide_deprecated_attribute_doesn_t_chrash_checker() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
@[deprecated{ reason = reasonString }]
function hello(x: number, y: number): number
    return x + y
end"#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_is_safe_integer_example() {
  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/isInteger"),
    String::from(
      r#"
        --!strict
        return function(value)
            return type(value) == "number" and value ~= math.huge and value == math.floor(value)
        end
    "#,
    ),
  );

  fixture.base.file_resolver.source.insert(
    String::from("game/MAX_SAFE_INTEGER"),
    String::from(
      r#"
        --!strict
        return 42
    "#,
    ),
  );

  fixture.base.file_resolver.source.insert(
    String::from("game/Util"),
    String::from(
      r#"
        --!strict
        local isInteger = require(script.Parent.isInteger)
        local MAX_SAFE_INTEGER = require(script.Parent.MAX_SAFE_INTEGER)
        return function(value)
        	return isInteger(value) and math.abs(value) <= MAX_SAFE_INTEGER
        end
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Util"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict() {
  let (_fixture, result) = bs_check!(
    r#"
        --!nonstrict
        function validate(stats, hits, misses)
            local checked = {}

            for _,l in ipairs(hits) do
                if not (stats[l] and stats[l] > 0) then
                    return false, string.format("expected line %d to be hit", l)
                end
                checked[l] = true
            end

            for _,l in ipairs(misses) do
                if not (stats[l] and stats[l] == 0) then
                    return false, string.format("expected line %d to be missed", l)
                end
                checked[l] = true
            end

            for k,v in pairs(stats) do
                if type(k) == "number" and not checked[k] then
                    return false, string.format("expected line %d to be absent", k)
                end
            end

            return true
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_2236_iterate_over_table_with_values_as_optional_types() {
  let _flags = [
    ScopedFastFlag::new(&fflag::LuauRefineNilFromTableIndexerResultType, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
  ];

  let (_fixture, result) = bs_check!(
    r#"
        --!strict
        local t: { number? } = {}

        for _, v in t do
            local x: number = v
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_iterate_over_local_table_with_optional_indexer_values() {
  let _flags = [
    ScopedFastFlag::new(&fflag::LuauRefineNilFromTableIndexerResultType, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
  ];

  let (_fixture, result) = bs_check!(
    r#"
        --!strict
        type TypeA = {Value: any}

        local list = {} :: {[string]: TypeA?}

        for index, a in list do
            a.Value = 1
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_iterate_over_table_with_optional_indexer_values() {
  let _flags = [
    ScopedFastFlag::new(&fflag::LuauRefineNilFromTableIndexerResultType, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
  ];

  let (_fixture, result) = bs_check!(
    r#"
        --!strict
        type Bar = {x: number}
        type Foo = {[string]: Bar?}

        function printAllClassNames(foo: Foo)
            for _, value in foo do
                print(value.x)
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_leading_ampersand() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Amp = & string
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_alias("Amp"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_leading_ampersand_no_type() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Amp = &
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected type, got <eof>",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.require_type_alias("Amp"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_leading_bar() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Bar = | number
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_alias("Bar"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_leading_bar_no_type() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Bar = |
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected type, got <eof>",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.require_type_alias("Bar"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_leading_bar_question_mark() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Bar = |?
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected type, got '?'",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "*error-type*?",
    to_string_type_id(fixture.require_type_alias("Bar"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_lti_must_record_contributing_locations() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (mut fixture, result) = bs_check!(
    r#"
        local function f(a)
            if math.random() > 0.5 then
                math.abs(a)
            else
                string.len(a)
            end
        end
    "#
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);

  let fn_ty = fixture.base.require_type_string("f");
  let function = get_type::get::<FunctionType>(fn_ty).expect("expected f to have a function type");

  let arg_ty = first(function.arg_types(), false).expect("expected first argument");
  let module = unsafe { &*fixture.base.get_main_module(false) };
  let locations = module
    .upper_bound_contributors
    .find(&arg_ty)
    .expect("expected upper-bound contributors for f argument");
  assert_eq!(2, locations.len());
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_luau_resolves_symbols_the_same_way_lua_does() {
  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        function Funky()
            local a: number = foo
        end

        local foo: string = 'hello'
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_multiple_assignment() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local function requireString(arg: string) end
        local function requireNumber(arg: number) end

        local function f(): ...number end

        local w: "a", x, y, z = "a", 1, f()
        requireString(w)
        requireNumber(x)
        requireNumber(y)
        requireNumber(z)
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_nested_functions_can_depend_on_outer_generics() {
  let (mut fixture, result) = fx_check!(
    r#"
        function name<P>(arg1: P)
            return function(what: P) return what end
        end

        local funcTest = name(nil)
        local out = funcTest(1) -- Doesn't report type mismatch error anymore
    "#
  );

  assert_eq!(
    "(nil) -> nil",
    to_string_type_id(fixture.require_type_string("funcTest"))
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("nil", to_string_type_id(tm.wanted_type));
  assert_eq!("number", to_string_type_id(tm.given_type));
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_no_heap_use_after_free_error() {
  let (_fixture, result) = bs_check!(
    r#"
        --!nonstrict
        _ += _:n0(xpcall,_)
        local l0
        do end
        while _ do
            function _:_()
                _ += _(_._(_:n0(xpcall,_)))
            end
        end
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_no_infinite_loop_when_trying_to_unify_uh_this() {
  let (_fixture, result) = fx_check!(
    r#"
        function _(l22,l0):((((boolean)|(t0))|(t0))&(()->(()->(()->()->{},(t0<t22>)|(t0)),any)))
            return function():t0<t0>
            end
        end
        type t0<t0> = ((typeof(_))|(any))|(typeof(_))
        _()
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_no_stack_overflow_from_isoptional() {
  let (mut fixture, result) = fx_check!(
    r#"
        function _(l0:t0): (any, ()->())
            return 0,_
        end

        type t0 = t0 | {}
        _(nil)
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let t0 = fixture.lookup_type("t0").expect("expected type alias t0");
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!("any", to_string_type_id(t0));
  } else {
    assert_eq!("*error-type*", to_string_type_id(t0));
  }

  assert!(
    result
      .errors
      .iter()
      .any(|error| type_error_data_ref::<OccursCheckFailed>(error).is_some()),
    "expected OccursCheckFailed: {:?}",
    result.errors
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_no_stack_overflow_from_isoptional_2() {
  let (_fixture, result) = bs_check!(
    r#"
        function _(l0:({})|(t0)):((((typeof((xpcall)))|(t96<t0>))|(t13))&(t96<t0>),()->typeof(...))
            return 0,_
        end

        type t0<t107> = ((typeof((_G)))|(({})|(t0)))|(t0)
        _(nil)

        local t: ({})|(t0)
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_non_standalone_constraint_solving_incomplete_is_hidden() {
  let _flags = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true),
  ];

  let (_fixture, result) = fx_check!(
    r#"
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
        local x: number = true
    "#
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<ConstraintSolvingIncompleteError>(&result.errors[0])
    .expect("expected ConstraintSolvingIncompleteError");
  type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_obvious_type_error_in_nocheck_mode() {
  let (_fixture, result) = fx_check!(
    r#"
        --!nocheck
        local x: string = 5
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_occurs_check_does_not_recurse_forever_if_asked_to_traverse_a_cyclic_type() {
  let (_fixture, result) = fx_check!(
    r#"
         --!strict
        function u(t, w)
            u(u, t)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_occurs_isnt_always_failure() {
  let (_fixture, result) = fx_check!(
    r#"
function f(x, c)                   -- x : X
    local y = if c then x else nil -- y : X?
    local z = if c then x else nil -- z : X?
    y = z
end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_oss_1815_verbatim() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        local item: "foo" = "bar"
        item = if true then "foo" else "foo"

        local item2: "foo" = if true then "doge" else "doge2"
    "#
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location {
      begin: Position {
        line: 2,
        column: 28
      },
      end: Position {
        line: 2,
        column: 33
      },
    },
    result.errors[0].location
  );
  let err1 = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("\"foo\"", to_string_type_id(err1.wanted_type));
  assert_eq!("\"bar\"", to_string_type_id(err1.given_type));

  assert_eq!(
    Location {
      begin: Position {
        line: 5,
        column: 42
      },
      end: Position {
        line: 5,
        column: 48
      },
    },
    result.errors[1].location
  );
  let err2 = type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!("\"foo\"", to_string_type_id(err2.wanted_type));
  assert_eq!("\"doge\"", to_string_type_id(err2.given_type));

  assert_eq!(
    Location {
      begin: Position {
        line: 5,
        column: 54
      },
      end: Position {
        line: 5,
        column: 61
      },
    },
    result.errors[2].location
  );
  let err3 = type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
  assert_eq!("\"foo\"", to_string_type_id(err3.wanted_type));
  assert_eq!("\"doge2\"", to_string_type_id(err3.given_type));
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_promote_tail_type_packs() {
  let (_fixture, result) = fx_check!(
    r#"
        --!strict

        local A: any = nil

        local C
        local D = A(
            A({}, {
                __call = function(a): string
                    local E: string = C(a)
                    return E
                end
            }),
            {
                F = function(s: typeof(C))
                end
            }
        )

        function C(b: any): string
            return ''
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_react_lua_follow_free_type_ub() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(

          r#"
        return function(Roact)
            local Tree = Roact.Component:extend("Tree")

            function Tree:render()
                local breadth, components, depth, id, wrap =
                    self.props.breadth, self.props.components, self.props.depth, self.props.id, self.props.wrap
                local Box = components.Box
                if depth == 0 then
                    Roact.createElement(Box, {})
                else
                    Roact.createElement(Tree, {})
                end

            end
        end
    "#
    ,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_read_table_type_refinements_persist_scope() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(

          r#"
_ = {n0=_,},if _._ then ... else if _[if _ then _ else ({nil,})].setmetatable then if _ then _ elseif l0 then ... elseif _.n0 then _ elseif function<A>(l0)
return _._G,_
end then _._G else ...
    "#
    ,
      None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        local TRUE: true = true

        local function matches(value, t: true)
            if value then
                return true
            end
        end

        local function readValue(breakpoint)
            if matches(breakpoint, TRUE) then
                readValue(breakpoint)
            end
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "(unknown) -> ()",
      to_string_type_id(fixture.base.require_type_string("readValue"))
    );
  } else {
    assert_eq!(
      "<a>(a) -> ()",
      to_string_type_id(fixture.base.require_type_string("readValue"))
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter_2() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function readValue(breakpoint)
            if type(breakpoint) == 'number' then
                readValue(breakpoint)
            end
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "(unknown) -> ()",
      to_string_type_id(fixture.base.require_type_string("readValue"))
    );
  } else {
    assert_eq!(
      "(number) -> ()",
      to_string_type_id(fixture.base.require_type_string("readValue"))
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_recursive_metatable_crash() {
  let (_fixture, result) = bs_check!(
    r#"
local function getIt()
    local y
    y = setmetatable({}, y)
    return y
end
local a = getIt()
local b = getIt()
local c = a or b
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_regexp_hang() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
local outln, group_id, verb_flags = {}, {}, {
    newline = 1,
    newline_seq = 1,
    not_empty = 0
}
if not escape_c then
elseif escape_c >= 48 and escape_c <= 57 then
elseif escape_c == 69 then
elseif escape_c == 81 then
elseif escape_c == 78 then
    if codes[i] ~= 125 or i == start_i then
    end
    table.insert(outln, code_point)
elseif escape_c == 80 or escape_c == 112 then
    if script_set then
    elseif not valid_categories[c_name]then
    else
        table.insert(outln, { 'category', negate, c_name })
    end
elseif escape_c == 103 and (codes[i + 1] == 123 or codes[i + 1] >= 48 and codes[i + 1] <= 57)then
elseif escape_c == 111 then
elseif escape_c == 120 then
else
    table.insert(outln, esc_char or escape_c)
end

for i, v in ipairs(outln)do
    if type(v) == 'table' and (v[1] == 40 or v[1] == 'quantifier' and type(v[5]) == 'table' and v[5][1] == 40)then
        v = v[5]
    elseif type(v) == 'table' and (v[1] == 'backref' or v[1] == 'recurmatch')then
        for i1, v1 in ipairs(outln)do
            break
        end
    end
end
    "#
    ,
      None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_self_bound_due_to_compound_assign() {
  let mut fixture = Fixture::fixture_bool(false);
  fixture.load_definition(
    r#"
        declare extern type Camera with
            CameraType: string
            CFrame: number
        end
    "#,
    false,
  );

  let result = fixture.check_string_optional_frontend_options(

          r#"
        --!strict
        function MT_UPDATE(CAMERA: Camera, Enum: any, totalOffsets: number, focusToCFrame: number, magnitude: number)
            if CAMERA.CameraType ~= Enum.CameraType.Custom then
                return
            end

            local goalCFrame = (CAMERA.CFrame) * totalOffsets
            if goalCFrame ~= CAMERA.CFrame then
                goalCFrame -= (focusToCFrame * magnitude) -- Offset the goalCFrame the raycast direction based on the cutoff distance.
            end
        end

        return {}
    "#
    ,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_should_be_able_to_infer_this_without_stack_overflowing() {
  let (_fixture, result) = fx_check!(
    r#"
        local function f(x, y)
            return x or y
        end

        local function dont_crash(x, y)
            local z: typeof(f(x, y)) = f(x, y)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_standalone_constraint_solving_incomplete_is_hidden() {
  let mut fixture = Fixture::fixture_bool(false);

  let _flags = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true),
    ScopedFastFlag::new(
      &fflag::DebugLuauAlwaysShowConstraintSolvingIncomplete,
      false,
    ),
  ];

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_statements_are_topologically_sorted() {
  let (_fixture, result) = fx_check!(
    r#"
        function foo()
            return bar(999), bar("hi")
        end

        function bar(i)
            return i
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_stringify_nested_unions_with_optionals() {
  let (mut fixture, result) = fx_check!(
    r#"
        --!strict
        local a: number | (string | boolean) | nil
        local b: number = a
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
  assert_eq!(
    "(boolean | number | string)?",
    to_string_type_id(tm.given_type)
  );
}

// Source: tests/TypeInfer.test.cpp:2995
#[test]
fn type_infer_table_insert_and_unpack_generic_order_independence() {
  let (_fixture, result) = bs_check!(
    r#"
        local tbl = {}
        for i=0, 3 do
            table.insert(tbl, i)
        end
        return table.unpack(tbl)
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_after_error_recovery() {
  let (mut fixture, result) = fx_check!(
    r#"
        local x =
        local a = 7
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let a_type = fixture.require_type_string("a");
  assert_eq!(
    Some(PrimitiveType::NUMBER),
    fixture.get_primitive_type(a_type)
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_after_error_recovery_no_assert() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options("function +() local _ = true end", None);

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_after_error_recovery_no_replacement_name_in_error() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  {
    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    let result = fixture.base.check_string_optional_frontend_options(
      r#"
            --!strict
            local t = { x = 10, y = 20 }
            return t.
        "#,
      None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }

  {
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    let result = fixture.base.check_string_optional_frontend_options(
      r#"
            --!strict
            export type = number
            export type = string
        "#,
      None,
    );
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }

  {
    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    let result = fixture.base.check_string_optional_frontend_options(
      r#"
            --!strict
            function string.() end
        "#,
      None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }

  {
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    let result = fixture.base.check_string_optional_frontend_options(
      r#"
            --!strict
            local function () end
            local function () end
        "#,
      None,
    );
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }

  {
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    let result = fixture.base.check_string_optional_frontend_options(
      r#"
            --!strict
            local dm = {}
            function dm.() end
            function dm.() end
        "#,
      None,
    );
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_error() {
  let mut fixture = Fixture::fixture_bool(false);
  let result =
    fixture.check_string_optional_frontend_options("local a = 7   local b = 'hi'   a = b", None);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number | string",
      to_string_type_id(fixture.require_type_string("a"))
    );
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 0,
          column: 35
        },
        end: Position {
          line: 0,
          column: 36
        }
      },
      result.errors[0].location
    );

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(fixture.get_builtins().number_type, tm.wanted_type);
    assert_eq!(fixture.get_builtins().string_type, tm.given_type);
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_error_2() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options("local a = 7   a = 'hi'", None);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number | string",
      to_string_type_id(fixture.require_type_string("a"))
    );
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 0,
          column: 18
        },
        end: Position {
          line: 0,
          column: 22
        }
      },
      result.errors[0].location
    );

    let a_type = fixture.require_type_string("a");
    let string_type = fixture.get_builtins().string_type;
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(a_type, tm.wanted_type);
    assert_eq!(string_type, tm.given_type);
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_hello_world() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options("local a = 7", None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_if_else_expressions_1() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture
    .check_string_optional_frontend_options(r#"local a = if true then "true" else "false""#, None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_if_else_expressions_2() {
  let (mut fixture, result) = fx_check!(
    r#"
local a = if false then "a" elseif false then "b" else "c"
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_string("a"))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_if_else_expressions_expected_type_1() {
  let (mut fixture, result) = fx_check!(
    r#"
type X = {number | string}
local a: X = if true then {"1", 2, 3} else {4, 5, 6}
"#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "{number | string}",
    to_string_type_id_to_string_options(fixture.require_type_string("a"), &mut opts)
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_if_else_expressions_expected_type_2() {
  let (_fixture, result) = fx_check!(
    r#"
local a: number? = if true then 1 else nil
"#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_if_else_expressions_expected_type_3() {
  let (_fixture, result) = bs_check!(
    r#"
local function times<T>(n: any, f: () -> T)
    local result: {T} = {}
    local res = f()
    table.insert(result, if true then res else n)
    return result
end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_if_else_expressions_type_union() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture
    .check_string_optional_frontend_options(r#"local a: number? = if true then 42 else nil"#, None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let mut opts = ToStringOptions::new(true);
  assert_eq!(
    "number?",
    to_string_type_id_to_string_options(fixture.require_type_string("a"), &mut opts)
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_interpolated_string_basic() {
  let (_fixture, result) = fx_check!(
    r#"
        local foo: string = `hello {"world"}`
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_interpolated_string_constant_type() {
  let (_fixture, result) = fx_check!(
    r#"
        local foo: "hello" = `hello`
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_interpolated_string_with_invalid_expression() {
  let (_fixture, result) = fx_check!(
    r#"
        local function f(x: number) end

        local foo: string = `hello {f("uh oh")}`
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_tc_propagation() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options("local a = 7   local b = a", None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let b_type = fixture.require_type_string("b");
  assert_eq!(
    Some(PrimitiveType::NUMBER),
    fixture.get_primitive_type(b_type)
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type() {
  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);

  let mut fixture = Fixture::default();
  let _result = fixture.check_string_optional_frontend_options(
    r#"
        local any = nil :: any

        function f1(x)
            x:m()
            local _ = x.A.p.a
        end

        function f2(x)
            local _ = x.d
        end

        function f3(x)
            local a = ""
            a = x.d.p
            local _ = undef[x.a]
        end

        function f4(x)
            f2(x)
            if undef and x and x:m() then
                any(x)
                return
            end
            f3(x)
            for _, v in any.x do
                local a = x[v].p
            end
            a.b = x
            if x.q ~= nil then
                f1(x) -- things go bad here
            end
        end

        return f4
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_type_errors_infer_types() {
  let (mut fixture, result) = fx_check!(
    r#"
        local err = (true).x
        local c = err.Parent.Reward.GetChildren
        local d = err.Parent.Reward
        local e = err.Parent
        local f = err
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let err =
    type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  assert_eq!("boolean", to_string_type_id(err.table()));
  assert_eq!("x", err.key());

  if fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string("c"))
    );
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string("d"))
    );
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string("e"))
    );
    assert_eq!(
      "*error-type*",
      to_string_type_id(fixture.require_type_string("f"))
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_type_infer_cache_limit_normalizer() {
  let _normalize_cache_limit = ScopedFastInt::new(&fint::LuauNormalizeCacheLimit, 10);

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local x : ((number) -> number) & ((string) -> string) & ((nil) -> nil) & (({}) -> {})
        local y : (number | string | nil | {}) -> (number | string | nil | {}) = x
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert_eq!(
    "Code is too complex to typecheck! Consider simplifying the code around this area",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_type_infer_recursion_limit_no_ice() {
  let _recursion_limit = ScopedFastInt::new(&fint::LuauTypeInferRecursionLimit, 2);

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function complex()
          function _(l0:t0): (any, ()->())
              return 0,_
          end
          type t0 = t0 | {}
          _(nil)
        end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Type contains a self-recursive construct that cannot be resolved",
      to_string_type_error(&result.errors[0])
    );
  } else {
    assert_eq!(
      "Code is too complex to typecheck! Consider simplifying the code around this area",
      to_string_type_error(&result.errors[0])
    );
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_type_infer_recursion_limit_normalizer() {
  let _recursion_limit = ScopedFastInt::new(&fint::LuauTypeInferRecursionLimit, 10);

  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        function f<a,b,c,d,e,f,g,h,i,j>()
            local x : a&b&c&d&e&f&g&h&(i?)
            local y : (a&b&c&d&e&f&g&h&i)? = x
        end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let too_complex =
    "Code is too complex to typecheck! Consider simplifying the code around this area";

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    let expected_locations = [
      Location {
        begin: Position {
          line: 2,
          column: 22,
        },
        end: Position {
          line: 2,
          column: 42,
        },
      },
      Location {
        begin: Position {
          line: 3,
          column: 22,
        },
        end: Position {
          line: 3,
          column: 42,
        },
      },
      Location {
        begin: Position {
          line: 3,
          column: 22,
        },
        end: Position {
          line: 3,
          column: 41,
        },
      },
    ];

    for (error, expected_location) in result.errors.iter().zip(expected_locations.iter()) {
      assert_eq!(*expected_location, error.location);
      assert_eq!(too_complex, to_string_type_error(error));
    }
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      Location {
        begin: Position {
          line: 3,
          column: 12,
        },
        end: Position {
          line: 3,
          column: 46,
        },
      },
      result.errors[0].location
    );
    assert_eq!(too_complex, to_string_type_error(&result.errors[0]));
  }
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_type_remover_heap_use_after_free() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        _ = if l0.n0.n0 then {n4(...,setmetatable(setmetatable(_),_)),_ == _,} elseif _.ceil._ then _ elseif _ then not _
    "#
    ,
      None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        do
        _ = if _[_] then {[_(``)]="y",} elseif _ then _ elseif _[_] then "" elseif _ then _ elseif _[_] then {} elseif _[_] then false else ""
        end
    "#
    ,
      None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local l249 = require(module0)
        _,_ = {[`{_}`]=_,[_._G._]=(_)(),[_["" + _]._G]={_=_,_=_,[_._G[_]._]=_G,},},_,(_)()
    "#,
    None,
  );
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_typechecking_in_type_guards() {
  let (_fixture, result) = bs_check!(
    r#"
local a = type(foo) == 'nil'
local b = typeof(foo) ~= 'nil'
    "#
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Unknown global 'foo'; consider assigning to it first",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "Unknown global 'foo'; consider assigning to it first",
    to_string_type_error(&result.errors[1])
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_typeof_cannot_refine_builtin_alias() {
  let mut fixture = Fixture::default();
  let frontend = fixture.get_frontend();

  let global_scope = frontend.globals.global_scope();
  // (b) 类句柄：被测 API（TableType::…scope 形参）取 `*mut Scope`，Arc::as_ptr
  // 仅取址不解引用，底层块由本帧 `global_scope` 强引用保活。
  let global_scope_ptr: *mut Scope = Arc::as_ptr(&global_scope).cast_mut();
  let arena = frontend.globals.global_types_mut();

  unfreeze(arena);

  let global_table_ty = arena.add_type(TableType::table_type_table_state_type_level_scope(
    TableState::Sealed,
    TypeLevel::default(),
    null_mut(),
  ));

  // Safety: global_scope_ptr 为 Arc<Scope>（本帧强引用保活，非空存活）堆块地址；
  // 对应 cpp exportedTypeBindings 写入，insert 借用随语句结束，单线程无第二可变借用。
  unsafe {
    (*global_scope_ptr).exported_type_bindings.insert(
      String::from("GlobalTable"),
      TypeFun::type_fun_type_id(global_table_ty),
    );
  }

  freeze(arena);

  let _result = fixture.check_string_optional_frontend_options(
    r#"
        function foo(x)
            if typeof(x) == 'GlobalTable' then
            end
        end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_types_stored_in_ast_resolved_types() {
  let mut fixture = Fixture::default();
  let result = fixture.check_string_optional_frontend_options(
    r#"
        type alias = typeof("hello")
        local function foo(param: alias)
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let node = find_node_at_position_source_module_position(
    fixture.main_source_module(),
    Position {
      line: 2,
      column: 16,
    },
  );
  assert!(!node.is_null());

  let ty = fixture.lookup_type("alias").expect("expected alias type");

  // 判型 + 下转 + 判空一步折叠为 Option（cpp `node->as<AstExprFunction>()`），
  // 替代 `ast_node_as` 直调核心 + `!is_null` 断言 + `(*func)` 裸解引用。
  let func =
    unsafe { ast_node_try_as_ptr::<AstExprFunction>(node) }.expect("expected AstExprFunction");
  assert_eq!(1, func.args.len());

  let arg = *func.args.as_slice().first().expect("expected function arg");
  // `Node` 经 `Deref` 直读字段,无需裸解引用。
  let annotation = arg.annotation;
  assert!(!annotation.is_null());

  let module = unsafe { &*fixture.get_main_module(false) };
  assert_eq!(
    Some(&ty),
    module.ast_resolved_types.find(&(annotation as *const _))
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_unify_nearly_identical_recursive_types() {
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let (_fixture, result) = fx_check!(
    r#"
        local o
        o:method()

        local p
        p:method()

        o = p
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_unterminated_function_body_causes_constraint_generator_crash() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
export type t = {
	func : typeof(
		function
	)
}

export type t1 = t12

export type t2 = {}

export type t3 = {
	foo:number
	bar:number
}

export type t4 = "foobar"

export type t5 = string

export type t6 = number

export type t7 = "foobar"

export type t8 = "foobar"

export type t9 = typeof(1)

export type t10 = typeof(1)

export type t11 = typeof(1)

export type t12 = {
	b:number
	pb:number
}
"#,
    None,
  );
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_visit_error_nodes_in_lvalue() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        (::,
    "#
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_warn_on_lowercase_parent_property() {
  let (_fixture, result) = bs_check!(
    r#"
        local M = require(script.parent.DoesNotMatter)
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let ed = type_error_data_ref::<DeprecatedApiUsed>(&result.errors[0])
    .expect("expected DeprecatedApiUsed");
  assert_eq!("parent", ed.symbol);
}

// Source: `tests/TypeInfer.test.cpp`
#[test]
fn type_infer_weird_case() {
  let (_fixture, result) = bs_check!(
    r#"
        local function f() return 4 end
        local d = math.deg(f())
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
