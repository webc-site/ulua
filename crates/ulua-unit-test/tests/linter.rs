extern crate alloc;

mod linter_break_from_infinite_loop_makes_statement_reachable {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:899:linter_break_from_infinite_loop_makes_statement_reachable`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_break_from_infinite_loop_makes_statement_reachable
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_break_from_infinite_loop_makes_statement_reachable() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_builtin_global_write {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:103:linter_builtin_global_write`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item linter_builtin_global_write
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_builtin_global_write() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
      &String::from(
        r#"
math = {}

function assert(x)
end

assert(5)
"#,
      ),
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
}

mod linter_clean_code {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:16:linter_clean_code`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_clean_code
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_clean_code() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function fib(n)
    return n < 2 and 1 or fib(n-1) + fib(n-2)
end

"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_comparison_precedence {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2488:linter_comparison_precedence`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_comparison_precedence
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_comparison_precedence() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_confusing_indentation {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:159:linter_confusing_indentation`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item linter_confusing_indentation
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_confusing_indentation() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
print(math.max(1,
2))
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Statement spans multiple lines; use indentation to silence",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_dead_locals_used {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1365:linter_dead_locals_used`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_dead_locals_used
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_dead_locals_used() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_deprecated_api_fenv {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1590:linter_deprecated_api_fenv`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_deprecated_api_fenv
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_api_fenv() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
      &String::from(
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
      ),
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
}

mod linter_deprecated_api_typed {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1517:linter_deprecated_api_typed`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
  //!   - calls -> function foreach (VM/src/ltablib.cpp)
  //!   - calls -> function getn (VM/src/ltablib.cpp)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item linter_deprecated_api_typed
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_api_typed() {
    use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
        freeze::freeze, get_global_binding::get_global_binding,
        get_mutable_type::get_mutable_type_id, persist_type::persist, unfreeze::unfreeze,
      },
      records::{
        binding::Binding, extern_type::ExternType, property_type::Property, scope::Scope,
        table_type::TableType, type_fun::TypeFun, type_level::TypeLevel,
      },
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    {
      let frontend = fixture.get_frontend();
      unfreeze(frontend.globals.global_types_mut());

      let builtins = unsafe { &mut *frontend.builtin_types };
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
        definition_module_name: String::from("Test"),
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

      if let Some(color_table) = get_mutable_type_id::<TableType>(color_type) {
        let mut to_hsv = Property::rw_type_id(any_type);
        to_hsv.deprecated = true;
        to_hsv.deprecated_suggestion = String::from("Color3:ToHSV");
        color_table.props.insert(String::from("toHSV"), to_hsv);
      }

      add_global_binding_builtin_definitions_alt_b(
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
      if let Some(table) = get_mutable_type_id::<TableType>(table_type) {
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
        &String::from(
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
"#,
        ),
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
}

mod linter_deprecated_api_untyped {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1567:linter_deprecated_api_untyped`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
  //!   - calls -> function foreach (VM/src/ltablib.cpp)
  //!   - calls -> function getn (VM/src/ltablib.cpp)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_deprecated_api_untyped
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_api_untyped() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_global_binding::get_global_binding, get_mutable_type::get_mutable_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    {
      let frontend = fixture.get_frontend();
      let table_type = get_global_binding(&mut frontend.globals, "table");
      if let Some(table) = get_mutable_type_id::<TableType>(table_type) {
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
      &String::from(
        r#"
-- TODO
return function ()
    print(table.getn({}))
    table.foreach({}, function() end)
    print(table.nogetn()) -- verify that we correctly handle non-existent members
end
"#,
      ),
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
}

mod linter_deprecated_attribute {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1624:linter_deprecated_attribute`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function checkDeprecatedWarning (tests/Linter.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item linter_deprecated_attribute
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_attribute() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::check_deprecated_warning::check_deprecated_warning, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    {
      let result = fixture.lint(
        &String::from(
          r#"
@deprecated
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@deprecated
function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@deprecated
local function testfun(x:number):number
    return x + 1
end

if math.random(2) == 2 then
    testfun(1)
end
"#,
        ),
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
        &String::from(
          r#"
@deprecated
local function testfun(x:number)
    return x + 1
end

g(testfun)
"#,
        ),
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
        &String::from(
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
        ),
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
        &String::from(
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
        ),
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
        &String::from(
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
        ),
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
        &String::from(
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
        ),
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
        &String::from(
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
        ),
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
        &String::from(
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
        ),
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
        &String::from(
          r#"
Account = { balance=0 }

@deprecated
function Account:deposit(v)
    self.balance = self.balance + v
end

Account:deposit(200.00)
"#,
        ),
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
        &String::from(
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
        ),
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
}

mod linter_deprecated_attribute_function_declaration {
  use super::*;
  #[cfg(test)]
  #[test]
  fn linter_deprecated_attribute_function_declaration() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::check_deprecated_warning::check_deprecated_warning, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
@deprecated declare function bar(x: number): string
"#,
      ),
      false,
    );

    let result = fixture.lint(
      &String::from(
        r#"
bar(2)
"#,
      ),
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
}

mod linter_deprecated_attribute_method_declaration {
  use super::*;
  #[cfg(test)]
  #[test]
  fn linter_deprecated_attribute_method_declaration() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::check_deprecated_warning::check_deprecated_warning, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
declare extern type Foo with
   @deprecated
   function bar(self, value: number) : number
end

declare Foo: {
   new: () -> Foo
}
"#,
      ),
      false,
    );

    let result = fixture.lint(
      &String::from(
        r#"
local foo = Foo.new()
print(foo:bar(2.0))
"#,
      ),
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
}

mod linter_deprecated_attribute_table_declaration {
  use super::*;
  #[cfg(test)]
  #[test]
  fn linter_deprecated_attribute_table_declaration() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::check_deprecated_warning::check_deprecated_warning, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
declare Hooty : {
    tooty : @deprecated @checked (number) -> number
}
"#,
      ),
      false,
    );

    let result = fixture.lint(
      &String::from(
        r#"
print(Hooty:tooty(2.0))
"#,
      ),
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
}

mod linter_deprecated_attribute_with_params {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1873:linter_deprecated_attribute_with_params`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function checkDeprecatedWarning (tests/Linter.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item linter_deprecated_attribute_with_params
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_attribute_with_params() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::check_deprecated_warning::check_deprecated_warning, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);

    {
      let result = fixture.lint(
        &String::from(
          r#"
@[deprecated{ use = "prodfun", reason = "Too old." }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@[deprecated{ use = "prodfun", reason = "Too old." }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@[deprecated{ use = "prodfun" }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@[deprecated{ use = "prodfun" }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@[deprecated{ reason = "Too old." }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
@[deprecated{ reason = "Too old." }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
        ),
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
        &String::from(
          r#"
Account = { balance=0 }

@[deprecated{use = 'credit', reason = 'It sounds cool'}]
function Account:deposit(v)
    self.balance = self.balance + v
end

Account:deposit(200.00)
"#,
        ),
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
        &String::from(
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
        ),
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
        &String::from(
          r#"
@[deprecated{use = 'foo', reason = 'Do better.'}] declare function bar(x: number): string
"#,
        ),
        false,
      );

      let result = fixture.lint(
        &String::from(
          r#"
bar(2)
"#,
        ),
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
        &String::from(
          r#"
declare Hooty : {
    tooty : @[deprecated{use = 'foo', reason = 'bar'}] @checked (number) -> number
}
"#,
        ),
        false,
      );

      let result = fixture.lint(
        &String::from(
          r#"
print(Hooty:tooty(2.0))
"#,
        ),
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
        &String::from(
          r#"
declare extern type Foo with
   @[deprecated{use = 'foo', reason = 'baz'}]
   function bar(self, value: number) : number
end

declare Foo: {
   new: () -> Foo
}
"#,
        ),
        false,
      );

      let result = fixture.lint(
        &String::from(
          r#"
local foo = Foo.new()
print(foo:bar(2.0))
"#,
        ),
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
}

mod linter_deprecated_global {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:48:linter_deprecated_global`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_deprecated_global
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_global() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
      records::binding::Binding,
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().any_type;
    add_global_binding_builtin_definitions_alt_b(
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

    let result = fixture.lint(&String::from("Wait(5)"), None);

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Global 'Wait' is deprecated, use 'wait' instead",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_deprecated_global_no_replacement {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:59:linter_deprecated_global_no_replacement`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_deprecated_global_no_replacement
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_deprecated_global_no_replacement() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
      records::binding::Binding,
    };
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().any_type;
    add_global_binding_builtin_definitions_alt_b(
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

    let result = fixture.lint(&String::from("Version()"), None);

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Global 'Version' is deprecated",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_disable_unknown_global_with_type_checking {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1315:linter_disable_unknown_global_with_type_checking`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_disable_unknown_global_with_type_checking
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_disable_unknown_global_with_type_checking() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        --!strict
        unknownGlobal()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1466:linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        if true then
            function c() end
        else
            function c() end
        end

        return c
    "#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_duplicate_conditions {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2237:linter_duplicate_conditions`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_duplicate_conditions
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_conditions() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
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
"#,
        ),
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
}

mod linter_duplicate_conditions_expr {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2277:linter_duplicate_conditions_expr`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_duplicate_conditions_expr
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_conditions_expr() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local correct, opaque = ...

if correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", `string {opaque}`)}) then
elseif correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", `string {opaque}`)}) then
elseif correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", false)}) then
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Condition has already been checked on line 4",
      result.warnings[0].text.as_str()
    );
    assert_eq!(5, result.warnings[0].location.begin.line + 1);
  }
}

mod linter_duplicate_conditions_if_stat_and_expr {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2375:linter_duplicate_conditions_if_stat_and_expr`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_duplicate_conditions_if_stat_and_expr
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_conditions_if_stat_and_expr() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
if if 1 then 2 else 3 then
elseif if 1 then 2 else 3 then
elseif if 0 then 5 else 4 then
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Condition has already been checked on line 2",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_duplicate_global_function {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1404:linter_duplicate_global_function`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_duplicate_global_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_global_function() {
    use alloc::string::String;

    use ulua_config::enums::code::Code;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        function x() end

        function x() end

        return x
    "#,
      ),
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
}

mod linter_duplicate_local {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2293:linter_duplicate_local`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_duplicate_local
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_local() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_duplicate_local_function {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1422:linter_duplicate_local_function`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintOptions (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method LintOptions::setDefaults (Config/src/LinterConfig.cpp)
  //!   - calls -> method LintOptions::enableWarning (Config/include/Luau/LinterConfig.h)
  //!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_duplicate_local_function
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_local_function() {
    use alloc::string::String;

    use ulua_config::{enums::code::Code, records::lint_options::LintOptions};
    use ulua_unit_test::records::fixture::Fixture;

    let mut options = LintOptions::default();
    options.set_defaults();
    options.enable_warning(Code::DuplicateFunction);
    options.enable_warning(Code::LocalShadow);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        local function x() end

        print(x)

        local function x() end

        return x
    "#,
      ),
      Some(options),
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(Code::DuplicateFunction, result.warnings[0].code);
  }
}

mod linter_duplicate_method {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1447:linter_duplicate_method`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_duplicate_method
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_duplicate_method() {
    use alloc::string::String;

    use ulua_config::enums::code::Code;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        local T = {}
        function T:x() end

        function T:x() end

        return x
    "#,
      ),
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
}

mod linter_for_range_backwards {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:667:linter_for_range_backwards`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_for_range_backwards
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_for_range_backwards() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
for i=8,1 do
end

for i=8,1,-1 do
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(1, result.warnings[0].location.begin.line);
    assert_eq!(
      "For loop should iterate backwards; did you forget to specify -1 as step?",
      result.warnings[0].text
    );
  }
}

mod linter_for_range_imprecise {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:682:linter_for_range_imprecise`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_for_range_imprecise
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_for_range_imprecise() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
for i=1.3,7.5 do
end

for i=1.3,7.5,1 do
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(1, result.warnings[0].location.begin.line);
    assert_eq!(
      "For loop ends at 7.3 instead of 7.5; did you forget to specify step?",
      result.warnings[0].text
    );
  }
}

mod linter_for_range_table {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:650:linter_for_range_table`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_for_range_table
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_for_range_table() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local t = {}

for i=#t,1 do
end

for i=#t,1,-1 do
end
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(3, result.warnings[0].location.begin.line);
    assert_eq!(
      "For loop should iterate backwards; did you forget to specify -1 as step?",
      result.warnings[0].text
    );
  }
}

mod linter_for_range_zero {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:697:linter_for_range_zero`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_for_range_zero
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_for_range_zero() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
for i=0,#t do
end

for i=(0),#t do -- to silence
end

for i=#t,0 do
end
"#,
      ),
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
}

mod linter_format_string_date {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1170:linter_format_string_date`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item linter_format_string_date
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_date() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_find_args {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1119:linter_format_string_find_args`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> method TxnLog::inverse (Analysis/src/TxnLog.cpp)
  //!   - translates_to -> rust_item linter_format_string_find_args
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_find_args() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_format {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:940:linter_format_string_format`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_format_string_format
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_format() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_match {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1009:linter_format_string_match`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> function gmatch (VM/src/lstrlib.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method TxnLog::inverse (Analysis/src/TxnLog.cpp)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item linter_format_string_match
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_match() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_match_nested {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1056:linter_format_string_match_nested`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item linter_format_string_match_nested
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_match_nested() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local s = ...

-- correct reference to nested pattern
string.match(s, "((a)%2)")

-- incorrect reference to nested pattern (not closed yet)
string.match(s, "((a)%1)")

-- incorrect reference to nested pattern (index out of range)
string.match(s, "((a)%3)")
"#,
      ),
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
}

mod linter_format_string_match_sets {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1078:linter_format_string_match_sets`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> method TxnLog::inverse (Analysis/src/TxnLog.cpp)
  //!   - translates_to -> rust_item linter_format_string_match_sets
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_match_sets() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_pack {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:962:linter_format_string_pack`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_format_string_pack
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_pack() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_replace {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1147:linter_format_string_replace`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - calls -> function digit (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item linter_format_string_replace
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_replace() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_format_string_typed {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1191:linter_format_string_typed`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item linter_format_string_typed
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_format_string_typed() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local s: string, nons = ...

string.match(s, "[]")
s:match("[]")

-- no warning here since we don't know that it's a string
nons:match("[]")
"#,
      ),
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
}

mod linter_function_unused {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:401:linter_function_unused`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_function_unused
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_function_unused() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_global_as_local {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:170:linter_global_as_local`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_global_as_local
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function bar()
    foo = 6
    return foo
end

return bar()
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Global 'foo' is only used in the enclosing function 'bar'; consider changing it to local",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_global_as_local_3_with_conditional_read {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:247:linter_global_as_local_3_with_conditional_read`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_global_as_local_3_with_conditional_read
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local3_with_conditional_read() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_global_as_local_inner_read {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:270:linter_global_as_local_inner_read`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_global_as_local_inner_read
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local_inner_read() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function foo()
   local f = function() return bar end
   f()
   bar = 42
end

function baz() bar = 0 end

return foo() + baz()
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_global_as_local_multi {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:287:linter_global_as_local_multi`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function createFunction (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_global_as_local_multi
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local_multi() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Global 'moreInternalLogic' is only used in the enclosing function defined at line 2; consider changing it to local",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_global_as_local_multi_fx {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:185:linter_global_as_local_multi_fx`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_global_as_local_multi_fx
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local_multi_fx() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Global 'foo' is never read before being written. Consider changing it to local",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_global_as_local_multi_fx_with_read {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:205:linter_global_as_local_multi_fx_with_read`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_global_as_local_multi_fx_with_read
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local_multi_fx_with_read() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_global_as_local_with_conditional {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:228:linter_global_as_local_with_conditional`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_global_as_local_with_conditional
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_global_as_local_with_conditional() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_ignore_lint_all {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:918:linter_ignore_lint_all`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_ignore_lint_all
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_ignore_lint_all() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
--!nolint
return foo
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_ignore_lint_specific {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:928:linter_ignore_lint_specific`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_ignore_lint_specific
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_ignore_lint_specific() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
--!nolint UnknownGlobal
local x = 1
return foo
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Variable 'x' is never used; prefix with '_' to silence",
      result.warnings[0].text
    );
  }
}

mod linter_implicit_return {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:747:linter_implicit_return`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_implicit_return
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_implicit_return() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_implicit_return_infinite_loop {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:824:linter_implicit_return_infinite_loop`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_implicit_return_infinite_loop
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_implicit_return_infinite_loop() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_import_only_used_in_return_type {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1302:linter_import_only_used_in_return_type`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_import_only_used_in_return_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_import_only_used_in_return_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        local Foo = require(script.Parent.Foo)

        function foo(): Foo.Y
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Function 'foo' is never used; prefix with '_' to silence",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_import_only_used_in_type_annotation {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1290:linter_import_only_used_in_type_annotation`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_import_only_used_in_type_annotation
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_import_only_used_in_type_annotation() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        local Foo = require(script.Parent.Foo)

        local x: Foo.Y = 1
    "#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Variable 'x' is never used; prefix with '_' to silence",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_import_unused {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:387:linter_import_unused`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_import_unused
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_import_unused() {
    use alloc::string::String;

    use ulua_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().any_type;
    add_global_binding_builtin_definitions(
      &mut fixture.get_frontend().globals,
      "game",
      any_type,
      "@test",
    );

    let result = fixture.lint(
      &String::from(
        r#"
local Roact = require(game.Packages.Roact)
local _Roact = require(game.Packages.Roact)
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Import 'Roact' is never used; prefix with '_' to silence",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_integer_parsing {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2417:linter_integer_parsing`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_integer_parsing
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_integer_parsing() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local _ = 0b10000000000000000000000000000000000000000000000000000000000000000
local _ = 0x10000000000000000
"#,
      ),
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
}

mod linter_integer_parsing_decimal_imprecise {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2429:linter_integer_parsing_decimal_imprecise`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_integer_parsing_decimal_imprecise
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_integer_parsing_decimal_imprecise() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_integer_parsing_hex_imprecise {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2465:linter_integer_parsing_hex_imprecise`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_integer_parsing_hex_imprecise
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_integer_parsing_hex_imprecise() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_lint_hygiene_uaf {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1481:linter_lint_hygiene_uaf`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_lint_hygiene_uaf
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_lint_hygiene_uaf() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(12, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_local_function_not_dead {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1394:linter_local_function_not_dead`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_local_function_not_dead
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_local_function_not_dead() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local foo
function foo() end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_local_shadow_argument {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:351:linter_local_shadow_argument`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_local_shadow_argument
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_local_shadow_argument() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function bar(a, b)
    local a = b + 1
    return a
end

return bar()
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Variable 'a' shadows previous declaration at line 2",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_local_shadow_global {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:333:linter_local_shadow_global`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_local_shadow_global
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_local_shadow_global() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
      &String::from(
        r#"
local math = math
global = math

function bar()
    local global = math.max(5, 1)
    return global
end

return bar()
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Variable 'global' shadows a global variable used at line 3",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_local_shadow_local {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:319:linter_local_shadow_local`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_local_shadow_local
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_local_shadow_local() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local arg = 6
print(arg)

local arg = 5
print(arg)
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Variable 'arg' shadows previous declaration at line 2",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_local_unused {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:366:linter_local_unused`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
  //!   - translates_to -> rust_item linter_local_unused
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_local_unused() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_misleading_and_or {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2316:linter_misleading_and_or`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item linter_misleading_and_or
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_misleading_and_or() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
_ = math.random() < 0.5 and true or 42
_ = math.random() < 0.5 and false or 42 -- misleading
_ = math.random() < 0.5 and nil or 42 -- misleading
_ = math.random() < 0.5 and 0 or 42
_ = (math.random() < 0.5 and false) or 42 -- currently ignored
"#,
      ),
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
}

mod linter_multiline_block {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:119:linter_multiline_block`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_multiline_block
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_multiline_block() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
if true then print(1) print(2) print(3) end
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "A new statement is on the same line; add semi-colon on previous statement to silence",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_multiline_block_local_do {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:148:linter_multiline_block_local_do`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_multiline_block_local_do
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_multiline_block_local_do() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local _x do
    _x = 5
end
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_multiline_block_missed_semicolon {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:138:linter_multiline_block_missed_semicolon`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_multiline_block_missed_semicolon
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_multiline_block_missed_semicolon() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
print(1); print(2) print(3)
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "A new statement is on the same line; add semi-colon on previous statement to silence",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_multiline_block_semicolons_whitelisted {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:129:linter_multiline_block_semicolons_whitelisted`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_multiline_block_semicolons_whitelisted
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_multiline_block_semicolons_whitelisted() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
print(1); print(2); print(3)
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_no_spurious_warning_after_a_function_type_alias {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1325:linter_no_spurious_warning_after_a_function_type_alias`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_no_spurious_warning_after_a_function_type_alias
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_no_spurious_warning_after_a_function_type_alias() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        local exports = {}
        export type PathFunction<P> = (P?) -> string
        exports.tokensToFunction = function() end
        return exports
    "#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_placeholder_read {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:71:linter_placeholder_read`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_placeholder_read
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_placeholder_read() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local _ = 5
return _
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Placeholder value '_' is read here; consider using a named variable",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_placeholder_read_global {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:82:linter_placeholder_read_global`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_placeholder_read_global
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_placeholder_read_global() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
_ = 5
print(_)
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Placeholder value '_' is read here; consider using a named variable",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_placeholder_write {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:93:linter_placeholder_write`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_placeholder_write
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_placeholder_write() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local _ = 5
_ = 6
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_read_write_table_props {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1266:linter_read_write_table_props`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item linter_read_write_table_props
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_read_write_table_props() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_redundant_native_attribute {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2520:linter_redundant_native_attribute`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item linter_redundant_native_attribute
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_redundant_native_attribute() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_table_literal {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1210:linter_table_literal`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item linter_table_literal
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_table_literal() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_table_operations {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2124:linter_table_operations`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_table_operations
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_table_operations() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
      &String::from(
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
      ),
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
}

mod linter_table_operations_indexer {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2179:linter_table_operations_indexer`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item linter_table_operations_indexer
  use ulua_common::FFlag;

  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_table_operations_indexer() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
        &String::from(
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
"#,
        ),
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
}

mod linter_test_string_interpolation {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2407:linter_test_string_interpolation`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item linter_test_string_interpolation
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_test_string_interpolation() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
        --!nocheck
        local _ = `unknown {foo}`
    "#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_type_annotations_should_not_produce_warnings {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:882:linter_type_annotations_should_not_produce_warnings`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item linter_type_annotations_should_not_produce_warnings
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_type_annotations_should_not_produce_warnings() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_type_function_fully_reduces {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:28:linter_type_function_fully_reduces`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_type_function_fully_reduces
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_type_function_fully_reduces() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function fib(n)
    return n < 2 or  fib(n-2)
end

"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_type_instantiation_lints {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2546:linter_type_instantiation_lints`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item linter_type_instantiation_lints
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_type_instantiation_lints() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
local function a<b>(cool: b)
    print(cool)
end

a<<"hi">>("hi")
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_unbalanced_assignment {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:720:linter_unbalanced_assignment`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_unbalanced_assignment
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unbalanced_assignment() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_unknown_global {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:40:linter_unknown_global`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item linter_unknown_global
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unknown_global() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(&String::from("--!nocheck\nreturn foo"), None);

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "Unknown global 'foo'; consider assigning to it first",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unknown_type {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:619:linter_unknown_type`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item linter_unknown_type
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unknown_type() {
    use alloc::{string::String, sync::Arc, vec::Vec};

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::unfreeze::unfreeze,
      records::{
        property_type::Property, scope::Scope, table_type::TableType, type_fun::TypeFun,
        type_level::TypeLevel,
      },
      type_aliases::props_type::Props,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      &String::from(
        r#"
local game = ...
local _e01 = type(game) == "Part"
local _e02 = typeof(game) == "Bar"
local _ok = typeof(game) == "vector"

local _o01 = type(game) == "number"
local _o02 = type(game) == "vector"
local _o03 = typeof(game) == "Part"
"#,
      ),
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
}

mod linter_unreachable_code_assert_false_return_silent {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:528:linter_unreachable_code_assert_false_return_silent`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_assert_false_return_silent
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_assert_false_return_silent() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function foo1(a)
    if a then
        return 'z'
    end

    assert(false)
end

return foo1
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_unreachable_code_basic {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:427:linter_unreachable_code_basic`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_basic
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_basic() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
do
return 'ok'
end

print("hi!")
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(5, result.warnings[0].location.begin.line);
    assert_eq!(
      "Unreachable code (previous statement always returns)",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unreachable_code_error_return_non_silent_branchy {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:545:linter_unreachable_code_error_return_non_silent_branchy`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_error_return_non_silent_branchy
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_error_return_non_silent_branchy() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(7, result.warnings[0].location.begin.line);
    assert_eq!(
      "Unreachable code (previous statement always errors)",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unreachable_code_error_return_propagate {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:565:linter_unreachable_code_error_return_propagate`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_error_return_propagate
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_error_return_propagate() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(8, result.warnings[0].location.begin.line);
    assert_eq!(
      "Unreachable code (previous statement always errors)",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unreachable_code_error_return_silent {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:510:linter_unreachable_code_error_return_silent`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_error_return_silent
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_error_return_silent() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_unreachable_code_if_merge {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:474:linter_unreachable_code_if_merge`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_if_merge
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_if_merge() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(7, result.warnings[0].location.begin.line);
    assert_eq!(
      "Unreachable code (previous statement always returns)",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unreachable_code_loop_break {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:442:linter_unreachable_code_loop_break`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_loop_break
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_loop_break() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
while true do
    do break end
    print("nope")
end

print("hi!")
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(3, result.warnings[0].location.begin.line);
    assert_eq!(
      "Unreachable code (previous statement always breaks)",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unreachable_code_loop_continue {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:458:linter_unreachable_code_loop_continue`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_loop_continue
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_loop_continue() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
while true do
    do continue end
    print("nope")
end

print("hi!")
"#,
      ),
      None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(3, result.warnings[0].location.begin.line);
    assert_eq!(
      "Unreachable code (previous statement always continues)",
      result.warnings[0].text.as_str()
    );
  }
}

mod linter_unreachable_code_loop_repeat {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:602:linter_unreachable_code_loop_repeat`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_loop_repeat
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_loop_repeat() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function foo1(a)
    repeat
        return 'z'
    until a
    return 'x'
end

return foo1
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_unreachable_code_loop_while {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:586:linter_unreachable_code_loop_while`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_unreachable_code_loop_while
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_unreachable_code_loop_while() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
function foo1(a)
    while a do
        return 'z'
    end
    return 'x'
end

return foo1
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_use_all_parent_scopes_for_globals {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1337:linter_use_all_parent_scopes_for_globals`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - calls -> method Frontend::addEnvironment (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_use_all_parent_scopes_for_globals
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_use_all_parent_scopes_for_globals() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{freeze::freeze, unfreeze::unfreeze},
      records::frontend::Frontend,
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let test_scope = fixture.get_frontend().add_environment(String::from("Test"));
    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    unsafe {
      unfreeze((*frontend_ptr).globals.global_types_mut());
      let result = (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        test_scope,
        r#"
        declare Foo: number
    "#,
        String::from("@test"),
        false,
        false,
      );
      assert!(result.success, "{:?}", result);
      freeze((*frontend_ptr).globals.global_types_mut());
    }

    fixture
      .base
      .file_resolver
      .environments
      .insert(String::from("A"), String::from("Test"));
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

    let result = fixture.base.lint_module(&String::from("A"), None);

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_wrong_comment {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2339:linter_wrong_comment`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record Comment (Ast/include/Luau/ParseResult.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item linter_wrong_comment
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_wrong_comment() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
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
      ),
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
}

mod linter_wrong_comment_mute_self {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2365:linter_wrong_comment_mute_self`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_wrong_comment_mute_self
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_wrong_comment_mute_self() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
      &String::from(
        r#"
--!nolint
--!struct
"#,
      ),
      None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
  }
}

mod linter_wrong_comment_optimize {
  //! Ported from `tests/Linter.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2388:linter_wrong_comment_optimize`
  //! Source: `tests/Linter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Linter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Linter.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Linter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - translates_to -> rust_item linter_wrong_comment_optimize
  use super::*;

  #[cfg(test)]
  #[test]
  fn linter_wrong_comment_optimize() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut result = fixture.lint(
      &String::from(
        r#"
--!optimize
--!optimize me
--!optimize 100500
--!optimize 2
"#,
      ),
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

    result = fixture.lint(&String::from("--!optimize   "), None);
    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
      "optimize directive requires an optimization level",
      result.warnings[0].text.as_str()
    );
  }
}
