use alloc::string::String;
use core::ptr::null_mut;

use ulua_vm::functions::lua_resume::lua_resume;
extern crate alloc;
// Port of `cpp/tests/Repl.test.cpp`.
// Repl tests.

#[cfg(test)]
#[test]
fn repl_addition_statement() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(fixture.l as *mut _, &String::from("return 30 + 12"));
  }

  assert_eq!("42", fixture.get_captured_output());
}

#[cfg(test)]
#[test]
fn repl_complete_global_variables() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
          myvariable1 = 5
          myvariable2 = 5
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("myvar");
    let prefix = "";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "myvariable1"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "myvariable2"
    ));
  }

  {
    let completions = fixture.get_completion_set("math.m");
    let prefix = "math.";
    assert_eq!(4, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "max("
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "min("
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "modf("
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "map("
    ));
  }
}

#[cfg(test)]
#[test]
fn repl_complete_table_keys() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
          t = { color = "red", size = 1, shape = "circle" }
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("t.");
    let prefix = "t.";
    assert_eq!(3, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "color"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "size"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "shape"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.s");
    let prefix = "t.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "size"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "shape"
    ));
  }
}

#[cfg(test)]
#[test]
fn repl_infinite_recursion() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
local NewProxyOne = newproxy(true)
local MetaTableOne = getmetatable(NewProxyOne)
MetaTableOne.__index = function()
	return NewProxyOne.Game
end
print(NewProxyOne.HelloICauseACrash)
"#,
      ),
    );
  }
}

#[cfg(test)]
#[test]
fn repl_interactive_stack_reserve1() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let fixture = ReplFixture::new();

  unsafe {
    lua_resume(fixture.l as *mut _, null_mut(), 0);
  }

  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
local t = {}
"#,
      ),
    );
  }
}

#[cfg(test)]
#[test]
fn repl_interactive_stack_reserve2() {
  use ulua_cli_test::records::repl_fixture::ReplFixture;

  let mut fixture = ReplFixture::new();

  unsafe {
    lua_resume(fixture.l as *mut _, null_mut(), 0);
  }

  fixture.get_completion_set("a");
}

#[cfg(test)]
#[test]
fn repl_multiple_arguments() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(fixture.l as *mut _, &String::from("return 3, 'three'"));
  }

  assert_eq!("3\t\"three\"", fixture.get_captured_output());
}

#[cfg(test)]
#[test]
fn repl_string_literal() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(fixture.l as *mut _, &String::from("return 'str'"));
  }

  assert_eq!("\"str\"", fixture.get_captured_output());
}

#[cfg(test)]
#[test]
fn repl_string_methods() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
          s = ""
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("s:l");
    let prefix = "s:";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "len("
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "lower("
    ));
  }
}

#[cfg(test)]
#[test]
fn repl_table_literal() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(fixture.l as *mut _, &String::from("return {1, 2, 3, 4}"));
  }

  assert_eq!("{1, 2, 3, 4}", fixture.get_captured_output());
}

#[cfg(test)]
#[test]
fn repl_table_with_deep_metatable_index_tables() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
-- Creates a table with a chain of metatables of length `count`
function makeChainedTable(count)
    local result = {}
    result.__index = result
    result[string.format("entry%d", count)] = { count = count }
    if count == 0 then
        return result
    else
        return setmetatable(result, makeChainedTable(count - 1))
    end
end

t30 = makeChainedTable(30)
t60 = makeChainedTable(60)
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("t30.entry0");
    let prefix = "t30.";
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "entry0"
    ));
  }

  {
    let completions = fixture.get_completion_set("t30.entry0.co");
    let prefix = "t30.entry0.";
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "count"
    ));
  }

  {
    let completions = fixture.get_completion_set("t60.entry0");
    assert_eq!(0, completions.len());
  }

  {
    let completions = fixture.get_completion_set("t60.entry0.co");
    assert_eq!(0, completions.len());
  }
}

#[cfg(test)]
#[test]
fn repl_table_with_metatable_index_function() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
        -- Create 't' which is a table with a metatable with an __index function
        mt = {}
        mt.__index = function(table, key)
            print("mt.__index called")
            if key == "foo" then
                return "FOO"
            elseif key == "bar" then
                return "BAR"
            else
                return nil
            end
        end

        t = {}
        setmetatable(t, mt)
        t.tkey = 0
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("t.t");
    let prefix = "t.";
    assert_eq!(1, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "tkey"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.foo");
    assert_eq!(0, completions.len());
  }

  {
    let completions = fixture.get_completion_set("t.foo:");
    assert_eq!(0, completions.len());
  }
}

#[cfg(test)]
#[test]
fn repl_table_with_metatable_index_table() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
        -- Create 't' which is a table with a metatable with an __index table
        mt = {}
        mt.__index = mt

        t = {}
        setmetatable(t, mt)

        mt.mtkey1 = {x="x value", y="y value", 1, 2}
        mt.mtkey2 = 2

        t.tkey1 = {data1 = 2, data2 = "str", 3, 4}
        t.tkey2 = 4
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("t.t");
    let prefix = "t.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "tkey1"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "tkey2"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.tkey1.data2:re");
    let prefix = "t.tkey1.data2:";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "rep("
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "reverse("
    ));
  }

  {
    let completions = fixture.get_completion_set("t.mtk");
    let prefix = "t.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "mtkey1"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "mtkey2"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.mtkey1.");
    let prefix = "t.mtkey1.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "x"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "y"
    ));
  }
}

#[cfg(test)]
#[test]
fn repl_table_with_multiple_metatable_index_tables() {
  use ulua_cli_test::{
    functions::run_code::run_code,
    methods::repl_fixture_check_completion::repl_fixture_check_completion,
    records::repl_fixture::ReplFixture,
  };

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from(
        r#"
        -- Create a table with a chain of metatables
        mt2 = {}
        mt2.__index = mt2

        mt = {}
        mt.__index = mt
        setmetatable(mt, mt2)

        t = {}
        setmetatable(t, mt)

        mt2.mt2key = {x=1, y=2}
        mt.mtkey = 2
        t.tkey = 3
"#,
      ),
    );
  }

  {
    let completions = fixture.get_completion_set("t.");
    let prefix = "t.";
    assert_eq!(4, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "__index"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "tkey"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "mtkey"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "mt2key"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.__index.");
    let prefix = "t.__index.";
    assert_eq!(3, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "__index"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "mtkey"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "mt2key"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.mt2key.");
    let prefix = "t.mt2key.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "x"
    ));
    assert!(repl_fixture_check_completion(
      &fixture,
      &completions,
      prefix,
      "y"
    ));
  }
}

#[cfg(test)]
#[test]
fn repl_table_with_string_literals() {
  use ulua_cli_test::{functions::run_code::run_code, records::repl_fixture::ReplFixture};

  let mut fixture = ReplFixture::new();
  unsafe {
    run_code(
      fixture.l as *mut _,
      &String::from("return {1, 'two', 3, 'four'}"),
    );
  }

  assert_eq!("{1, \"two\", 3, \"four\"}", fixture.get_captured_output());
}
