extern crate alloc;
use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_analysis::{
  enums::table_state::TableState,
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_value, freeze::freeze,
    get_global_binding::get_global_binding, get_mutable_type, unfreeze::unfreeze,
  },
  records::{
    binding::Binding, property_type::Property, scope::Scope, table_type::TableType,
    type_fun::TypeFun, type_level::TypeLevel,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::{location::Location, position::Position};
use ulua_common::fflag;
use ulua_config::enums::code::Code;
use ulua_unit_test::{
  functions::check_deprecated_warning::check_deprecated_warning,
  records::{builtins_fixture::BuiltinsFixture, fixture::Fixture},
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_break_from_infinite_loop_makes_statement_reachable() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local bar = ...

repeat
    if bar then
        break
    end

    return 2
until true

return 1
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_builtin_global_write() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.lint(
    r#"
math = {}

function assert(x)
end

assert(5)
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Built-in global 'math' is overwritten here; consider using a local or changing the name",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Built-in global 'assert' is overwritten here; consider using a local or changing the name",
    result.warnings[1].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_clean_code() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function fib(n)
    return n < 2 and 1 or fib(n-1) + fib(n-2)
end

"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_comparison_precedence() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local a, b = ...

local _ = not a == b
local _ = not a ~= b
local _ = not a <= b
local _ = a <= b == 0
local _ = a <= b <= 0

local _ = not a == not b -- weird but ok

-- silence tests for all of the above
local _ = not (a == b)
local _ = (not a) == b
local _ = not (a ~= b)
local _ = (not a) ~= b
local _ = not (a <= b)
local _ = (not a) <= b
local _ = (a <= b) == 0
local _ = a <= (b == 0)
"#,
    None,
  );

  assert_eq!(5, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "not X == Y is equivalent to (not X) == Y; consider using X ~= Y, or add parentheses to silence",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "not X ~= Y is equivalent to (not X) ~= Y; consider using X == Y, or add parentheses to silence",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "not X <= Y is equivalent to (not X) <= Y; add parentheses to silence",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "X <= Y == Z is equivalent to (X <= Y) == Z; add parentheses to silence",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    "X <= Y <= Z is equivalent to (X <= Y) <= Z; did you mean X <= Y and Y <= Z?",
    result.warnings[4].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_confusing_indentation() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
print(math.max(1,
2))
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Statement spans multiple lines; use indentation to silence",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_dead_locals_used() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!nolint LocalShadow
do
    local x
    for x in pairs({}) do
        print(x)
    end
    print(x) -- x is not initialized
end

do
    local a, b, c = 1, 2
    print(a, b, c) -- c is not initialized
end

do
    local a, b, c = table.unpack({})
    print(a, b, c) -- no warning as we don't know anything about c
end
    "#,
    None,
  );

  assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'x' defined at line 4 is never initialized or assigned; initialize with 'nil' to silence",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Assigning 2 values to 3 variables initializes extra variables with nil; add 'nil' to value list to silence",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "Variable 'c' defined at line 12 is never initialized or assigned; initialize with 'nil' to silence",
    result.warnings[2].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_api_fenv() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.lint(
    r#"
local f, g, h = ...

getfenv(1)
getfenv(f :: () -> ())
getfenv(g :: number)
getfenv(h :: any)

setfenv(1, {})
setfenv(f :: () -> (), {})
setfenv(g :: number, {})
setfenv(h :: any, {})
"#,
    None,
  );

  assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Function 'getfenv' is deprecated; consider using 'debug.info' instead",
    result.warnings[0].text.as_str()
  );
  assert_eq!(4, result.warnings[0].location.begin.line + 1);
  assert_eq!(
    "Function 'getfenv' is deprecated; consider using 'debug.info' instead",
    result.warnings[1].text.as_str()
  );
  assert_eq!(6, result.warnings[1].location.begin.line + 1);
  assert_eq!(
    "Function 'setfenv' is deprecated",
    result.warnings[2].text.as_str()
  );
  assert_eq!(9, result.warnings[2].location.begin.line + 1);
  assert_eq!(
    "Function 'setfenv' is deprecated",
    result.warnings[3].text.as_str()
  );
  assert_eq!(11, result.warnings[3].location.begin.line + 1);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_api_typed() {
  use alloc::collections::BTreeMap;

  use ulua_analysis::{functions::persist_type::persist, records::extern_type::ExternType};

  let mut fixture = BuiltinsFixture::default();

  {
    let frontend = fixture.get_frontend();
    unfreeze(frontend.globals.global_types_mut());

    let builtins = frontend.builtin_types_ref();
    let string_type = builtins.string_type;
    let number_type = builtins.number_type;
    let any_type = builtins.any_type;

    let mut instance_props = BTreeMap::new();
    instance_props.insert(String::from("Name"), Property::rw_type_id(string_type));

    let mut data_cost = Property::rw_type_id(number_type);
    data_cost.deprecated = true;
    instance_props.insert(String::from("DataCost"), data_cost);

    let mut wait = Property::rw_type_id(any_type);
    wait.deprecated = true;
    instance_props.insert(String::from("Wait"), wait);

    let instance_type = frontend.globals.global_types_mut().add_type(ExternType {
      name: String::from("Instance"),
      props: instance_props,
      parent: None,
      metatable: None,
      tags: Vec::new(),
      user_data: None,
      definition_module_name: ModuleName::from("Test"),
      definition_location: None,
      indexer: None,
      relation: None,
    });

    persist(instance_type);

    let global_scope = frontend.globals.global_scope();
    let scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
    unsafe {
      (*scope_ptr).exported_type_bindings.insert(
        String::from("Instance"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          Vec::new(),
          instance_type,
          None,
        ),
      );
    }

    let color_type = frontend.globals.global_types_mut().add_type(
      TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
        &BTreeMap::new(),
        None,
        TypeLevel::default(),
        scope_ptr,
        TableState::Sealed,
      ),
    );

    if let Some(color_table) = get_mutable_type::get_mutable::<TableType>(color_type) {
      let mut to_hsv = Property::rw_type_id(any_type);
      to_hsv.deprecated = true;
      to_hsv.deprecated_suggestion = String::from("Color3:ToHSV");
      color_table.props.insert(String::from("toHSV"), to_hsv);
    }

    add_global_binding_value(
      &mut frontend.globals,
      "Color3",
      Binding {
        type_id: color_type,
        location: Location::default(),
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
      },
    );

    let table_type = get_global_binding(&mut frontend.globals, "table");
    if let Some(table) = get_mutable_type::get_mutable::<TableType>(table_type) {
      if let Some(prop) = table.props.get_mut("foreach") {
        prop.deprecated = true;
      }
      if let Some(prop) = table.props.get_mut("getn") {
        prop.deprecated = true;
        prop.deprecated_suggestion = String::from("#");
      }
    }

    freeze(frontend.globals.global_types_mut());
  }

  let result = fixture.base.lint(

          r#"
return function (i: Instance)
    i:Wait(1.0)
    print(i.Name)
    print(Color3.toHSV())
    print(Color3.doesntexist, i.doesntexist) -- type error, but this verifies we correctly handle non-existent members
    print(table.getn({}))
    table.foreach({}, function() end)
    print(table.nogetn()) -- verify that we correctly handle non-existent members
    return i.DataCost
end
"#
,
      None,
  );

  assert_eq!(5, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Member 'Instance.Wait' is deprecated",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Member 'toHSV' is deprecated, use 'Color3:ToHSV' instead",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "Member 'table.getn' is deprecated, use '#' instead",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "Member 'table.foreach' is deprecated",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    "Member 'Instance.DataCost' is deprecated",
    result.warnings[4].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_api_untyped() {
  let mut fixture = BuiltinsFixture::default();

  {
    let frontend = fixture.get_frontend();
    let table_type = get_global_binding(&mut frontend.globals, "table");
    if let Some(table) = get_mutable_type::get_mutable::<TableType>(table_type) {
      if let Some(prop) = table.props.get_mut("foreach") {
        prop.deprecated = true;
      }
      if let Some(prop) = table.props.get_mut("getn") {
        prop.deprecated = true;
        prop.deprecated_suggestion = String::from("#");
      }
    }
  }

  let result = fixture.base.lint(
    r#"
-- TODO
return function ()
    print(table.getn({}))
    table.foreach({}, function() end)
    print(table.nogetn()) -- verify that we correctly handle non-existent members
end
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Member 'table.getn' is deprecated, use '#' instead",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Member 'table.foreach' is deprecated",
    result.warnings[1].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_attribute() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  {
    let result = fixture.lint(
      r#"
@deprecated
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
local function testfun(x:number):number
    return x + 1
end

if math.random(2) == 2 then
    testfun(1)
end
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(7, 4),
      Position::new(7, 11),
      "Function 'testfun' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
local function testfun(x:number)
    return x + 1
end

g(testfun)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 2),
      Position::new(6, 9),
      "Function 'testfun' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
local function testfun(x):number
    if x == 1 then
        return x
    else
        return 1 + testfun(x - 1)
    end
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(10, 0),
      Position::new(10, 7),
      "Function 'testfun' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
function flipFlop()
    local state = false

    @deprecated
    local function invert()
        state = !state
        return state
    end

    return invert
end

f = flipFlop()
assert(f() == true)
"#,
      None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(10, 11),
      Position::new(10, 17),
      "Function 'invert' is deprecated",
    );
    check_deprecated_warning(
      &result.warnings[1],
      Position::new(14, 7),
      Position::new(14, 8),
      "Function 'f' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
function flipFlop()
    local state = false

    local function invert()
        state = !state
        return state
    end

    return invert
end

f = flipFlop()
assert(f() == true)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(13, 4),
      Position::new(13, 12),
      "Function 'flipFlop' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
local function doTheThing()
    print("doing")
end

doTheThing()

local function shadow()
    local function doTheThing()
        print("doing!")
    end

    doTheThing()
end

shadow()
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 10),
      "Function 'doTheThing' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
function fibonacci(n)
    if n == 0 then
        return 0
    elseif n == 1 then
        return 1
    else
        return fibonacci(n - 1) + fibonacci(n - 2)
    end
end

fibonacci(5)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(12, 0),
      Position::new(12, 9),
      "Function 'fibonacci' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
@deprecated
function odd(x)
    if x == 0 then
        return false
    else
        return even(x - 1)
    end
end

@deprecated
function even(x)
    if x == 0 then
        return true
    else
        return odd(x - 1)
    end
end

assert(odd(1) == true)
assert(even(0) == true)
"#,
      None,
    );

    assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 15),
      Position::new(6, 19),
      "Function 'even' is deprecated",
    );
    check_deprecated_warning(
      &result.warnings[1],
      Position::new(15, 15),
      Position::new(15, 18),
      "Function 'odd' is deprecated",
    );
    check_deprecated_warning(
      &result.warnings[2],
      Position::new(19, 7),
      Position::new(19, 10),
      "Function 'odd' is deprecated",
    );
    check_deprecated_warning(
      &result.warnings[3],
      Position::new(20, 7),
      Position::new(20, 11),
      "Function 'even' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
Account = { balance=0 }

@deprecated
function Account:deposit(v)
    self.balance = self.balance + v
end

Account:deposit(200.00)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(8, 0),
      Position::new(8, 15),
      "Member 'Account.deposit' is deprecated",
    );
  }

  {
    let result = fixture.lint(
      r#"
Account = { balance=0 }

function getAccount()
    return Account
end

@deprecated
function Account:deposit (v)
    self.balance = self.balance + v
end

(getAccount()):deposit(200.00)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(12, 0),
      Position::new(12, 22),
      "Member 'deposit' is deprecated",
    );
  }
}

