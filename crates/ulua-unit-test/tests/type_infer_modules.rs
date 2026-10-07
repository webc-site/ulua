// 集成测试共享样板：fixture 前奏宏 fx_check!/bs_check! 由 common 上提，#[macro_export] 后按裸名调用。
mod common;
use ulua_analysis::type_aliases::module_name_type::ModuleName;

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_bound_free_table_export_is_ok() {
  use ulua_unit_test::records::fixture::Fixture;

  let (_fixture, result) = fx_check!(
    r#"
local n = {}
function n:Clone() end

local m = {}

function m.a(x)
    x:Clone()
end

function m.b()
    m.a(n)
end

return m
"#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_check_imported_module_names() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
return function(...) end
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local l0 = require(game.A)
return l0
    "#,
  );

  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local l0 = require(game.B)
if true then
    local l1 = require(game.A)
end
return l0
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = fixture.base.get_main_module(false);
  assert!(!module.is_null());

  // Safety: 主模块由 fixture frontend 的 resolver 保有（存活、非空）；&* 物化只读借用，引用不出帧
  let scopes = unsafe { &(*module).scopes };
  assert_eq!(4, scopes.len());

  let root_scope = &scopes[0].1;
  let block_scope = &scopes[3].1;
  assert_eq!(
    Some(&ModuleName::from("game/B")),
    root_scope.imported_modules.get("l0")
  );
  assert_eq!(
    Some(&ModuleName::from("game/A")),
    block_scope.imported_modules.get("l1")
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_constrained_anyification_clone_immutable_types() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
return function(...) end
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local l0 = require(game.A)
return l0
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_cross_module_function_mutation() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
function test2(a: number, b: string)
    return 1
end

return test2
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
function wrapper<A...>(f: (A...) -> number, ...: A...)
end

local test2 = require(game.A)

return wrapper(test2, 1, "")
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_cross_module_table_freeze() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_pack_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        return {
            a = 1,
        }
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        return table.freeze(require(game.A))
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let a_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(
    "{ a: number }",
    to_string_type_pack_id(a_module.return_type)
  );

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "{ read a: number }"
  } else {
    "{ a: number }"
  };
  assert_eq!(expected, to_string_type_pack_id(b_module.return_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_custom_require_global() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let (_fixture, result) = bs_check!(
    r#"
--!nonstrict
require = function(a) end

local crash = require(game.A)
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_cycles_dont_make_everything_any() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_pack_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        local module = {}

        function module.foo()
            return 2
        end

        function module.bar()
            local m = require(game.B)
            return m.foo() + 1
        end

        return module
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local module = {}

        function module.foo()
            return 2
        end

        function module.bar()
            local m = require(game.A)
            return m.foo() + 1
        end

        return module
    "#,
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!("module", to_string_type_pack_id(b_module.return_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_dcr_require_basic() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        return {
            a = 1,
        }
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local b = A.a
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let b_type = fixture.base.require_type_module_ptr_string(&b_module, "b");
  assert_eq!("number", to_string_type_id(b_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_do_not_modify_imported_types() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
export type Type = { unrelated: boolean }
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local types = require(game.A)
type Type = types.Type
local x: Type = {}
function x:Destroy(): () end
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_do_not_modify_imported_types_2() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
export type Type = { x: { a: number } }
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local types = require(game.A)
type Type = types.Type
local x: Type = { x = { a = 2 } }
type Rename = typeof(x.x)
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_do_not_modify_imported_types_3() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
local y = setmetatable({}, {})
export type Type = { x: typeof(y) }
return { x = y }
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local types = require(game.A)
type Type = types.Type
local x: Type = types
type Rename = typeof(x.x)
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_do_not_modify_imported_types_4() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
export type Array<T> = {T}
local arrayops = {}
function arrayops.foo(x: Array<any>) end
return arrayops
    "#,
  );
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local arrayops = require(game.A)

local tbl = {}
tbl.a = 2
function tbl:foo(b: number, c: number)
    -- introduce BoundType to imported type
    arrayops.foo(self._regions)
end
-- this alias decreases function type level and causes a demotion of its type
type Table = typeof(tbl)
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_do_not_modify_imported_types_5() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
export type Type = {x: number, y: number}
local arrayops = {}
function arrayops.foo(x: Type) end
return arrayops
    "#,
  );
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local arrayops = require(game.A)

local tbl = {}
tbl.a = 2
function tbl:foo(b: number, c: number)
    -- introduce bound_to TableType to imported type
    self.x.a = 2
    arrayops.foo(self.x)
end
-- this alias decreases function type level and causes a demotion of its type
type Table = typeof(tbl)
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_ensure_free_variables_are_generialized_across_function_boundaries() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
-- Roughly taken from react-shallow-renderer
function createUpdater(renderer)
    local updater = {
        _renderer = renderer,
    }

    function updater.enqueueForceUpdate(publicInstance, callback, _callerName)
        updater._renderer.render(
            updater._renderer,
            updater._renderer._element,
            updater._renderer._context
        )
    end

    function updater.enqueueReplaceState(
        publicInstance,
        completeState,
        callback,
        _callerName
    )
        updater._renderer.render(
            updater._renderer,
            updater._renderer._element,
            updater._renderer._context
        )
    end

    function updater.enqueueSetState(publicInstance, partialState, callback, _callerName)
        local currentState = updater._renderer._newState or publicInstance.state
        updater._renderer.render(
            updater._renderer,
            updater._renderer._element,
            updater._renderer._context
        )
    end

    return updater
end

local ReactShallowRenderer = {}

function ReactShallowRenderer:_reset()
    self._updater = createUpdater(self)
end

return ReactShallowRenderer
    "#,
  );

  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local ReactShallowRenderer = require(game.A);
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_ensure_scope_is_nullptr_after_shallow_copy() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend().options.retain_full_type_graphs = false;

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
-- Roughly taken from ReactTypes.lua
type CoreBinding<T> = {}
type BindingMap = {}
export type Binding<T> = CoreBinding<T> & BindingMap

return {}
    "#,
  );

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local Types = require(game.A)
type Binding<T> = Types.Binding<T>
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_export_class() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        export class Point
            public x: number
            public y: number

            function __tostring(self): string
                return `Point x={self.x} y={self.y}`
            end
        end
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        local A = require(game.A)

        local a: A.Point = A.Point.new { x=2, y=3 }

        local x, y = a.x, a.y
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_module_name_string("game/B", "x"))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_module_name_string("game/B", "y"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_exported_module_basic() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export local version = "1.0.0"
        export const name = "test module"
        export local count = 41

        count += 1
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local version = A.version
        local name = A.name
        local count = A.count
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "count"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "version"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "name"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_exported_module_function() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export function add(a: number, b: number): number
            return a + b
        end

        export function greet(name: string): string
            return "Hello, " .. name
        end

        export function noop()
            -- do nothing
        end
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local add = A.add
        local greet = A.greet
        local noop = A.noop
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "(number, number) -> number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "add"))
  );
  assert_eq!(
    "(string) -> string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "greet"))
  );
  assert_eq!(
    "(...any) -> ()",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "noop"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_exported_module_mutual_recursive_functions() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export local a, b

        function a()
            return b() + 1
        end

        function b()
            return 42
        end
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "(...any) -> number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "a"))
  );
  assert_eq!(
    "(...any) -> number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "b"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_exported_module_unassigned_local_stays_nil() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export local a
        export local b = function() return 1 end
        b = nil
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "nil",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "a"))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "b"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_exported_multret() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        local function huh()
            return 42, "huh", false
        end

        export local a, b, c = huh()
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
        local c = A.c
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "a"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "b"))
  );
  assert_eq!(
    "boolean",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "c"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_exported_partial_multret() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        local function huh()
            return "huh", false
        end

        export local a, b, c = 42, huh()
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
        local c = A.c
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "a"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "b"))
  );
  assert_eq!(
    "boolean",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "c"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_fuzz_anyify_variadic_return_must_follow() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
return unpack(l0[_])
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_general_require_call_expression() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
--!strict
return { def = 4 }
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
--!strict
local tbl = { abc = require(game.A) }
local a : string = ""
a = tbl.abc.def
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'string', but got 'number'",
    to_string_type_error(&result.errors[0])
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_general_require_type_mismatch() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
return { def = 4 }
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local tbl: string = require(game.A)
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Expected this to be 'string', but got '{ def: number }'",
    to_string_type_error(&result.errors[0])
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_internal_type_errors_are_only_reported_once() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_pack_id, records::internal_error::InternalError,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _magic_types = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
return function(): { X: _luau_blocked_type, Y: _luau_blocked_type } return nil :: any end
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<InternalError>(&result.errors[0]).is_some());

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(
    "(...any) -> { X: *error-type*, Y: *error-type* }",
    to_string_type_pack_id(module.return_type)
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_internal_types_are_scrubbed_from_module() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_pack_id,
    records::{
      constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
      internal_error::InternalError,
    },
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _magic_types = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
return function(): _luau_blocked_type return nil :: any end
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<ConstraintSolvingIncompleteError>(&result.errors[0]).is_some());
  assert!(type_error_data_ref::<InternalError>(&result.errors[1]).is_some());

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(
    "(...any) -> *error-type*",
    to_string_type_pack_id(module.return_type)
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_invalid_alias_should_export_as_error_type() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::recursive_restraint_violation::RecursiveRestraintViolation,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        export type bad<T> = {bad<{T}>}
        return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        local a_mod = require(game.A)
        local f: a_mod.bad<number>
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0]).is_some());

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let f_type = fixture.base.require_type_module_ptr_string(&b, "f");
  assert_eq!("bad<number>", to_string_type_id(f_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_invalid_local_alias_shouldnt_shadow_imported_type() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::recursive_restraint_violation::RecursiveRestraintViolation,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        export type bad<T> = {T}
        return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        local a_mod = require(game.A)
        type bad<T> = {bad<{T}>}
        type fine<T> = a_mod.bad<T>
        local f: fine<number>
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0]).is_some());

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let f_type = fixture.base.require_type_module_ptr_string(&b, "f");
  assert_eq!("fine<number>", to_string_type_id(f_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_leaky_generics() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local Cache = {}

        Cache.settings = {}

        function Cache.should_cache(url)
            for key, _ in pairs(Cache.settings) do
                return key
            end

            return ""
        end

        function Cache.is_cached(url)
            local setting_key = Cache.should_cache(url)
            local settings = Cache.settings[setting_key]

            return settings
        end

        return Cache
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(unknown) -> unknown",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 13,
      column: 23
    }))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_module_type_conflict() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
