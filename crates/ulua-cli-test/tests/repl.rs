//! `cpp/tests/Repl.test.cpp` 的移植：REPL 补全与输出测试。

use ulua_cli_test::{
  methods::repl_fixture_check_completion::repl_fixture_check_completion,
  records::repl_fixture::ReplFixture,
};

#[test]
fn repl_addition_statement() {
  let mut fixture = ReplFixture::new();
  fixture.run("return 30 + 12");

  assert_eq!("42", fixture.get_captured_output());
}

#[test]
fn repl_complete_global_variables() {
  let mut fixture = ReplFixture::new();
  fixture.run(
    r#"
      myvariable1 = 5
      myvariable2 = 5
"#,
  );

  {
    let completions = fixture.get_completion_set("myvar");
    let prefix = "";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "myvariable1"
    ));
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "myvariable2"
    ));
  }

  {
    let completions = fixture.get_completion_set("math.m");
    let prefix = "math.";
    assert_eq!(4, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "max("));
    assert!(repl_fixture_check_completion(&completions, prefix, "min("));
    assert!(repl_fixture_check_completion(&completions, prefix, "modf("));
    assert!(repl_fixture_check_completion(&completions, prefix, "map("));
  }
}

#[test]
fn repl_complete_table_keys() {
  let mut fixture = ReplFixture::new();
  fixture.run(
    r#"
      t = { color = "red", size = 1, shape = "circle" }
"#,
  );

  {
    let completions = fixture.get_completion_set("t.");
    let prefix = "t.";
    assert_eq!(3, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "color"));
    assert!(repl_fixture_check_completion(&completions, prefix, "size"));
    assert!(repl_fixture_check_completion(&completions, prefix, "shape"));
  }

  {
    let completions = fixture.get_completion_set("t.s");
    let prefix = "t.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "size"));
    assert!(repl_fixture_check_completion(&completions, prefix, "shape"));
  }
}

#[test]
fn repl_infinite_recursion() {
  let mut fixture = ReplFixture::new();
  fixture.run(
    r#"
local NewProxyOne = newproxy(true)
local MetaTableOne = getmetatable(NewProxyOne)
MetaTableOne.__index = function()
	return NewProxyOne.Game
end
print(NewProxyOne.HelloICauseACrash)
"#,
  );
  // cpp 同名用例（Repl.test.cpp:420）只防崩溃回归；此处加锁可观测输出：
  // __index 无限递归在求值 print 实参时即以 "C stack overflow" 失败（实测），
  // runCode 的错误文本不进 _PRETTYPRINT，capturedoutput 必须保持为空——
  // 若递归被「修好」成了可求值，print 结果反而会出现在这里
  assert_eq!("", fixture.get_captured_output());
}

#[test]
fn repl_interactive_stack_reserve1() {
  let mut fixture = ReplFixture::new();

  fixture.resume();

  fixture.run("\nlocal t = {}\n");
}

#[test]
fn repl_interactive_stack_reserve2() {
  let mut fixture = ReplFixture::new();

  fixture.resume();

  fixture.get_completion_set("a");
}

#[test]
fn repl_multiple_arguments() {
  let mut fixture = ReplFixture::new();
  fixture.run("return 3, 'three'");

  assert_eq!("3\t\"three\"", fixture.get_captured_output());
}

#[test]
fn repl_string_literal() {
  let mut fixture = ReplFixture::new();
  fixture.run("return 'str'");

  assert_eq!("\"str\"", fixture.get_captured_output());
}

#[test]
fn repl_string_methods() {
  let mut fixture = ReplFixture::new();
  fixture.run(
    r#"
      s = ""
"#,
  );

  {
    let completions = fixture.get_completion_set("s:l");
    let prefix = "s:";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "len("));
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "lower("
    ));
  }
}

#[test]
fn repl_table_literal() {
  let mut fixture = ReplFixture::new();
  fixture.run("return {1, 2, 3, 4}");

  assert_eq!("{1, 2, 3, 4}", fixture.get_captured_output());
}

#[test]
fn repl_table_with_deep_metatable_index_tables() {
  let mut fixture = ReplFixture::new();
  fixture.run(
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
  );

  {
    let completions = fixture.get_completion_set("t30.entry0");
    let prefix = "t30.";
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "entry0"
    ));
  }

  {
    let completions = fixture.get_completion_set("t30.entry0.co");
    let prefix = "t30.entry0.";
    assert!(repl_fixture_check_completion(&completions, prefix, "count"));
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

#[test]
fn repl_table_with_metatable_index_function() {
  let mut fixture = ReplFixture::new();
  fixture.run(
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
  );

  {
    let completions = fixture.get_completion_set("t.t");
    let prefix = "t.";
    assert_eq!(1, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "tkey"));
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

#[test]
fn repl_table_with_metatable_index_table() {
  let mut fixture = ReplFixture::new();
  fixture.run(
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
  );

  {
    let completions = fixture.get_completion_set("t.t");
    let prefix = "t.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "tkey1"));
    assert!(repl_fixture_check_completion(&completions, prefix, "tkey2"));
  }

  {
    let completions = fixture.get_completion_set("t.tkey1.data2:re");
    let prefix = "t.tkey1.data2:";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "rep("));
    assert!(repl_fixture_check_completion(
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
      &completions,
      prefix,
      "mtkey1"
    ));
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "mtkey2"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.mtkey1.");
    let prefix = "t.mtkey1.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "x"));
    assert!(repl_fixture_check_completion(&completions, prefix, "y"));
  }
}

#[test]
fn repl_table_with_multiple_metatable_index_tables() {
  let mut fixture = ReplFixture::new();
  fixture.run(
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
  );

  {
    let completions = fixture.get_completion_set("t.");
    let prefix = "t.";
    assert_eq!(4, completions.len());
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "__index"
    ));
    assert!(repl_fixture_check_completion(&completions, prefix, "tkey"));
    assert!(repl_fixture_check_completion(&completions, prefix, "mtkey"));
    assert!(repl_fixture_check_completion(
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
      &completions,
      prefix,
      "__index"
    ));
    assert!(repl_fixture_check_completion(&completions, prefix, "mtkey"));
    assert!(repl_fixture_check_completion(
      &completions,
      prefix,
      "mt2key"
    ));
  }

  {
    let completions = fixture.get_completion_set("t.mt2key.");
    let prefix = "t.mt2key.";
    assert_eq!(2, completions.len());
    assert!(repl_fixture_check_completion(&completions, prefix, "x"));
    assert!(repl_fixture_check_completion(&completions, prefix, "y"));
  }
}

#[test]
fn repl_table_with_string_literals() {
  let mut fixture = ReplFixture::new();
  fixture.run("return {1, 'two', 3, 'four'}");

  assert_eq!("{1, \"two\", 3, \"four\"}", fixture.get_captured_output());
}