#[test]
fn linter_deprecated_attribute_function_declaration() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  fixture.load_definition(
    r#"
@deprecated declare function bar(x: number): string
"#,
    false,
  );

  let result = fixture.lint(
    r#"
bar(2)
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  check_deprecated_warning(
    &result.warnings[0],
    Position::new(1, 0),
    Position::new(1, 3),
    "Function 'bar' is deprecated",
  );
}

#[test]
fn linter_deprecated_attribute_method_declaration() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  fixture.load_definition(
    r#"
declare extern type Foo with
   @deprecated
   function bar(self, value: number) : number
end

declare Foo: {
   new: () -> Foo
}
"#,
    false,
  );

  let result = fixture.lint(
    r#"
local foo = Foo.new()
print(foo:bar(2.0))
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  check_deprecated_warning(
    &result.warnings[0],
    Position::new(2, 6),
    Position::new(2, 13),
    "Member 'bar' is deprecated",
  );
}

#[test]
fn linter_deprecated_attribute_table_declaration() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  fixture.load_definition(
    r#"
declare Hooty : {
    tooty : @deprecated @checked (number) -> number
}
"#,
    false,
  );

  let result = fixture.lint(
    r#"
print(Hooty:tooty(2.0))
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  check_deprecated_warning(
    &result.warnings[0],
    Position::new(1, 6),
    Position::new(1, 17),
    "Member 'Hooty.tooty' is deprecated",
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_attribute_with_params() {
  let mut fixture = Fixture::fixture_bool(false);

  {
    let result = fixture.lint(
      r#"
@[deprecated{ use = "prodfun", reason = "Too old." }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated, use 'prodfun' instead. Too old.",
    );
  }

  {
    let result = fixture.lint(
      r#"
@[deprecated{ use = "prodfun", reason = "Too old." }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated, use 'prodfun' instead. Too old.",
    );
  }

  {
    let result = fixture.lint(
      r#"
@[deprecated{ use = "prodfun" }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated, use 'prodfun' instead",
    );
  }

  {
    let result = fixture.lint(
      r#"
@[deprecated{ use = "prodfun" }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated, use 'prodfun' instead",
    );
  }

  {
    let result = fixture.lint(
      r#"
@[deprecated{ reason = "Too old." }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated. Too old.",
    );
  }

  {
    let result = fixture.lint(
      r#"
@[deprecated{ reason = "Too old." }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(6, 0),
      Position::new(6, 7),
      "Function 'testfun' is deprecated. Too old.",
    );
  }

  {
    let result = fixture.lint(
      r#"
Account = { balance=0 }

@[deprecated{use = 'credit', reason = 'It sounds cool'}]
function Account:deposit(v)
    self.balance = self.balance + v
end

Account:deposit(200.00)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(8, 0),
      Position::new(8, 15),
      "Member 'Account.deposit' is deprecated, use 'credit' instead. It sounds cool",
    );
  }

  {
    let result = fixture.lint(
      r#"
Account = { balance=0 }

function getAccount()
    return Account
end

@[deprecated{use = 'credit', reason = 'It sounds cool'}]
function Account:deposit (v)
    self.balance = self.balance + v
end

(getAccount()):deposit(200.00)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(12, 0),
      Position::new(12, 22),
      "Member 'deposit' is deprecated, use 'credit' instead. It sounds cool",
    );
  }

  {
    fixture.load_definition(
      r#"
@[deprecated{use = 'foo', reason = 'Do better.'}] declare function bar(x: number): string
"#,
      false,
    );

    let result = fixture.lint(
      r#"
bar(2)
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(1, 0),
      Position::new(1, 3),
      "Function 'bar' is deprecated, use 'foo' instead. Do better.",
    );
  }

  {
    fixture.load_definition(
      r#"
declare Hooty : {
    tooty : @[deprecated{use = 'foo', reason = 'bar'}] @checked (number) -> number
}
"#,
      false,
    );

    let result = fixture.lint(
      r#"
print(Hooty:tooty(2.0))
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(1, 6),
      Position::new(1, 17),
      "Member 'Hooty.tooty' is deprecated, use 'foo' instead. bar",
    );
  }

  {
    fixture.load_definition(
      r#"
declare extern type Foo with
   @[deprecated{use = 'foo', reason = 'baz'}]
   function bar(self, value: number) : number
end

declare Foo: {
   new: () -> Foo
}
"#,
      false,
    );

    let result = fixture.lint(
      r#"
local foo = Foo.new()
print(foo:bar(2.0))
"#,
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
      &result.warnings[0],
      Position::new(2, 6),
      Position::new(2, 13),
      "Member 'bar' is deprecated, use 'foo' instead. baz",
    );
  }
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_global() {
  let mut fixture = Fixture::fixture_bool(false);
  let any_type = fixture.get_builtins().any_type;
  add_global_binding_value(
    &mut fixture.get_frontend().globals,
    "Wait",
    Binding {
      type_id: any_type,
      location: Location::default(),
      deprecated: true,
      deprecated_suggestion: String::from("wait"),
      documentation_symbol: Some(String::from("@test/global/Wait")),
    },
  );

  let result = fixture.lint("Wait(5)", None);

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Global 'Wait' is deprecated, use 'wait' instead",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_deprecated_global_no_replacement() {
  let mut fixture = Fixture::fixture_bool(false);
  let any_type = fixture.get_builtins().any_type;
  add_global_binding_value(
    &mut fixture.get_frontend().globals,
    "Version",
    Binding {
      type_id: any_type,
      location: Location::default(),
      deprecated: true,
      deprecated_suggestion: String::new(),
      documentation_symbol: None,
    },
  );

  let result = fixture.lint("Version()", None);

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Global 'Version' is deprecated",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_disable_unknown_global_with_type_checking() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        --!strict
        unknownGlobal()
    "#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        if true then
            function c() end
        else
            function c() end
        end

        return c
    "#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_conditions() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(

          r#"
if true then
elseif false then
elseif true then -- duplicate
end

if true then
elseif false then
else
    if true then -- duplicate
    end
end

_ = true and true
_ = true or true
_ = (true and false) and true
_ = (true and true) and true
_ = (true and true) or true
_ = (true and false) and (42 and false)

_ = true and true or false -- no warning since this is is a common pattern used as a ternary replacement

_ = if true then 1 elseif true then 2 else 3
"#
,
      None,
  );

  assert_eq!(8, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Condition has already been checked on line 2",
    result.warnings[0].text.as_str()
  );
  assert_eq!(4, result.warnings[0].location.begin.line + 1);
  assert_eq!(
    "Condition has already been checked on column 5",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "Condition has already been checked on column 5",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "Condition has already been checked on column 6",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    "Condition has already been checked on column 6",
    result.warnings[4].text.as_str()
  );
  assert_eq!(
    "Condition has already been checked on column 6",
    result.warnings[5].text.as_str()
  );
  assert_eq!(
    "Condition has already been checked on column 15",
    result.warnings[6].text.as_str()
  );
  assert_eq!(19, result.warnings[6].location.begin.line + 1);
  assert_eq!(
    "Condition has already been checked on column 8",
    result.warnings[7].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_conditions_expr() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local correct, opaque = ...

if correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", `string {opaque}`)}) then
elseif correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", `string {opaque}`)}) then
elseif correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", false)}) then
end
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Condition has already been checked on line 4",
    result.warnings[0].text.as_str()
  );
  assert_eq!(5, result.warnings[0].location.begin.line + 1);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_conditions_if_stat_and_expr() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
if if 1 then 2 else 3 then
elseif if 1 then 2 else 3 then
elseif if 0 then 5 else 4 then
end
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Condition has already been checked on line 2",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_global_function() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        function x() end

        function x() end

        return x
    "#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);

  let warning = &result.warnings[0];
  assert_eq!(Code::DuplicateFunction, warning.code);
  assert_eq!(
    "Duplicate function definition: 'x' also defined on line 2",
    warning.text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_local() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo(a1, a2, a3, a1)
end

local _, _, _ = ... -- ok!
local a1, a2, a1 = ... -- not ok

local moo = {}
function moo:bar(self)
end

return foo, moo, a1, a2
"#,
    None,
  );

  assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Function parameter 'a1' already defined on column 14",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Variable 'a1' is never used; prefix with '_' to silence",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "Variable 'a1' already defined on column 7",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "Function parameter 'self' already defined implicitly",
    result.warnings[3].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_local_function() {
  use ulua_config::records::lint_options::LintOptions;

  let mut options = LintOptions::default();
  options.set_defaults();
  options.enable_warning(Code::DuplicateFunction);
  options.enable_warning(Code::LocalShadow);

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        local function x() end

        print(x)

        local function x() end

        return x
    "#,
    Some(options),
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(Code::DuplicateFunction, result.warnings[0].code);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_duplicate_method() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        local T = {}
        function T:x() end

        function T:x() end

        return x
    "#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);

  let warning = &result.warnings[0];
  assert_eq!(Code::DuplicateFunction, warning.code);
  assert_eq!(
    "Duplicate function definition: 'T.x' also defined on line 3",
    warning.text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_for_range_backwards() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
for i=8,1 do
end

for i=8,1,-1 do
end
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(1, result.warnings[0].location.begin.line);
  assert_eq!(
    "For loop should iterate backwards; did you forget to specify -1 as step?",
    result.warnings[0].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_for_range_imprecise() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
for i=1.3,7.5 do
end

for i=1.3,7.5,1 do
end
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(1, result.warnings[0].location.begin.line);
  assert_eq!(
    "For loop ends at 7.3 instead of 7.5; did you forget to specify step?",
    result.warnings[0].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_for_range_table() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local t = {}

for i=#t,1 do
end

for i=#t,1,-1 do
end
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(3, result.warnings[0].location.begin.line);
  assert_eq!(
    "For loop should iterate backwards; did you forget to specify -1 as step?",
    result.warnings[0].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_for_range_zero() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
for i=0,#t do
end

for i=(0),#t do -- to silence
end

for i=#t,0 do
end
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(1, result.warnings[0].location.begin.line);
  assert_eq!(
    "For loop starts at 0, but arrays start at 1",
    result.warnings[0].text
  );
  assert_eq!(7, result.warnings[1].location.begin.line);
  assert_eq!(
    "For loop should iterate backwards; did you forget to specify -1 as step? Also consider changing 0 to 1 since arrays start at 1",
    result.warnings[1].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_date() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
-- incorrect formats
os.date("%")
os.date("%l")
os.date("%?")
os.date("\0")

-- correct formats
os.date("it's %c now")
os.date("!*t")
"#,
    None,
  );

  assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid date format: unfinished replacement",
    result.warnings[0].text
  );
  assert_eq!(
    "Invalid date format: unexpected replacement character; must be a date format specifier or %",
    result.warnings[1].text
  );
  assert_eq!(
    "Invalid date format: unexpected replacement character; must be a date format specifier or %",
    result.warnings[2].text
  );
  assert_eq!(
    "Invalid date format: date format can not contain null characters",
    result.warnings[3].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_find_args() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local s = ...

-- incorrect character class specifier
string.find(s, "%q")

-- raw string find
string.find(s, "%q", 1, true)
string.find(s, "%q", 1, math.random() < 0.5)

-- incorrect character class specifier
string.find(s, "%q", 1, false)

-- missing arguments
string.find()
string.find("foo");
("foo"):find()
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[0].text
  );
  assert_eq!(4, result.warnings[0].location.begin.line);
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[1].text
  );
  assert_eq!(11, result.warnings[1].location.begin.line);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_format() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
-- incorrect format strings
string.format("%")
string.format("%??d")
string.format("%Y")

-- incorrect format strings, self call
local _ = ("%"):format()

-- correct format strings, just to uh make sure
string.format("hello %+10d %.02f %%", 4, 5)
"#,
    None,
  );

  assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid format string: unfinished format specifier",
    result.warnings[0].text
  );
  assert_eq!(
    "Invalid format string: invalid format specifier: must be a string format specifier or %",
    result.warnings[1].text
  );
  assert_eq!(
    "Invalid format string: invalid format specifier: must be a string format specifier or %",
    result.warnings[2].text
  );
  assert_eq!(
    "Invalid format string: unfinished format specifier",
    result.warnings[3].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_match() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local s = ...

-- incorrect character class specifiers
string.match(s, "%q")
string.gmatch(s, "%q")
string.find(s, "%q")
string.gsub(s, "%q", "")

-- various errors
string.match(s, "%")
string.match(s, "[%1]")
string.match(s, "%0")
string.match(s, "(%d)%2")
string.match(s, "%bx")
string.match(s, "%foo")
string.match(s, '(%d))')
string.match(s, '(%d')
string.match(s, '[%d')
string.match(s, '%,')

-- self call - not detected because we don't know the type!
local _ = s:match("%q")

-- correct patterns
string.match(s, "[A-Z]+(%d)%1")
"#,
    None,
  );

  assert_eq!(14, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[0].text
  );
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[1].text
  );
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[2].text
  );
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[3].text
  );
  assert_eq!(
    "Invalid match pattern: unfinished character class",
    result.warnings[4].text
  );
  assert_eq!(
    "Invalid match pattern: sets can not contain capture references",
    result.warnings[5].text
  );
  assert_eq!(
    "Invalid match pattern: invalid capture reference, must be 1-9",
    result.warnings[6].text
  );
  assert_eq!(
    "Invalid match pattern: invalid capture reference, must refer to a valid capture",
    result.warnings[7].text
  );
  assert_eq!(
    "Invalid match pattern: missing brace characters for balanced match",
    result.warnings[8].text
  );
  assert_eq!(
    "Invalid match pattern: missing set after a frontier pattern",
    result.warnings[9].text
  );
  assert_eq!(
    "Invalid match pattern: unexpected ) without a matching (",
    result.warnings[10].text
  );
  assert_eq!(
    "Invalid match pattern: expected ) at the end of the string to close a capture",
    result.warnings[11].text
  );
  assert_eq!(
    "Invalid match pattern: expected ] at the end of the string to close a set",
    result.warnings[12].text
  );
  assert_eq!(
    "Invalid match pattern: expected a magic character after %",
    result.warnings[13].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_match_nested() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local s = ...

-- correct reference to nested pattern
string.match(s, "((a)%2)")

-- incorrect reference to nested pattern (not closed yet)
string.match(s, "((a)%1)")

-- incorrect reference to nested pattern (index out of range)
string.match(s, "((a)%3)")
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid match pattern: invalid capture reference, must refer to a closed capture",
    result.warnings[0].text
  );
  assert_eq!(7, result.warnings[0].location.begin.line);
  assert_eq!(
    "Invalid match pattern: invalid capture reference, must refer to a valid capture",
    result.warnings[1].text
  );
  assert_eq!(10, result.warnings[1].location.begin.line);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_match_sets() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local s = ...

-- fake empty sets (but actually sets that aren't closed)
string.match(s, "[]")
string.match(s, "[^]")

-- character ranges in sets
string.match(s, "[%a-b]")
string.match(s, "[a-%b]")

-- invalid escapes
string.match(s, "[%q]")
string.match(s, "[%;]")

-- capture refs in sets
string.match(s, "[%1]")

-- valid escapes and - at the end
string.match(s, "[%]x-]")

-- % escapes itself
string.match(s, "[%%]")

-- this abomination is a valid pattern due to rules wrt handling empty sets
string.match(s, "[]|'[]")
string.match(s, "[^]|'[]")
"#,
    None,
  );

  assert_eq!(7, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid match pattern: expected ] at the end of the string to close a set",
    result.warnings[0].text
  );
  assert_eq!(
    "Invalid match pattern: expected ] at the end of the string to close a set",
    result.warnings[1].text
  );
  assert_eq!(
    "Invalid match pattern: character range can't include character sets",
    result.warnings[2].text
  );
  assert_eq!(
    "Invalid match pattern: character range can't include character sets",
    result.warnings[3].text
  );
  assert_eq!(
    "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
    result.warnings[4].text
  );
  assert_eq!(
    "Invalid match pattern: expected a magic character after %",
    result.warnings[5].text
  );
  assert_eq!(
    "Invalid match pattern: sets can not contain capture references",
    result.warnings[6].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_pack() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
-- incorrect pack specifiers
string.pack("?")
string.packsize("?")
string.unpack("?")

-- missing size
string.packsize("bc")

-- incorrect X alignment
string.packsize("X")
string.packsize("X i")

-- correct X alignment
string.packsize("Xi")

-- packsize can't be used with variable sized formats
string.packsize("s")

-- out of range size specifiers
string.packsize("i0")
string.packsize("i17")

-- a very very very out of range size specifier
string.packsize("i99999999999999999999")
string.packsize("c99999999999999999999")

-- correct format specifiers
string.packsize("=!1bbbI3c42")
"#,
    None,
  );

  assert_eq!(11, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid pack format: unexpected character; must be a pack specifier or space",
    result.warnings[0].text
  );
  assert_eq!(
    "Invalid pack format: unexpected character; must be a pack specifier or space",
    result.warnings[1].text
  );
  assert_eq!(
    "Invalid pack format: unexpected character; must be a pack specifier or space",
    result.warnings[2].text
  );
  assert_eq!(
    "Invalid pack format: fixed-sized string format must specify the size",
    result.warnings[3].text
  );
  assert_eq!(
    "Invalid pack format: X must be followed by a size specifier",
    result.warnings[4].text
  );
  assert_eq!(
    "Invalid pack format: X must be followed by a size specifier",
    result.warnings[5].text
  );
  assert_eq!(
    "Invalid pack format: pack specifier must be fixed-size",
    result.warnings[6].text
  );
  assert_eq!(
    "Invalid pack format: integer size must be in range [1,16]",
    result.warnings[7].text
  );
  assert_eq!(
    "Invalid pack format: integer size must be in range [1,16]",
    result.warnings[8].text
  );
  assert_eq!(
    "Invalid pack format: size specifier is too large",
    result.warnings[9].text
  );
  assert_eq!(
    "Invalid pack format: size specifier is too large",
    result.warnings[10].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_replace() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local s = ...

-- incorrect replacements
string.gsub(s, '(%d+)', "%")
string.gsub(s, '(%d+)', "%x")
string.gsub(s, '(%d+)', "%2")
string.gsub(s, '', "%1")

-- correct replacements
string.gsub(s, '[A-Z]+(%d)', "%0%1")
string.gsub(s, 'foo', "%0")
"#,
    None,
  );

  assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid match replacement: unfinished replacement",
    result.warnings[0].text
  );
  assert_eq!(
    "Invalid match replacement: unexpected replacement character; must be a digit or %",
    result.warnings[1].text
  );
  assert_eq!(
    "Invalid match replacement: invalid capture index, must refer to pattern capture",
    result.warnings[2].text
  );
  assert_eq!(
    "Invalid match replacement: invalid capture index, must refer to pattern capture",
    result.warnings[3].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_format_string_typed() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local s: string, nons = ...

string.match(s, "[]")
s:match("[]")

-- no warning here since we don't know that it's a string
nons:match("[]")
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Invalid match pattern: expected ] at the end of the string to close a set",
    result.warnings[0].text.as_str()
  );
  assert_eq!(3, result.warnings[0].location.begin.line);
  assert_eq!(
    "Invalid match pattern: expected ] at the end of the string to close a set",
    result.warnings[1].text.as_str()
  );
  assert_eq!(4, result.warnings[1].location.begin.line);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_function_unused() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar()
end

local function qux()
end

function foo()
end

local function _unusedl()
end

function _unusedg()
end

return foo()
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Function 'bar' is never used; prefix with '_' to silence",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Function 'qux' is never used; prefix with '_' to silence",
    result.warnings[1].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar()
    foo = 6
    return foo
end

return bar()
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Global 'foo' is only used in the enclosing function 'bar'; consider changing it to local",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local3_with_conditional_read() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar()
    foo = 6
    return foo
end

function baz()
    foo = 6
    return foo
end

function read()
    if false then print(foo) end
end

return bar() + baz() + read()
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local_inner_read() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo()
   local f = function() return bar end
   f()
   bar = 42
end

function baz() bar = 0 end

return foo() + baz()
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local_multi() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local createFunction = function(configValue)
    -- Create an internal convenience function
    local function internalLogic()
        print(configValue) -- prints passed-in value
    end
    -- Here, we thought we were creating another internal convenience function
    -- that closed over the passed-in configValue, but this is actually being
    -- declared at module scope!
    function moreInternalLogic()
        print(configValue) -- nil!!!
    end
    return function()
        internalLogic()
        moreInternalLogic()
        return nil
    end
end
fnA = createFunction(true)
fnB = createFunction(false)
fnA() -- prints "true", "nil"
fnB() -- prints "false", "nil"
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Global 'moreInternalLogic' is only used in the enclosing function defined at line 2; consider changing it to local",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local_multi_fx() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar()
    foo = 6
    return foo
end

function baz()
    foo = 6
    return foo
end

return bar() + baz()
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Global 'foo' is never read before being written. Consider changing it to local",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local_multi_fx_with_read() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar()
    foo = 6
    return foo
end

function baz()
    foo = 6
    return foo
end

function read()
    print(foo)
end

return bar() + baz() + read()
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_global_as_local_with_conditional() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar()
    if true then foo = 6 end
    return foo
end

function baz()
    foo = 6
    return foo
end

return bar() + baz()
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_ignore_lint_all() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!nolint
return foo
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_ignore_lint_specific() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!nolint UnknownGlobal
local x = 1
return foo
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'x' is never used; prefix with '_' to silence",
    result.warnings[0].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_implicit_return() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!nonstrict
function f1(a)
    if not a then
        return 5
    end
end

function f2(a)
    if not a then
        return
    end
end

function f3(a)
    if not a then
        return 5
    else
        return
    end
end

function f4(a)
    for i in pairs(a) do
        if i > 5 then
            return i
        end
    end

    print("element not found")
end

function f5(a)
    for i in pairs(a) do
        if i > 5 then
            return i
        end
    end

    error("element not found")
end

f6 = function(a)
    if a == 0 then
        return 42
    end
end

function f7(a)
    repeat
        return 10
    until a ~= nil
end

return f1,f2,f3,f4,f5,f6,f7
"#,
    None,
  );

  assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(5, result.warnings[0].location.begin.line);
  assert_eq!(
    "Function 'f1' can implicitly return no values even though there's an explicit return at line 5; add explicit return to silence",
    result.warnings[0].text
  );
  assert_eq!(29, result.warnings[1].location.begin.line);
  assert_eq!(
    "Function 'f4' can implicitly return no values even though there's an explicit return at line 26; add explicit return to silence",
    result.warnings[1].text
  );
  assert_eq!(45, result.warnings[2].location.begin.line);
  assert_eq!(
    "Function can implicitly return no values even though there's an explicit return at line 45; add explicit return to silence",
    result.warnings[2].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_implicit_return_infinite_loop() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!nonstrict
function f1(a)
    while true do
        if math.random() > 0.5 then
            return 5
        end
    end
end

function f2(a)
    repeat
        if math.random() > 0.5 then
            return 5
        end
    until false
end

function f3(a)
    while true do
        if math.random() > 0.5 then
            return 5
        end
        if math.random() < 0.1 then
            break
        end
    end
end

function f4(a)
    repeat
        if math.random() > 0.5 then
            return 5
        end
        if math.random() < 0.1 then
            break
        end
    until false
end

return f1,f2,f3,f4
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(26, result.warnings[0].location.begin.line);
  assert_eq!(
    "Function 'f3' can implicitly return no values even though there's an explicit return at line 22; add explicit return to silence",
    result.warnings[0].text
  );
  assert_eq!(37, result.warnings[1].location.begin.line);
  assert_eq!(
    "Function 'f4' can implicitly return no values even though there's an explicit return at line 33; add explicit return to silence",
    result.warnings[1].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_import_only_used_in_return_type() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        local Foo = require(script.Parent.Foo)

        function foo(): Foo.Y
        end
    "#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Function 'foo' is never used; prefix with '_' to silence",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_import_only_used_in_type_annotation() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        local Foo = require(script.Parent.Foo)

        local x: Foo.Y = 1
    "#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'x' is never used; prefix with '_' to silence",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_import_unused() {
  use ulua_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;

  let mut fixture = Fixture::fixture_bool(false);
  let any_type = fixture.get_builtins().any_type;
  add_global_binding_builtin_definitions(
    &mut fixture.get_frontend().globals,
    "game",
    any_type,
    "@test",
  );

  let result = fixture.lint(
    r#"
local Roact = require(game.Packages.Roact)
local _Roact = require(game.Packages.Roact)
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Import 'Roact' is never used; prefix with '_' to silence",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_integer_parsing() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local _ = 0b10000000000000000000000000000000000000000000000000000000000000000
local _ = 0x10000000000000000
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Binary number literal exceeded available precision and was truncated to 2^64",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Hexadecimal number literal exceeded available precision and was truncated to 2^64",
    result.warnings[1].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_integer_parsing_decimal_imprecise() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local _ = 10000000000000000000000000000000000000000000000000000000000000000
local _ = 10000000000000001
local _ = -10000000000000001

-- 10^16 = 2^16 * 5^16, 5^16 only requires 38 bits
local _ = 10000000000000000
local _ = -10000000000000000

-- smallest possible number that is parsed imprecisely
local _ = 9007199254740993
local _ = -9007199254740993

-- note that numbers before and after parse precisely (number after is even => 1 more mantissa bit)
local _ = 9007199254740992
local _ = 9007199254740994

-- large powers of two should work as well (this is 2^63)
local _ = -9223372036854775808
"#,
    None,
  );

  assert_eq!(5, result.warnings.len(), "{:?}", result.warnings);
  for warning in &result.warnings {
    assert_eq!(
      "Number literal exceeded available precision and was truncated to closest representable number",
      warning.text.as_str()
    );
  }
  assert_eq!(1, result.warnings[0].location.begin.line);
  assert_eq!(2, result.warnings[1].location.begin.line);
  assert_eq!(3, result.warnings[2].location.begin.line);
  assert_eq!(10, result.warnings[3].location.begin.line);
  assert_eq!(11, result.warnings[4].location.begin.line);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_integer_parsing_hex_imprecise() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local _ = 0x1234567812345678

-- smallest possible number that is parsed imprecisely
local _ = 0x20000000000001

-- note that numbers before and after parse precisely (number after is even => 1 more mantissa bit)
local _ = 0x20000000000000
local _ = 0x20000000000002

-- large powers of two should work as well (this is 2^63)
local _ = 0x80000000000000
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Number literal exceeded available precision and was truncated to closest representable number",
    result.warnings[0].text.as_str()
  );
  assert_eq!(1, result.warnings[0].location.begin.line);
  assert_eq!(
    "Number literal exceeded available precision and was truncated to closest representable number",
    result.warnings[1].text.as_str()
  );
  assert_eq!(4, result.warnings[1].location.begin.line);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_lint_hygiene_uaf() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        local Hooty = require(workspace.A)

        local  HoHooty = require(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        local h: H
        local h: Hooty.Pointy = ruire(workspace.A)

        local hh: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        linooty.Pointy = ruire(workspace.A)

        local hh: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        linty = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        local hh: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pt
    "#,
    None,
  );

  assert_eq!(12, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_local_function_not_dead() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local foo
function foo() end
    "#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_local_shadow_argument() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function bar(a, b)
    local a = b + 1
    return a
end

return bar()
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'a' shadows previous declaration at line 2",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_local_shadow_global() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.lint(
    r#"
local math = math
global = math

function bar()
    local global = math.max(5, 1)
    return global
end

return bar()
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'global' shadows a global variable used at line 3",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_local_shadow_local() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local arg = 6
print(arg)

local arg = 5
print(arg)
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'arg' shadows previous declaration at line 2",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_local_unused() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local arg = 6

local function bar()
    local arg = 5
    local blarg = 6
    if arg then
        blarg = 42
    end
end

return bar()
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Variable 'arg' is never used; prefix with '_' to silence",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Variable 'blarg' is never used; prefix with '_' to silence",
    result.warnings[1].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_misleading_and_or() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
_ = math.random() < 0.5 and true or 42
_ = math.random() < 0.5 and false or 42 -- misleading
_ = math.random() < 0.5 and nil or 42 -- misleading
_ = math.random() < 0.5 and 0 or 42
_ = (math.random() < 0.5 and false) or 42 -- currently ignored
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    concat!(
      "The and-or expression always evaluates to the second alternative because the first alternative is false; ",
      "consider using if-then-else expression instead"
    ),
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    concat!(
      "The and-or expression always evaluates to the second alternative because the first alternative is nil; ",
      "consider using if-then-else expression instead"
    ),
    result.warnings[1].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_multiline_block() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
if true then print(1) print(2) print(3) end
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "A new statement is on the same line; add semi-colon on previous statement to silence",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_multiline_block_local_do() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local _x do
    _x = 5
end
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_multiline_block_missed_semicolon() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
print(1); print(2) print(3)
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "A new statement is on the same line; add semi-colon on previous statement to silence",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_multiline_block_semicolons_whitelisted() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
print(1); print(2); print(3)
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_no_spurious_warning_after_a_function_type_alias() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        local exports = {}
        export type PathFunction<P> = (P?) -> string
        exports.tokensToFunction = function() end
        return exports
    "#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_placeholder_read() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local _ = 5
return _
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Placeholder value '_' is read here; consider using a named variable",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_placeholder_read_global() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
_ = 5
print(_)
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Placeholder value '_' is read here; consider using a named variable",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_placeholder_write() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local _ = 5
_ = 6
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_read_write_table_props() {
  ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"-- line 1
        type A = {x: number}
        type B = {read x: number, write x: number}
        type C = {x: number, read x: number} -- line 4
        type D = {x: number, write x: number}
        type E = {read x: number, x: boolean}
        type F = {read x: number, read x: number}
        type G = {write x: number, x: boolean}
        type H = {write x: number, write x: boolean}
    "#,
    None,
  );

  assert_eq!(6, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Table type field 'x' is already read-write; previously defined at line 4",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Table type field 'x' is already read-write; previously defined at line 5",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "Table type field 'x' already has a read type defined at line 6",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "Table type field 'x' is a duplicate; previously defined at line 7",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    "Table type field 'x' already has a write type defined at line 8",
    result.warnings[4].text.as_str()
  );
  assert_eq!(
    "Table type field 'x' is a duplicate; previously defined at line 9",
    result.warnings[5].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_redundant_native_attribute() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!native

@native
local function f(a)
    @native
    local function g(b)
        return (a + b)
    end
    return g
end

f(3)(4)
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "native attribute on a function is redundant in a native module; consider removing it",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    Location {
      begin: Position { line: 3, column: 0 },
      end: Position { line: 3, column: 7 },
    },
    result.warnings[0].location
  );
  assert_eq!(
    "native attribute on a function is redundant in a native module; consider removing it",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    Location {
      begin: Position { line: 5, column: 4 },
      end: Position {
        line: 5,
        column: 11,
      },
    },
    result.warnings[1].location
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_table_literal() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"-- line 1
_ = {
    first = 1,
    second = 2,
    first = 3,
}

_ = {
    first = 1,
    ["first"] = 2,
}

_ = {
    1, 2, 3,
    [1] = 42
}

_ = {
    [3] = 42,
    1, 2, 3,
}

local _: {
    first: number,
    second: string,
    first: boolean
}

_ = {
    1, 2, 3,
    [0] = 42,
    [4] = 42,
}

_ = {
    [1] = 1,
    [2] = 2,
    [1] = 3,
}

function _foo(): { first: number, second: string, first: boolean }
end
"#,
    None,
  );

  assert_eq!(7, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Table field 'first' is a duplicate; previously defined at line 3",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Table field 'first' is a duplicate; previously defined at line 9",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "Table index 1 is a duplicate; previously defined as a list entry",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "Table index 3 is a duplicate; previously defined as a list entry",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    "Table type field 'first' is a duplicate; previously defined at line 24",
    result.warnings[4].text.as_str()
  );
  assert_eq!(
    "Table index 1 is a duplicate; previously defined at line 36",
    result.warnings[5].text.as_str()
  );
  assert_eq!(
    "Table type field 'first' is a duplicate; previously defined at line 41",
    result.warnings[6].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_table_operations() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.lint(
    r#"
local t = {}
local tt = {}

table.insert(t, #t, 42)
table.insert(t, (#t), 42) -- silenced

table.insert(t, #t + 1, 42)
table.insert(t, #tt + 1, 42) -- different table, ok

table.insert(t, 0, 42)

table.remove(t, 0)

table.remove(t, #t-1)

table.insert(t, string.find("hello", "h"))

table.move(t, 0, #t, 1, tt)
table.move(t, 1, #t, 0, tt)

table.create(42, {})
table.create(42, {} :: {})
"#,
    None,
  );

  assert_eq!(10, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    concat!(
      "table.insert will insert the value before the last element, which is likely a bug; consider removing the ",
      "second argument or wrap it in parentheses to silence"
    ),
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "table.insert will append the value to the table; consider removing the second argument for efficiency",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "table.insert uses index 0 but arrays are 1-based; did you mean 1 instead?",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "table.remove uses index 0 but arrays are 1-based; did you mean 1 instead?",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    concat!(
      "table.remove will remove the value before the last element, which is likely a bug; consider removing the ",
      "second argument or wrap it in parentheses to silence"
    ),
    result.warnings[4].text.as_str()
  );
  assert_eq!(
    "table.insert may change behavior if the call returns more than one result; consider adding parentheses around second argument",
    result.warnings[5].text.as_str()
  );
  assert_eq!(
    "table.move uses index 0 but arrays are 1-based; did you mean 1 instead?",
    result.warnings[6].text.as_str()
  );
  assert_eq!(
    "table.move uses index 0 but arrays are 1-based; did you mean 1 instead?",
    result.warnings[7].text.as_str()
  );
  assert_eq!(
    "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead",
    result.warnings[8].text.as_str()
  );
  assert_eq!(
    "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead",
    result.warnings[9].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_table_operations_indexer() {
  use ulua_common::fflag;

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.lint(

          r#"
local t1 = {} -- ok: empty
local t2 = {1, 2} -- ok: array
local t3 = { a = 1, b = 2 } -- not ok: dictionary
local t4: {[number]: number} = {} -- ok: array
local t5: {[string]: number} = {} -- not ok: dictionary
local t6: typeof(setmetatable({1, 2}, {})) = {} -- ok: table with metatable
local t7: string = "hello" -- ok: string
local t8: {number} | {n: number} = {} -- ok: union

-- not ok
print(#t3)
print(#t5)
ipairs(t5)

-- disabled
-- ipairs(t3) adds indexer to t3, silencing error on #t3

-- ok
print(#t1)
print(#t2)
print(#t4)
print(#t6)
print(#t7)
print(#t8)

ipairs(t1)
ipairs(t2)
ipairs(t4)
ipairs(t6)
ipairs(t7)
ipairs(t8)

-- ok, subtle: text is a string here implicitly, but the type annotation isn't available
-- type checker assigns a type of generic table with the 'sub' member; we don't emit warnings on generic tables
-- to avoid generating a false positive here
function _impliedstring(element, text)
        for i = 1, #text do
                element:sendText(text:sub(i, i))
        end
end
"#
,
      None,
  );

  assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(12, result.warnings[0].location.begin.line + 1);
  assert_eq!(
    "Using '#' on a table without an array part is likely a bug",
    result.warnings[0].text.as_str()
  );
  assert_eq!(13, result.warnings[1].location.begin.line + 1);
  assert_eq!(
    "Using '#' on a table with string keys is likely a bug",
    result.warnings[1].text.as_str()
  );
  assert_eq!(14, result.warnings[2].location.begin.line + 1);
  assert_eq!(
    "Using 'ipairs' on a table with string keys is likely a bug",
    result.warnings[2].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_test_string_interpolation() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
        --!nocheck
        local _ = `unknown {foo}`
    "#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_type_annotations_should_not_produce_warnings() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"--!strict
type InputData = {
    id: number,
    inputType: EnumItem,
    inputState: EnumItem,
    updated: number,
    position: Vector3,
    keyCode: EnumItem,
    name: string
}
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_type_function_fully_reduces() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function fib(n)
    return n < 2 or  fib(n-2)
end

"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_type_instantiation_lints() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
local function a<b>(cool: b)
    print(cool)
end

a<<"hi">>("hi")
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unbalanced_assignment() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
do
local _a,_b,_c = pcall()
end
do
local _a,_b,_c = pcall(), 5
end
do
local _a,_b,_c = pcall(), 5, 6
end
do
local _a,_b,_c = pcall(), 5, 6, 7
end
do
local _a,_b,_c = pcall(), nil
end
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(5, result.warnings[0].location.begin.line);
  assert_eq!(
    "Assigning 2 values to 3 variables initializes extra variables with nil; add 'nil' to value list to silence",
    result.warnings[0].text
  );
  assert_eq!(11, result.warnings[1].location.begin.line);
  assert_eq!(
    "Assigning 4 values to 3 variables leaves some values unused",
    result.warnings[1].text
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unknown_global() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint("--!nocheck\nreturn foo", None);

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Unknown global 'foo'; consider assigning to it first",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unknown_type() {
  use ulua_analysis::type_aliases::props_type::Props;

  let mut fixture = Fixture::fixture_bool(false);
  let any_type = fixture.get_builtins().any_type;

  {
    let frontend = fixture.get_frontend();
    unfreeze(frontend.globals.global_types_mut());

    let scope = frontend.globals.global_scope();
    let scope_ptr = Arc::as_ptr(&scope) as *mut Scope;
    let mut instance_props = Props::default();
    instance_props.insert(String::from("ClassName"), Property::rw_type_id(any_type));

    let instance_table =
      TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
        &instance_props,
        None,
        TypeLevel::default(),
        scope_ptr,
        TableState::Sealed,
      );
    let instance_type = frontend.globals.global_types_mut().add_type(instance_table);

    unsafe {
      (*scope_ptr).exported_type_bindings.insert(
        String::from("Part"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          Vec::new(),
          instance_type,
          None,
        ),
      );
    }
  }

  let result = fixture.lint(
    r#"
local game = ...
local _e01 = type(game) == "Part"
local _e02 = typeof(game) == "Bar"
local _ok = typeof(game) == "vector"

local _o01 = type(game) == "number"
local _o02 = type(game) == "vector"
local _o03 = typeof(game) == "Part"
"#,
    None,
  );

  assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(2, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unknown type 'Part' (expected primitive type)",
    result.warnings[0].text
  );
  assert_eq!(3, result.warnings[1].location.begin.line);
  assert_eq!("Unknown type 'Bar'", result.warnings[1].text);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_assert_false_return_silent() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    if a then
        return 'z'
    end

    assert(false)
end

return foo1
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_basic() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
do
return 'ok'
end

print("hi!")
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(5, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unreachable code (previous statement always returns)",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_error_return_non_silent_branchy() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    if a then
        error('x')
    else
        error('y')
    end
    return 'z'
end

return foo1
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(7, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unreachable code (previous statement always errors)",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_error_return_propagate() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    if a then
        error('x')
        return 'z'
    else
        error('y')
    end
    return 'x'
end

return foo1
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(8, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unreachable code (previous statement always errors)",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_error_return_silent() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    if a then
        error('x')
        return 'z'
    else
        error('y')
    end
end

return foo1
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_if_merge() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    if a then
        return 'x'
    else
        return 'y'
    end
    return 'z'
end

function foo2(a)
    if a then
        return 'x'
    end
    return 'z'
end

function foo3(a)
    if a then
        return 'x'
    else
        print('y')
    end
    return 'z'
end

return { foo1, foo2, foo3 }
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(7, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unreachable code (previous statement always returns)",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_loop_break() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
while true do
    do break end
    print("nope")
end

print("hi!")
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(3, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unreachable code (previous statement always breaks)",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_loop_continue() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
while true do
    do continue end
    print("nope")
end

print("hi!")
"#,
    None,
  );

  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(3, result.warnings[0].location.begin.line);
  assert_eq!(
    "Unreachable code (previous statement always continues)",
    result.warnings[0].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_loop_repeat() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    repeat
        return 'z'
    until a
    return 'x'
end

return foo1
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_unreachable_code_loop_while() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
function foo1(a)
    while a do
        return 'z'
    end
    return 'x'
end

return foo1
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_use_all_parent_scopes_for_globals() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let test_scope = fixture.get_frontend().add_environment(String::from("Test"));
  let frontend = fixture.get_frontend();
  unfreeze(frontend.globals.global_types_mut());
  let result = frontend.load_definition_file(
    |frontend| &mut frontend.globals,
    test_scope,
    r#"
        declare Foo: number
    "#,
    String::from("@test"),
    false,
    false,
  );
  assert!(result.success, "{:?}", result);
  freeze(frontend.globals.global_types_mut());

  fixture
    .base
    .file_resolver
    .environments
    .insert(ModuleName::from("A"), String::from("Test"));
  fixture.base.file_resolver.source.insert(
    String::from("A"),
    String::from(
      r#"
        local _foo: Foo = 123
        -- os.clock comes from the global scope, the parent of this module's environment
        local _bar: typeof(os.clock) = os.clock
    "#,
    ),
  );

  let result = fixture.base.lint_module(&ModuleName::from("A"), None);

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_wrong_comment() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!strict
--!struct
--!nolintGlobal
--!nolint Global
--!nolint KnownGlobal
--!nolint UnknownGlobal
--! no more lint
--!strict here
--!native on
do end
--!nolint
"#,
    None,
  );

  assert_eq!(7, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "Unknown comment directive 'struct'; did you mean 'strict'?",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "Unknown comment directive 'nolintGlobal'",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "nolint directive refers to unknown lint rule 'Global'",
    result.warnings[2].text.as_str()
  );
  assert_eq!(
    "nolint directive refers to unknown lint rule 'KnownGlobal'; did you mean 'UnknownGlobal'?",
    result.warnings[3].text.as_str()
  );
  assert_eq!(
    "Comment directive with the type checking mode has extra symbols at the end of the line",
    result.warnings[4].text.as_str()
  );
  assert_eq!(
    "native directive has extra symbols at the end of the line",
    result.warnings[5].text.as_str()
  );
  assert_eq!(
    "Comment directive is ignored because it is placed after the first non-comment token",
    result.warnings[6].text.as_str()
  );
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_wrong_comment_mute_self() {
  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.lint(
    r#"
--!nolint
--!struct
"#,
    None,
  );

  assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}

// Ported from `tests/Linter.test.cpp`.
// Source: `tests/Linter.test.cpp`
#[test]
fn linter_wrong_comment_optimize() {
  let mut fixture = Fixture::fixture_bool(false);
  let mut result = fixture.lint(
    r#"
--!optimize
--!optimize me
--!optimize 100500
--!optimize 2
"#,
    None,
  );

  assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "optimize directive requires an optimization level",
    result.warnings[0].text.as_str()
  );
  assert_eq!(
    "optimize directive uses unknown optimization level 'me', 0..2 expected",
    result.warnings[1].text.as_str()
  );
  assert_eq!(
    "optimize directive uses unknown optimization level '100500', 0..2 expected",
    result.warnings[2].text.as_str()
  );

  result = fixture.lint("--!optimize   ", None);
  assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  assert_eq!(
    "optimize directive requires an optimization level",
    result.warnings[0].text.as_str()
  );
}

// 缺口（未移植，对照 `tests/Linter.test.cpp`，共 7 例）：
// - DeprecatedAttributeOnFunctionInCompositeTable /
//   DeprecatedAttributeOnFunctionInCompositeTable2 /
//   DeprecatedAttributeOnNestedCompositeFunctions（:1890,1925,1963）——依赖上游
//   FFlag `LuauImproveDeprecatedLint`（交集/并集/setmetatable 组合类型内成员的
//   deprecated 告警），本移植未 sync 该 flag 与对应判定，faithful 断言（4 条
//   warning）不可表达。
// - DuplicateConditionsIfLocalExcluded / DuplicateConditionsMixedWithIfLocal /
//   DuplicateConditionsIfLocalExpressionExcluded /
//   DuplicateConditionsMixedWithIfLocalExpression（:2523,2545,2569,2588）——依赖
//   `if local`/`if const` 语法（DebugLuauIfLocalSyntax / DebugLuauIfLocalAnalysis），
//   本移植 parser 未接入该语法（见 ulua-ast ast_expr_if_else.rs 注释），用例源码
//   无法解析。待上述功能落地后应补齐。