export type T = { x: number }
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
export type T = { x: string }
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/C",
    r#"
local A = require(game.A)
local B = require(game.B)
local a: A.T = { x = 2 }
local b: B.T = a
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/C"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Expected this to be 'T' from 'game/B', but got 'T' from 'game/A'; \naccessing `x` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`"
  } else {
    "Expected this to be exactly 'T' from 'game/B', but got 'T' from 'game/A'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'string', but got 'number'"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_module_type_conflict_instantiated() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
export type Wrap<T> = { x: T }
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
local A = require(game.A)
export type T = A.Wrap<number>
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/C",
    r#"
local A = require(game.A)
export type T = A.Wrap<string>
return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/D",
    r#"
local A = require(game.B)
local B = require(game.C)
local a: A.T = { x = 2 }
local b: B.T = a
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/D"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let expected = if !fflag::DebugLuauForceOldSolver.get() {
    "Expected this to be 'T' from 'game/C', but got 'T' from 'game/B'; \naccessing `x` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`"
  } else {
    "Expected this to be exactly 'T' from 'game/C', but got 'T' from 'game/B'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'string', but got 'number'"
  };
  assert_eq!(expected, to_string_type_error(&result.errors[0]));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_non_exported_class() {
  use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        class Point
            public x: number
            public y: number

            function __tostring(self): string
                return `Point x={self.x} y={self.y}`
            end
        end

        return {Point=Point}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        local A = require(game.A)

        local a: A.Point = A.Point.new { x=2, y=3 }
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let err =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol error");
  assert_eq!("A.Point", err.name());
  assert_eq!(Context::Type, err.context());
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_require() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        local function hooty(x: number): string
            return "Hi there!"
        end

        return {hooty=hooty}
    "#,
  );

  let source_b = if !fflag::DebugLuauForceOldSolver.get() {
    r#"
            local Hooty = require(game.A)

            local h = 4
            local i = Hooty.hooty(h)
        "#
  } else {
    r#"
            local Hooty = require(game.A)

            local h -- free!
            local i = Hooty.hooty(h)
        "#
  };

  fixture.base.file_resolver.source.insert("game/B", source_b);

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));

  let i_type = fixture.base.require_type_module_ptr_string(&b_module, "i");
  assert_eq!("string", to_string_type_id(i_type));

  let h_type = fixture.base.require_type_module_ptr_string(&b_module, "h");
  assert_eq!("number", to_string_type_id(h_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_require_a_variadic_function() {
  use ulua_analysis::{
    functions::{begin_type_pack, end_type_pack, follow_type::follow, get_type, get_type_pack},
    records::{function_type::FunctionType, variadic_type_pack::VariadicTypePack},
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        local T = {}
        function T.f(...) end
        return T
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        local A = require(game.A)
        local f = A.f
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let f = follow(fixture.base.require_type_module_ptr_string(&b_module, "f"));

  let ftv = get_type::get::<FunctionType>(f).expect("expected function type");
  let iter = begin_type_pack::begin(ftv.arg_types());
  let end_iter = end_type_pack::end(ftv.arg_types());

  assert!(iter == end_iter);
  let tail = iter.tail().expect("expected variadic argument tail");
  assert!(get_type_pack::get::<VariadicTypePack>(tail).is_some());
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_require_failed_module() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
return unfortunately()
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert!(!a_result.errors.is_empty(), "{:?}", a_result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local ModuleA = require(game.A)
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module_a = fixture.base.require_type_string("ModuleA");
  assert_eq!("*error-type*", to_string_type_id(module_a));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_require_module_that_does_not_export() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::illegal_require::IllegalRequire,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();

  fixture
    .base
    .file_resolver
    .source
    .insert("game/Workspace/A", "");
  fixture.base.file_resolver.source.insert(
    "game/Workspace/B",
    r#"
        local Hooty = require(script.Parent.A)
    "#,
  );

  let _ = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Workspace/A"), None);
  let _ = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Workspace/B"), None);

  let a_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Workspace/A"));
  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Workspace/B"));

  assert!(a_module.errors.is_empty(), "{:?}", a_module.errors);
  assert_eq!(1, b_module.errors.len(), "{:?}", b_module.errors);
  assert!(
    type_error_data_ref::<IllegalRequire>(&b_module.errors[0]).is_some(),
    "Should be IllegalRequire: {:?}",
    b_module.errors[0]
  );

  let hooty_type = fixture
    .base
    .require_type_module_ptr_string(&b_module, "Hooty");
  assert_eq!("*error-type*", to_string_type_id(hooty_type));
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_require_types() {
  use ulua_analysis::{
    functions::{get_type, to_string_to_string::to_string_type_id},
    records::table_type::TableType,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "workspace/A",
    r#"
        export type Point = {x: number, y: number}

        return {}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "workspace/B",
    r#"
        local Hooty = require(workspace.A)

        local h: Hooty.Point
    "#,
  );

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("workspace/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("workspace/B"));
  let h_type = fixture.base.require_type_module_ptr_string(&b_module, "h");
  assert!(
    get_type::get::<TableType>(h_type).is_some(),
    "Expected table but got {}",
    to_string_type_id(h_type)
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_returned_module_unassigned_local_stays_nil() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        local a = nil
        local b = function() return 1 end
        b = nil
        return {a = a, b = b}
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "nil",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "a"))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "b"))
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_scrub_unsealed_tables() {
  use ulua_analysis::records::{
    cannot_extend_table::CannotExtendTable, code_too_complex::CodeTooComplex,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    internal_error::InternalError,
  };
  use ulua_common::{fflag, fint};
  use ulua_unit_test::{
    functions::has_error::has_error,
    records::builtins_fixture::BuiltinsFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _constraint_limit = ScopedFastInt::new(&fint::LuauSolverConstraintLimit, 5);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        type Array<T> = {T}
        type Hello = Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<number>>>>>>>>>>
        local X = {}
        X.foo = 42
        X.bar = ""
        return X
    "#,
  );

  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        local x = require(game.A)
        x.lmao = 42
        return {}
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
  assert!(
    has_error::<ConstraintSolvingIncompleteError>(&result),
    "{:?}",
    result.errors
  );
  assert!(has_error::<InternalError>(&result), "{:?}", result.errors);
  assert!(
    has_error::<CannotExtendTable>(&result),
    "{:?}",
    result.errors
  );
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_spooky_blocked_type_laundered_by_bound_type() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        local Cache = {}

        Cache.settings = {}

        Cache.data = {}

        function Cache.should_cache(url)
            url = url:split("?")[1]

            for key, _ in pairs(Cache.settings) do
                if url:match('') then
                    return key
                end
            end

            return ""
        end

        function Cache.is_cached(url, req_id)
            -- check local server cache first

            local setting_key = Cache.should_cache(url)
            local settings = Cache.settings[setting_key]

            if not setting_key then
                return false
            end

            if Cache.data[req_id] ~= nil then
                return true
            end

            if Cache.settings[setting_key].cache_globally then
                return false
            else
                return true
            end
        end

        function Cache.get_expire(url)
            local setting_key = Cache.should_cache(url)
            return Cache.settings[setting_key].expires or math.huge
        end

        return Cache
    "#,
  );

  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local _ = require(game.A);
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_type_error_of_unknown_qualified_type() {
  use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
  use ulua_ast::records::{location::Location, position::Position};
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::fixture::Fixture,
  };

  let (_fixture, result) = fx_check!(
    r#"
        local p: SomeModule.DoesNotExist
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 17
      },
      end: Position {
        line: 1,
        column: 40
      }
    },
    result.errors[0].location
  );
  let error =
    type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
  assert_eq!("SomeModule.DoesNotExist", error.name());
  assert_eq!(Context::Type, error.context());
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_untitled_segfault_number_13() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
      "game/A",
      r#"
        -- minimized from roblox-requests/http/src/response.lua
        local Response = {}
        Response.__index = Response
        function Response.new(content_type)
            -- creates response object from original request and roblox http response
            local self = setmetatable({}, Response)
            self.content_type = content_type
            return self
        end

        function Response:xml(ignore_content_type)
            if ignore_content_type or self.content_type:find("+xml") or self.content_type:find("/xml") then
            else
            end
        end

        ---------------

        return Response
    "#,
  );

  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local _ = require(game.A);
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/TypeInfer.modules.test.cpp`.
#[test]
fn type_infer_modules_warn_if_you_try_to_require_a_non_modulescript() {
  use ulua_analysis::{
    enums::type_file_resolver::Type as SourceCodeType, records::illegal_require::IllegalRequire,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert("Modules/A", "");
  fixture
    .base
    .file_resolver
    .source_types
    .borrow_mut()
    .insert(ModuleName::from("Modules/A"), SourceCodeType::Script);
  fixture.base.file_resolver.source.insert(
    "Modules/B",
    r#"
        local M = require(script.Parent.A)
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<IllegalRequire>(&result.errors[0]).is_some());
}

// 缺口（未移植，对照 `tests/TypeInfer.modules.test.cpp`，共 3 例；tw-10 复核）：
// - cli_194463_modify_bounds_of_visited_generic_regression（:982）——Rust 端 check
//   "game/Main" 时在 ulua-analysis internal_error_reporter_ice_error.rs 直接 ICE
//   panic（Box<dyn Any> unwind），faithful 断言（0 错误）不可达。早前记录的
//   CannotAssignToExport 分歧已恶化为崩溃，均指向求解器 visited-generic bounds
//   未同步（`Container<K,V> = {[K]: V} & typeof(Container)` 交叉别名递归边界）。
// - exported_module_initializer_type_packs（:1057）/
//   exported_module_initializer_type_packs_multi（:1095）——cpp 依赖 FFlag
//   `LuauExportTypecheckTypepacks`，本移植未定义该旗标；tw-10 去旗标探针实测：
//   多导出绑定解包不完整，exports 串中直绑项被置为 *error-type*
//   （`read constDirect: *error-type*` / `read const3: *error-type*`，别名项正常）。
// 已转正（tw-10）：exported_module_annotation_uses_binding_type（:1417）、
// exported_module_annotation_preferred_over_initializer（:1459）、
// exported_module_binding_is_readonly（:1491）、
// exported_module_annotation_mismatch_errors（:1519）——cpp 依赖 FFlag
// `LuauExportAnnotationBinding`，Rust 产品端虽未定义该旗标但行为已无条件同步，
// 去旗标探针 4 例全通过，遂按 cpp 断言正式移植。

// Source: `tests/TypeInfer.modules.test.cpp:1417-1457`
#[test]
fn type_infer_modules_exported_module_annotation_uses_binding_type() {
  use ulua_analysis::functions::{first::first, to_string_to_string::to_string_type_id};
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export local x: number = 5
        export local y: string = "hello"
        export local z: {name: string} = {name = "test"}
    "#,
  );
  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local x = A.x
        local y = A.y
        local z = A.z
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "number",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "x"))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "y"))
  );
  assert_eq!(
    "{ name: string }",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "z"))
  );

  let a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  let exports = first(a.return_type, true).expect("expected exports");
  assert_eq!(
    "{ read x: number, read y: string, read z: { name: string } }",
    to_string_type_id(exports)
  );
}

// Source: `tests/TypeInfer.modules.test.cpp:1459-1489`
#[test]
fn type_infer_modules_exported_module_annotation_preferred_over_initializer() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        type Callback = (number) -> string
        export local handler: Callback = function(n) return tostring(n) end
    "#,
  );
  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)

        local h = A.handler
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(
    "(number) -> string",
    to_string_type_id(fixture.base.require_type_module_ptr_string(&b, "h"))
  );
}

// Source: `tests/TypeInfer.modules.test.cpp:1491-1517`
#[test]
fn type_infer_modules_exported_module_binding_is_readonly() {
  use ulua_analysis::records::property_access_violation::PropertyAccessViolation;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export local Value = 42
    "#,
  );
  fixture.base.file_resolver.source.insert(
    "game/B",
    r#"
        --!strict
        local A = require(game.A)
        A.Value = 13
    "#,
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(1, b_result.errors.len(), "{:?}", b_result.errors);
  assert!(type_error_data_ref::<PropertyAccessViolation>(&b_result.errors[0]).is_some());
}

// Source: `tests/TypeInfer.modules.test.cpp:1519-1532`
#[test]
fn type_infer_modules_exported_module_annotation_mismatch_errors() {
  use ulua_analysis::records::type_mismatch::TypeMismatch;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    "game/A",
    r#"
        --!strict
        export local x: number = "RUH ROH"
    "#,
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(type_error_data_ref::<TypeMismatch>(&result.errors[0]).is_some());
}
