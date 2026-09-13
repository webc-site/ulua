extern crate alloc;

mod type_infer_modules_bound_free_table_export_is_ok {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_bound_free_table_export_is_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
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
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_check_imported_module_names {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_check_imported_module_names() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
return function(...) end
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local l0 = require(game.A)
return l0
    "#,
      ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local l0 = require(game.B)
if true then
    local l1 = require(game.A)
end
return l0
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = fixture.base.get_main_module(false);
    assert!(!module.is_null());

    let scopes = unsafe { &(*module).scopes };
    assert_eq!(4, scopes.len());

    let root_scope = &scopes[0].1;
    let block_scope = &scopes[3].1;
    assert_eq!(
      Some(&String::from("game/B")),
      root_scope.imported_modules.get(&String::from("l0"))
    );
    assert_eq!(
      Some(&String::from("game/A")),
      block_scope.imported_modules.get(&String::from("l1"))
    );
  }
}

mod type_infer_modules_constrained_anyification_clone_immutable_types {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_constrained_anyification_clone_immutable_types() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
return function(...) end
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local l0 = require(game.A)
return l0
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_cross_module_function_mutation {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_cross_module_function_mutation() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
function test2(a: number, b: string)
    return 1
end

return test2
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
function wrapper<A...>(f: (A...) -> number, ...: A...)
end

local test2 = require(game.A)

return wrapper(test2, 1, "")
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_cross_module_table_freeze {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_cross_module_table_freeze() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        return {
            a = 1,
        }
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        return table.freeze(require(game.A))
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let a_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert_eq!(
      "{ a: number }",
      to_string_type_pack_id(a_module.return_type)
    );

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "{ read a: number }"
    } else {
      "{ a: number }"
    };
    assert_eq!(expected, to_string_type_pack_id(b_module.return_type));
  }
}

mod type_infer_modules_custom_require_global {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_custom_require_global() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
require = function(a) end

local crash = require(game.A)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_cycles_dont_make_everything_any {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_cycles_dont_make_everything_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
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
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
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
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!("module", to_string_type_pack_id(b_module.return_type));
  }
}

mod type_infer_modules_dcr_require_basic {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_dcr_require_basic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        return {
            a = 1,
        }
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local b = A.a
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let b_type = fixture
      .base
      .require_type_module_ptr_string(&b_module, &String::from("b"));
    assert_eq!("number", to_string_type_id(b_type));
  }
}

mod type_infer_modules_do_not_modify_imported_types {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_do_not_modify_imported_types() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type Type = { unrelated: boolean }
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local types = require(game.A)
type Type = types.Type
local x: Type = {}
function x:Destroy(): () end
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_do_not_modify_imported_types_2 {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_do_not_modify_imported_types_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type Type = { x: { a: number } }
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local types = require(game.A)
type Type = types.Type
local x: Type = { x = { a = 2 } }
type Rename = typeof(x.x)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_do_not_modify_imported_types_3 {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_do_not_modify_imported_types_3() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
local y = setmetatable({}, {})
export type Type = { x: typeof(y) }
return { x = y }
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local types = require(game.A)
type Type = types.Type
local x: Type = types
type Rename = typeof(x.x)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_do_not_modify_imported_types_4 {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_do_not_modify_imported_types_4() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type Array<T> = {T}
local arrayops = {}
function arrayops.foo(x: Array<any>) end
return arrayops
    "#,
      ),
    );
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_do_not_modify_imported_types_5 {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_do_not_modify_imported_types_5() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type Type = {x: number, y: number}
local arrayops = {}
function arrayops.foo(x: Type) end
return arrayops
    "#,
      ),
    );
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_ensure_free_variables_are_generialized_across_function_boundaries {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_ensure_free_variables_are_generialized_across_function_boundaries() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
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
      ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local ReactShallowRenderer = require(game.A);
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_ensure_scope_is_nullptr_after_shallow_copy {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_ensure_scope_is_nullptr_after_shallow_copy() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
-- Roughly taken from ReactTypes.lua
type CoreBinding<T> = {}
type BindingMap = {}
export type Binding<T> = CoreBinding<T> & BindingMap

return {}
    "#,
      ),
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local Types = require(game.A)
type Binding<T> = Types.Binding<T>
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_export_class {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_export_class() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        export class Point
            public x: number
            public y: number

            function __tostring(self)
                return `Point x={self.x} y={self.y}`
            end
        end
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local A = require(game.A)

        local a: A.Point = A.Point.new { x=2, y=3 }

        local x, y = a.x, a.y
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("game/B", &String::from("x"))
      )
    );
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_name_string("game/B", &String::from("y"))
      )
    );
  }
}

mod type_infer_modules_exported_module_basic {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_exported_module_basic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        export local version = "1.0.0"
        export const name = "test module"
        export local count = 41

        count += 1
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local version = A.version
        local name = A.name
        local count = A.count
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("count"))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("version"))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("name"))
      )
    );
  }
}

mod type_infer_modules_exported_module_function {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_exported_module_function() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
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
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local add = A.add
        local greet = A.greet
        local noop = A.noop
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "(number, number) -> number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("add"))
      )
    );
    assert_eq!(
      "(string) -> string",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("greet"))
      )
    );
    assert_eq!(
      "(...any) -> ()",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("noop"))
      )
    );
  }
}

mod type_infer_modules_exported_module_mutual_recursive_functions {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_exported_module_mutual_recursive_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
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
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "(...any) -> number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("a"))
      )
    );
    assert_eq!(
      "(...any) -> number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("b"))
      )
    );
  }
}

mod type_infer_modules_exported_module_unassigned_local_stays_nil {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_exported_module_unassigned_local_stays_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        export local a
        export local b = function() return 1 end
        b = nil
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("a"))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("b"))
      )
    );
  }
}

mod type_infer_modules_exported_multret {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_exported_multret() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        local function huh()
            return 42, "huh", false
        end

        export local a, b, c = huh()
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
        local c = A.c
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("a"))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("b"))
      )
    );
    assert_eq!(
      "boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("c"))
      )
    );
  }
}

mod type_infer_modules_exported_partial_multret {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_exported_partial_multret() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        local function huh()
            return "huh", false
        end

        export local a, b, c = 42, huh()
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
        local c = A.c
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("a"))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("b"))
      )
    );
    assert_eq!(
      "boolean",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("c"))
      )
    );
  }
}

mod type_infer_modules_fuzz_anyify_variadic_return_must_follow {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_fuzz_anyify_variadic_return_must_follow() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
return unpack(l0[_])
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod type_infer_modules_general_require_call_expression {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_general_require_call_expression() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
--!strict
return { def = 4 }
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
--!strict
local tbl = { abc = require(game.A) }
local a : string = ""
a = tbl.abc.def
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_modules_general_require_type_mismatch {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_general_require_type_mismatch() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
return { def = 4 }
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local tbl: string = require(game.A)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Expected this to be 'string', but got '{ def: number }'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_modules_internal_type_errors_are_only_reported_once {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_internal_type_errors_are_only_reported_once() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_d::to_string_type_pack_id,
      records::internal_error::InternalError,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
return function(): { X: _luau_blocked_type, Y: _luau_blocked_type } return nil :: any end
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<InternalError>(&result.errors[0]).is_some());

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert_eq!(
      "(...any) -> { X: *error-type*, Y: *error-type* }",
      to_string_type_pack_id(module.return_type)
    );
  }
}

mod type_infer_modules_internal_types_are_scrubbed_from_module {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_internal_types_are_scrubbed_from_module() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_d::to_string_type_pack_id,
      records::{
        constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
        internal_error::InternalError,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
return function(): _luau_blocked_type return nil :: any end
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<ConstraintSolvingIncompleteError>(&result.errors[0]).is_some());
    assert!(type_error_data_ref::<InternalError>(&result.errors[1]).is_some());

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert_eq!(
      "(...any) -> *error-type*",
      to_string_type_pack_id(module.return_type)
    );
  }
}

mod type_infer_modules_invalid_alias_should_export_as_error_type {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_invalid_alias_should_export_as_error_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::recursive_restraint_violation::RecursiveRestraintViolation,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        export type bad<T> = {bad<{T}>}
        return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local a_mod = require(game.A)
        local f: a_mod.bad<number>
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0]).is_some());

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let f_type = fixture
      .base
      .require_type_module_ptr_string(&b, &String::from("f"));
    assert_eq!("bad<number>", to_string_type_id(f_type));
  }
}

mod type_infer_modules_invalid_local_alias_shouldnt_shadow_imported_type {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_invalid_local_alias_shouldnt_shadow_imported_type() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::recursive_restraint_violation::RecursiveRestraintViolation,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        export type bad<T> = {T}
        return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local a_mod = require(game.A)
        type bad<T> = {bad<{T}>}
        type fine<T> = a_mod.bad<T>
        local f: fine<number>
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0]).is_some());

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let f_type = fixture
      .base
      .require_type_module_ptr_string(&b, &String::from("f"));
    assert_eq!("fine<number>", to_string_type_id(f_type));
  }
}

mod type_infer_modules_leaky_generics {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_leaky_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_modules_module_type_conflict {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_module_type_conflict() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type T = { x: number }
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
export type T = { x: string }
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/C"),
      String::from(
        r#"
local A = require(game.A)
local B = require(game.B)
local a: A.T = { x = 2 }
local b: B.T = a
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/C"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Expected this to be 'T' from 'game/B', but got 'T' from 'game/A'; \naccessing `x` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`"
    } else {
      "Expected this to be exactly 'T' from 'game/B', but got 'T' from 'game/A'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'string', but got 'number'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_modules_module_type_conflict_instantiated {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_module_type_conflict_instantiated() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
export type Wrap<T> = { x: T }
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
local A = require(game.A)
export type T = A.Wrap<number>
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/C"),
      String::from(
        r#"
local A = require(game.A)
export type T = A.Wrap<string>
return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/D"),
      String::from(
        r#"
local A = require(game.B)
local B = require(game.C)
local a: A.T = { x = 2 }
local b: B.T = a
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/D"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Expected this to be 'T' from 'game/C', but got 'T' from 'game/B'; \naccessing `x` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`"
    } else {
      "Expected this to be exactly 'T' from 'game/C', but got 'T' from 'game/B'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'string', but got 'number'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_modules_non_exported_class {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_non_exported_class() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        class Point
            public x: number
            public y: number

            function __tostring(self)
                return `Point x={self.x} y={self.y}`
            end
        end

        return {Point=Point}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local A = require(game.A)

        local a: A.Point = A.Point.new { x=2, y=3 }
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = type_error_data_ref::<UnknownSymbol>(&result.errors[0])
      .expect("expected UnknownSymbol error");
    assert_eq!("A.Point", err.name());
    assert_eq!(Context::Type, err.context());
  }
}

mod type_infer_modules_require {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_require() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        local function hooty(x: number): string
            return "Hi there!"
        end

        return {hooty=hooty}
    "#,
      ),
    );

    let source_b = if !FFlag::DebugLuauForceOldSolver.get() {
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

    fixture
      .base
      .file_resolver
      .source
      .insert(String::from("game/B"), String::from(source_b));

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));

    let i_type = fixture
      .base
      .require_type_module_ptr_string(&b_module, &String::from("i"));
    assert_eq!("string", to_string_type_id(i_type));

    let h_type = fixture
      .base
      .require_type_module_ptr_string(&b_module, &String::from("h"));
    assert_eq!("number", to_string_type_id(h_type));
  }
}

mod type_infer_modules_require_a_variadic_function {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_require_a_variadic_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        begin_type_pack, end_type_pack, follow_type::follow, get_type_id::get_type_id,
        get_type_pack::get_type_pack_id,
      },
      records::{function_type::FunctionType, variadic_type_pack::VariadicTypePack},
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        local T = {}
        function T.f(...) end
        return T
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local A = require(game.A)
        local f = A.f
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let f = follow(
      fixture
        .base
        .require_type_module_ptr_string(&b_module, &String::from("f")),
    );

    let ftv = get_type_id::<FunctionType>(f).expect("expected function type");
    let iter = begin_type_pack::begin(ftv.arg_types());
    let end_iter = end_type_pack::end(ftv.arg_types());

    assert!(iter.operator_eq(&end_iter));
    let tail = iter.tail().expect("expected variadic argument tail");
    assert!(get_type_pack_id::<VariadicTypePack>(tail).is_some());
  }
}

mod type_infer_modules_require_failed_module {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_require_failed_module() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
return unfortunately()
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!a_result.errors.is_empty(), "{:?}", a_result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
local ModuleA = require(game.A)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_a = fixture.base.require_type_string(&String::from("ModuleA"));
    assert_eq!("*error-type*", to_string_type_id(module_a));
  }
}

mod type_infer_modules_require_module_that_does_not_export {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_require_module_that_does_not_export() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::illegal_require::IllegalRequire,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();

    fixture
      .base
      .file_resolver
      .source
      .insert(String::from("game/Workspace/A"), String::from(""));
    fixture.base.file_resolver.source.insert(
      String::from("game/Workspace/B"),
      String::from(
        r#"
        local Hooty = require(script.Parent.A)
    "#,
      ),
    );

    let _ = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Workspace/A"), None);
    let _ = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Workspace/B"), None);

    let a_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Workspace/A"));
    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Workspace/B"));

    assert!(a_module.errors.is_empty(), "{:?}", a_module.errors);
    assert_eq!(1, b_module.errors.len(), "{:?}", b_module.errors);
    assert!(
      type_error_data_ref::<IllegalRequire>(&b_module.errors[0]).is_some(),
      "Should be IllegalRequire: {:?}",
      b_module.errors[0]
    );

    let hooty_type = fixture
      .base
      .require_type_module_ptr_string(&b_module, &String::from("Hooty"));
    assert_eq!("*error-type*", to_string_type_id(hooty_type));
  }
}

mod type_infer_modules_require_types {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_require_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_id::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("workspace/A"),
      String::from(
        r#"
        export type Point = {x: number, y: number}

        return {}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("workspace/B"),
      String::from(
        r#"
        local Hooty = require(workspace.A)

        local h: Hooty.Point
    "#,
      ),
    );

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("workspace/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("workspace/B"));
    let h_type = fixture
      .base
      .require_type_module_ptr_string(&b_module, &String::from("h"));
    assert!(
      get_type_id::<TableType>(h_type).is_some(),
      "Expected table but got {}",
      to_string_type_id(h_type)
    );
  }
}

mod type_infer_modules_returned_module_unassigned_local_stays_nil {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_returned_module_unassigned_local_stays_nil() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!strict
        local a = nil
        local b = function() return 1 end
        b = nil
        return {a = a, b = b}
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!strict
        local A = require(game.A)

        local a = A.a
        local b = A.b
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("a"))
      )
    );
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_module_ptr_string(&b, &String::from("b"))
      )
    );
  }
}

mod type_infer_modules_scrub_unsealed_tables {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_scrub_unsealed_tables() {
    use alloc::string::String;

    use ulua_analysis::records::{
      cannot_extend_table::CannotExtendTable, code_too_complex::CodeTooComplex,
      constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
      internal_error::InternalError,
    };
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      functions::has_error::has_error,
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _constraint_limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 5);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        type Array<T> = {T}
        type Hello = Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<number>>>>>>>>>>
        local X = {}
        X.foo = 42
        X.bar = ""
        return X
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local x = require(game.A)
        x.lmao = 42
        return {}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
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
}

mod type_infer_modules_spooky_blocked_type_laundered_by_bound_type {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_spooky_blocked_type_laundered_by_bound_type() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
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
      ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = require(game.A);
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_type_error_of_unknown_qualified_type {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_type_error_of_unknown_qualified_type() {
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
        local p: SomeModule.DoesNotExist
    "#,
      ),
      None,
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
}

mod type_infer_modules_untitled_segfault_number_13 {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_untitled_segfault_number_13() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
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
        ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = require(game.A);
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_modules_warn_if_you_try_to_require_a_non_modulescript {
  //! Ported from `tests/TypeInfer.modules.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_modules_warn_if_you_try_to_require_a_non_modulescript() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::type_file_resolver::Type as SourceCodeType, records::illegal_require::IllegalRequire,
    };
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();

    fixture
      .base
      .file_resolver
      .source
      .insert(String::from("Modules/A"), String::new());
    fixture
      .base
      .file_resolver
      .source_types
      .insert(String::from("Modules/A"), SourceCodeType::Script);
    fixture.base.file_resolver.source.insert(
      String::from("Modules/B"),
      String::from(
        r#"
        local M = require(script.Parent.A)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<IllegalRequire>(&result.errors[0]).is_some());
  }
}
