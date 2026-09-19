//! Port of `cpp/tests/Parser.test.cpp`（聚合套件 part4）。
//!
//! 语义以 C++ 原版逐条对应；AST 下转统一走 `functions::ast_node_ref` 的
//! `NodePtr::as_node` / `PtrRef::as_ref_opt` / `as_node_at` / `deref_at` / `elem`，
//! 调用点零 `unsafe`、零裸指针。
//!
//! C++ 侧 `try { parse(...) ; FAIL(...) } catch (const ParseErrors& e)` 的抛错语义在
//! Rust 侧统一由 `try_parse` + `errors` 断言承担（`Fixture::parse` 遇错直接 panic，
//! 与 `FAIL` 等价），`matchParseError` 则对应 `match_parse_error`。

/// `parse_error_confusing_function_call`：四处共用的歧义调用提示。
const AMBIGUOUS_ARGUMENT_LIST: &str = "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements";

/// `parse_interpolated_string_double_brace_*`：双花括号提示。
const DOUBLE_BRACE_MESSAGE: &str =
  "Double braces are not permitted within interpolated strings; did you mean '\\{'?";

/// `parse_interpolated_string_*`：插值表达式缺 `}` 提示。
const MISSING_END_BRACE_MESSAGE: &str =
  "Malformed interpolated string; did you forget to add a '}'?";

/// `parse_interpolated_string_*_in_table`：错误恢复后表字面量的收尾错误。
const UNCLOSED_TABLE_MESSAGE: &str = "Expected '}' (to close '{' at line 2), got <eof>";

/// `parse_error_with_too_many_nested_type_group`：类型标注递归超限。
const TYPE_RECURSION_MESSAGE: &str =
  "Exceeded allowed recursion depth; simplify your type annotation to make the code compile";

/// `parse_error_with_too_many_nested_ifelse_expressions*`：表达式递归超限。
const EXPR_RECURSION_MESSAGE: &str =
  "Exceeded allowed recursion depth; simplify your expression to make the code compile";

mod parser_parse_class_declarations_unaffected_by_global_flag {
  //! `cpp/tests/Parser.test.cpp:2175` `parse_class_declarations_unaffected_by_global_flag`

  use ulua_ast::records::{
    ast_stat_declare_extern_type::AstStatDeclareExternType, parse_options::ParseOptions,
  };
  use ulua_common::fflag::LuauAllowGlobalDeclarationToBeCalledClass;
  use ulua_unit_test::{
    functions::ast_node_ref::as_node_at, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  #[test]
  fn parser_parse_class_declarations_unaffected_by_global_flag() {
    let _flag = ScopedFastFlag::new(&LuauAllowGlobalDeclarationToBeCalledClass, true);

    let mut fixture = Fixture::default();
    let stat = fixture
      .parse_ex(
        r#"
        declare extern type Foo with
            prop: number
        end
    "#,
        &ParseOptions::default(),
      )
      .root_block()
      .expect("解析成功时根块必存在");

    assert_eq!(1, stat.body.size);
    let declared = as_node_at::<AstStatDeclareExternType, _>(&stat.body, 0)
      .expect("body[0] 应为 extern 类型声明");
    assert_eq!(declared.name, "Foo");
  }
}

mod parser_parse_compound_assignment {
  //! `cpp/tests/Parser.test.cpp:1071` `parse_compound_assignment`

  use ulua_ast::records::{
    ast_expr_binary::AstExprBinaryOp, ast_stat_compound_assign::AstStatCompoundAssign,
    parse_options::ParseOptions,
  };
  use ulua_unit_test::{functions::ast_node_ref::as_node_at, records::fixture::Fixture};

  #[test]
  fn parser_parse_compound_assignment() {
    let mut fixture = Fixture::default();
    let block = fixture.parse("a += 5", &ParseOptions::default());

    assert_eq!(1, block.body.size);
    let compound =
      as_node_at::<AstStatCompoundAssign, _>(&block.body, 0).expect("body[0] 应为复合赋值语句");
    assert_eq!(AstExprBinaryOp::Add, compound.op);
  }
}

mod parser_parse_compound_assignment_error_call {
  //! `cpp/tests/Parser.test.cpp:1083` `parse_compound_assignment_error_call`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_compound_assignment_error_call() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "a() += 5",
      "Expected identifier when parsing expression, got '+='",
      None,
    );
  }
}

mod parser_parse_compound_assignment_error_multiple {
  //! `cpp/tests/Parser.test.cpp:1113` `parse_compound_assignment_error_multiple`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_compound_assignment_error_multiple() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "a, b += 5",
      "Expected '=' when parsing assignment, got '+='",
      None,
    );
  }
}

mod parser_parse_compound_assignment_error_not_lvalue {
  //! `cpp/tests/Parser.test.cpp:1098` `parse_compound_assignment_error_not_lvalue`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_compound_assignment_error_not_lvalue() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "(a) += 5",
      "Assigned expression must be a variable or a field",
      None,
    );
  }
}

mod parser_parse_const {
  //! `cpp/tests/Parser.test.cpp:3171` `parse_const`

  use ulua_ast::records::{
    ast_expr_constant_number::AstExprConstantNumber, ast_stat_local::AstStatLocal,
    parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{as_node_at, deref_at},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_const() {
    let mut fixture = Fixture::default();
    let stat = fixture.parse("const f = 42", &ParseOptions::default());

    assert_eq!(1, stat.body.size);
    let stat_local = as_node_at::<AstStatLocal, _>(&stat.body, 0).expect("body[0] 应为局部声明");
    assert_eq!(1, stat_local.vars.size);
    assert_eq!(1, stat_local.values.size);

    let local = deref_at(&stat_local.vars, 0).expect("vars[0] 必存在");
    assert_eq!(local.name, "f");
    assert!(local.is_const);

    let value = as_node_at::<AstExprConstantNumber, _>(&stat_local.values, 0)
      .expect("values[0] 应为数字常量");
    assert_eq!(42.0, value.value);
  }
}

mod parser_parse_const_call {
  //! `cpp/tests/Parser.test.cpp:3231` `parse_const_call`：`const` 作普通标识符仍可解析。

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_const_call() {
    let mut fixture = Fixture::default();
    fixture.parse(
      "local const = function(t) return t end\nconst { a = \"a\" }",
      &ParseOptions::default(),
    );
  }
}

mod parser_parse_const_function {
  //! `cpp/tests/Parser.test.cpp:3203` `parse_const_function`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_const_function() {
    let mut fixture = Fixture::default();
    fixture.parse("const function f() return 42 end", &ParseOptions::default());
  }
}

mod parser_parse_const_function_with_attr {
  //! `cpp/tests/Parser.test.cpp:3212` `parse_const_function_with_attr`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_const_function_with_attr() {
    let mut fixture = Fixture::default();
    fixture.parse(
      "@deprecated\nconst function f() return 42 end",
      &ParseOptions::default(),
    );
  }
}

mod parser_parse_const_multi_initialize {
  //! `cpp/tests/Parser.test.cpp:3190` `parse_const_multi_initialize`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_const_multi_initialize() {
    let mut fixture = Fixture::default();
    fixture.parse(
      r#"
        const a, b = 42, 32

        const a, b, c = 42, f()

        const a, b, c = 42, ...
    "#,
      &ParseOptions::default(),
    );
  }
}

mod parser_parse_continue {
  //! `cpp/tests/Parser.test.cpp:991` `parse_continue`：`continue` 仅在语句位置才是关键字。

  use ulua_ast::records::{
    ast_stat_assign::AstStatAssign, ast_stat_continue::AstStatContinue, ast_stat_expr::AstStatExpr,
    ast_stat_while::AstStatWhile, parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{PtrRef, as_node_at},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_continue() {
    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      r#"
        while true do
            continue()
            continue = 5
            continue, continue = continue
            continue
        end
    "#,
      &ParseOptions::default(),
    );

    assert_eq!(1, stat.body.size);
    let wb = as_node_at::<AstStatWhile, _>(&stat.body, 0).expect("body[0] 应为 while 语句");

    let wblock = wb.body.as_ref_opt().expect("while 循环体必存在");
    assert_eq!(4, wblock.body.size);

    assert!(as_node_at::<AstStatExpr, _>(&wblock.body, 0).is_some());
    assert!(as_node_at::<AstStatAssign, _>(&wblock.body, 1).is_some());
    assert!(as_node_at::<AstStatAssign, _>(&wblock.body, 2).is_some());
    assert!(as_node_at::<AstStatContinue, _>(&wblock.body, 3).is_some());
  }
}

mod parser_parse_debugnoinline_on_local_function {
  //! `cpp/tests/Parser.test.cpp:5349` `parse_debugnoinline_on_local_function`

  use ulua_ast::records::{
    ast_attr::AstAttrType, ast_stat_local_function::AstStatLocalFunction, location::Location,
    parse_options::ParseOptions, position::Position,
  };
  use ulua_common::fflag::DebugLuauNoInline;
  use ulua_unit_test::{
    functions::{
      ast_node_ref::{PtrRef, as_node_at, deref_at},
      check_attribute::check_attribute,
    },
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  #[test]
  fn parser_parse_debugnoinline_on_local_function() {
    let _no_inline = ScopedFastFlag::new(&DebugLuauNoInline, true);

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "\n    @debugnoinline\nlocal function hello(x, y)\n    return x + y\nend",
      &ParseOptions::default(),
    );

    let stat_fun =
      as_node_at::<AstStatLocalFunction, _>(&stat.body, 0).expect("body[0] 应为 local function");
    let func = stat_fun
      .func
      .as_ref_opt()
      .expect("local function 必带函数表达式");
    assert_eq!(1, func.attributes.size);

    check_attribute(
      deref_at(&func.attributes, 0).expect("attributes[0] 必存在"),
      AstAttrType::DebugNoinline,
      Location::new(Position::new(1, 4), Position::new(1, 18)),
    );
  }
}

mod parser_parse_declarations {
  //! `cpp/tests/Parser.test.cpp:2116` `parse_declarations`

  use ulua_ast::records::{
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_type_pack_explicit::AstTypePackExplicit,
    location::Location, parse_options::ParseOptions, position::Position,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, PtrRef, as_node_at},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_declarations() {
    let mut fixture = Fixture::default();
    let stat = fixture
      .parse_ex(
        r#"
        declare foo: number
        declare function bar(x: number): string
        declare function var(...: any)
    "#,
        &ParseOptions::default(),
      )
      .root_block()
      .expect("解析成功时根块必存在");

    assert_eq!(3, stat.body.size);

    let global =
      as_node_at::<AstStatDeclareGlobal, _>(&stat.body, 0).expect("body[0] 应为全局变量声明");
    assert_eq!(global.name, "foo");
    assert_eq!(
      Location::new(Position::new(1, 16), Position::new(1, 19)),
      global.name_location
    );
    assert!(global.type_.as_ref_opt().is_some());

    let func =
      as_node_at::<AstStatDeclareFunction, _>(&stat.body, 1).expect("body[1] 应为函数声明");
    assert_eq!(func.name, "bar");
    assert_eq!(
      Location::new(Position::new(2, 25), Position::new(2, 28)),
      func.name_location
    );
    assert_eq!(1, func.params.types.size);

    let ret_type_pack = func
      .ret_types
      .as_node::<AstTypePackExplicit>()
      .expect("返回类型应为显式 type pack");
    assert_eq!(1, ret_type_pack.type_list.types.size);

    let var_func =
      as_node_at::<AstStatDeclareFunction, _>(&stat.body, 2).expect("body[2] 应为函数声明");
    assert_eq!(var_func.name, "var");
    assert_eq!(
      Location::new(Position::new(3, 25), Position::new(3, 28)),
      var_func.name_location
    );
    assert!(var_func.params.tail_type.as_ref_opt().is_some());
    assert!(var_func.vararg);
    assert_eq!(
      Location::new(Position::new(3, 29), Position::new(3, 32)),
      var_func.vararg_location
    );

    fixture.match_parse_error(
      "declare function foo(x)",
      "All declaration parameters must be annotated",
      None,
    );
    fixture.match_parse_error(
      "declare foo",
      "Expected ':' when parsing global variable declaration, got <eof>",
      None,
    );
  }
}

mod parser_parse_declared_table_checked_member {
  //! `cpp/tests/Parser.test.cpp:5017` `parse_declared_table_checked_member`

  use ulua_ast::records::{
    ast_stat_declare_global::AstStatDeclareGlobal, ast_type_function::AstTypeFunction,
    ast_type_table::AstTypeTable, parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at, elem},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_declared_table_checked_member() {
    let mut fixture = Fixture::default();
    // 与 C++ R"BUILTIN_SRC(...)" 逐字符一致：首行换行 + 4/8 空格缩进。
    let code = "\n    declare math : {\n        abs : @checked (number) -> number\n}\n";
    let opts = ParseOptions {
      allow_declaration_syntax: true,
      ..Default::default()
    };

    let pr = fixture.try_parse(code, &opts);
    assert!(pr.errors.is_empty());

    let root = pr.root_block().expect("解析成功时根块必存在");
    assert_eq!(1, root.body.size);
    // C++ `AstStat* root = *(pr.root->body.data);` —— 取的是首条语句。
    let glob =
      as_node_at::<AstStatDeclareGlobal, _>(&root.body, 0).expect("body[0] 应为全局变量声明");
    let tbl = glob
      .type_
      .as_node::<AstTypeTable>()
      .expect("声明类型应为表类型");
    assert_eq!(1, tbl.props.size);

    let prop = elem(&tbl.props, 0);
    let func = prop
      .r#type
      .as_node::<AstTypeFunction>()
      .expect("属性类型应为函数类型");
    assert!(func.is_checked_function());
  }
}

mod parser_parse_error_assignment_lvalue {
  //! `cpp/tests/Parser.test.cpp:2052` `parse_error_assignment_lvalue`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_assignment_lvalue() {
    let mut fixture = Fixture::default();
    for code in [
      "\n        local a, b\n        (2), b = b, a\n    ",
      "\n        local a, b\n        a, (3) = b, a\n    ",
    ] {
      fixture.match_parse_error(
        code,
        "Assigned expression must be a variable or a field",
        None,
      );
    }
  }
}

mod parser_parse_error_broken_comment {
  //! `cpp/tests/Parser.test.cpp:1866` `parse_error_broken_comment`

  use ulua_unit_test::records::fixture::Fixture;

  const EXPECTED: &str = "Expected identifier when parsing expression, got unfinished comment";

  #[test]
  fn parser_parse_error_broken_comment() {
    let mut fixture = Fixture::default();
    for code in [
      "--[[unfinished work",
      "--!strict\n--[[unfinished work",
      "local x = 1 --[[unfinished work",
    ] {
      fixture.match_parse_error(code, EXPECTED, None);
    }
  }
}

mod parser_parse_error_confusing_function_call {
  //! `cpp/tests/Parser.test.cpp:1994` `parse_error_confusing_function_call`

  use ulua_unit_test::records::fixture::Fixture;

  use super::AMBIGUOUS_ARGUMENT_LIST;

  #[test]
  fn parser_parse_error_confusing_function_call() {
    let mut fixture = Fixture::default();
    let sources = [
      "\n        function add(x, y) return x + y end\n        add\n        (4, 7)\n    ",
      "\n        function add(x, y) return x + y end\n        local f = add\n        (f :: any)['x'] = 2\n    ",
      "\n        local x = {}\n        function x:add(a, b) return a + b end\n        x:add\n        (1, 2)\n    ",
      "\n        local t = {}\n        function f() return t end\n        t.x, (f)\n        ().y = 5, 6\n    ",
    ];

    for code in sources {
      let result = fixture.match_parse_error(code, AMBIGUOUS_ARGUMENT_LIST, None);
      assert_eq!(1, result.errors.len());
    }
  }
}

mod parser_parse_error_function_call {
  //! `cpp/tests/Parser.test.cpp:1517` `parse_error_function_call`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_function_call() {
    let mut fixture = Fixture::default();
    let code =
      "\nfunction stringifyTable(t)\n    local foo = t:Parse 2\n    return foo\nend\n        ";
    let result = fixture.try_parse(code, &ParseOptions::default());

    let first = result
      .errors
      .first()
      .expect("Expected ParseErrors to be thrown");
    assert_eq!(2, first.get_location().begin.line);
    assert_eq!(
      "Expected '(', '{' or <string> when parsing function call, got '2'",
      first.get_message()
    );
  }
}

mod parser_parse_error_function_call_newline {
  //! `cpp/tests/Parser.test.cpp:1536` `parse_error_function_call_newline`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_function_call_newline() {
    let mut fixture = Fixture::default();
    let code =
      "\nfunction stringifyTable(t)\n    local foo = t:Parse\n    return foo\nend\n        ";
    let result = fixture.try_parse(code, &ParseOptions::default());

    let first = result
      .errors
      .first()
      .expect("Expected ParseErrors to be thrown");
    assert_eq!(2, first.get_location().begin.line);
    assert_eq!(
      "Expected function call arguments after '('",
      first.get_message()
    );
  }
}

mod parser_parse_error_loop_control {
  //! `cpp/tests/Parser.test.cpp:1986` `parse_error_loop_control`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_loop_control() {
    let mut fixture = Fixture::default();
    for (code, message) in [
      ("break", "break statement must be inside a loop"),
      (
        "repeat local function a() break end until false",
        "break statement must be inside a loop",
      ),
      ("continue", "continue statement must be inside a loop"),
      (
        "repeat local function a() continue end until false",
        "continue statement must be inside a loop",
      ),
    ] {
      fixture.match_parse_error(code, message, None);
    }
  }
}

mod parser_parse_error_messages {
  //! `cpp/tests/Parser.test.cpp:602` `parse_error_messages`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_messages() {
    let mut fixture = Fixture::default();
    for (code, message) in [
      (
        "\n        local a: (number, number) -> (string\n    ",
        "Expected ')' (to close '(' at line 2), got <eof>",
      ),
      (
        "\n        local a: (number, number) -> (\n            string\n    ",
        "Expected ')' (to close '(' at line 2), got <eof>",
      ),
      (
        "\n        local a: (number, number)\n    ",
        "Expected '->' when parsing function type, got <eof>",
      ),
      (
        "\n        local a: (number, number\n    ",
        "Expected ')' (to close '(' at line 2), got <eof>",
      ),
      (
        "\n        local a: {foo: string,\n    ",
        "Expected identifier when parsing table field, got <eof>",
      ),
      (
        "\n        local a: {foo: string\n    ",
        "Expected '}' (to close '{' at line 2), got <eof>",
      ),
      (
        "\n        local a: { [string]: number, [number]: string }\n    ",
        "Cannot have more than one table indexer",
      ),
      (
        "\n        type T = <a>foo\n    ",
        "Expected '(' when parsing function parameters, got 'foo'",
      ),
    ] {
      fixture.match_parse_error(code, message, None);
    }
  }
}

mod parser_parse_error_missing_type_annotation {
  //! `cpp/tests/Parser.test.cpp:2076` `parse_error_missing_type_annotation`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_missing_type_annotation() {
    let mut fixture = Fixture::default();
    for (code, width, message) in [
      ("local x:", 0, "Expected type, got <eof>"),
      ("\nlocal x:=42\n    ", 1, "Expected type, got '='"),
      ("\nfunction func():end\n    ", 3, "Expected type, got 'end'"),
    ] {
      let result = fixture.try_parse(code, &ParseOptions::default());
      assert_eq!(1, result.errors.len());

      let error = &result.errors[0];
      let location = error.get_location();
      assert_eq!(location.begin.line, location.end.line);
      assert_eq!(width, location.end.column - location.begin.column);
      assert_eq!(message, error.get_message());
    }
  }
}

mod parser_parse_error_table_literal {
  //! `cpp/tests/Parser.test.cpp:1497` `parse_error_table_literal`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_table_literal() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "\nfunction stringifyTable(t)\n    local foo = (name = t)\n    return foo\nend\n        ",
      "Expected ')' (to close '(' at column 17), got '='; did you mean to use '{' when defining a table?",
      None,
    );
  }
}

mod parser_parse_error_type_annotation {
  //! `cpp/tests/Parser.test.cpp:2071` `parse_error_type_annotation`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_type_annotation() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error("local a : 2 = 2", "Expected type, got '2'", None);
  }
}

mod parser_parse_error_type_name {
  //! `cpp/tests/Parser.test.cpp:792` `parse_error_type_name`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_type_name() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "\n        local a: Foo.=\n    ",
      "Expected identifier when parsing field name, got '='",
      None,
    );
  }
}

mod parser_parse_error_varargs {
  //! `cpp/tests/Parser.test.cpp:2047` `parse_error_varargs`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_error_varargs() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "function add(x, y) return ... end",
      "Cannot use '...' outside of a vararg function",
      None,
    );
  }
}

mod parser_parse_error_with_too_many_changed_elseif_statements {
  //! `cpp/tests/Parser.test.cpp:2364` `parse_error_with_too_many_changed_elseif_statements`

  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  #[test]
  fn parser_parse_error_with_too_many_changed_elseif_statements() {
    let _sfi = ScopedFastInt::new(&fint::LuauRecursionLimit, 10);

    let mut fixture = Fixture::default();
    fixture.match_parse_error_prefix(
      "function f() if false then elseif false then elseif false then elseif false then \
       elseif false then elseif false then elseif false then elseif false then \
       elseif false then elseif false then elseif false then end end",
      "Exceeded allowed recursion depth;",
    );
  }
}

mod parser_parse_error_with_too_many_nested_if_statements {
  //! `cpp/tests/Parser.test.cpp:2354` `parse_error_with_too_many_nested_if_statements`

  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  #[test]
  fn parser_parse_error_with_too_many_nested_if_statements() {
    let _sfi = ScopedFastInt::new(&fint::LuauRecursionLimit, 10);

    let mut fixture = Fixture::default();
    fixture.match_parse_error_prefix(
      "function f() if true then if true then if true then if true then if true then \
       if true then if true then if true then if true then if true then if true then \
       end end end end end end end end end end end end",
      "Exceeded allowed recursion depth;",
    );
  }
}

mod parser_parse_error_with_too_many_nested_ifelse_expressions1 {
  //! `cpp/tests/Parser.test.cpp:2374` `parse_error_with_too_many_nested_ifelse_expressions1`

  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  use super::EXPR_RECURSION_MESSAGE;

  #[test]
  fn parser_parse_error_with_too_many_nested_ifelse_expressions1() {
    let _sfi = ScopedFastInt::new(&fint::LuauRecursionLimit, 10);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "function f() return if true then 1 elseif true then 2 elseif true then 3 elseif true then 4 elseif true then 5 elseif true then 6 elseif true then 7 elseif true then 8 elseif true then 9 elseif true then 10 else 11 end",
      EXPR_RECURSION_MESSAGE,
      None,
    );
  }
}

mod parser_parse_error_with_too_many_nested_ifelse_expressions2 {
  //! `cpp/tests/Parser.test.cpp:2384` `parse_error_with_too_many_nested_ifelse_expressions2`

  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  use super::EXPR_RECURSION_MESSAGE;

  #[test]
  fn parser_parse_error_with_too_many_nested_ifelse_expressions2() {
    let _sfi = ScopedFastInt::new(&fint::LuauRecursionLimit, 10);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "function f() return if if if if if if if if if if true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then 1 else 2 end",
      EXPR_RECURSION_MESSAGE,
      None,
    );
  }
}

mod parser_parse_error_with_too_many_nested_type_group {
  //! `cpp/tests/Parser.test.cpp:1555` `parse_error_with_too_many_nested_type_group`

  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  use super::TYPE_RECURSION_MESSAGE;

  #[test]
  fn parser_parse_error_with_too_many_nested_type_group() {
    let _sfi = ScopedFastInt::new(&fint::LuauRecursionLimit, 10);

    let mut fixture = Fixture::default();
    for code in [
      "function f(): ((((((((((Fail)))))))))) end",
      "function f(): () -> () -> () -> () -> () -> () -> () -> () -> () -> () -> () end",
      "local t: {a: {b: {c: {d: {e: {f: {g: {h: {i: {j: {}}}}}}}}}}}",
      "local f: ((((((((((Fail))))))))))",
      "local t: a & (b & (c & (d & (e & (f & (g & (h & (i & (j & nil)))))))))",
    ] {
      fixture.match_parse_error(code, TYPE_RECURSION_MESSAGE, None);
    }
  }
}

mod parser_parse_export_type {
  //! `cpp/tests/Parser.test.cpp:1026` `parse_export_type`

  use ulua_ast::records::{
    ast_stat_assign::AstStatAssign, ast_stat_expr::AstStatExpr,
    ast_stat_type_alias::AstStatTypeAlias, parse_options::ParseOptions,
  };
  use ulua_unit_test::{functions::ast_node_ref::as_node_at, records::fixture::Fixture};

  #[test]
  fn parser_parse_export_type() {
    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      r#"
        export()
        export = 5
        export, export = export
        export type A = number
        type A = number
    "#,
      &ParseOptions::default(),
    );

    assert_eq!(5, stat.body.size);
    assert!(as_node_at::<AstStatExpr, _>(&stat.body, 0).is_some());
    assert!(as_node_at::<AstStatAssign, _>(&stat.body, 1).is_some());
    assert!(as_node_at::<AstStatAssign, _>(&stat.body, 2).is_some());
    assert!(as_node_at::<AstStatTypeAlias, _>(&stat.body, 3).is_some());
    assert!(as_node_at::<AstStatTypeAlias, _>(&stat.body, 4).is_some());
  }
}

mod parser_parse_extern_type_declarations {
  //! `cpp/tests/Parser.test.cpp:2243` `parse_extern_type_declarations`

  use ulua_ast::records::{
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_type_function::AstTypeFunction,
    ast_type_reference::AstTypeReference, location::Location, parse_options::ParseOptions,
    position::Position,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at, elem},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_extern_type_declarations() {
    let mut fixture = Fixture::default();
    let stat = fixture
      .parse_ex(
        r#"
        declare extern type Foo with
            prop: number
            function method(self, foo: number): string
        end

        declare extern type Bar extends Foo with
            prop2: string
        end
    "#,
        &ParseOptions::default(),
      )
      .root_block()
      .expect("解析成功时根块必存在");

    assert_eq!(2, stat.body.size);

    let declared_extern_type = as_node_at::<AstStatDeclareExternType, _>(&stat.body, 0)
      .expect("body[0] 应为 extern 类型声明");
    assert_eq!(declared_extern_type.name, "Foo");
    assert!(declared_extern_type.super_name.is_none());
    assert_eq!(2, declared_extern_type.props.size);

    let prop = elem(&declared_extern_type.props, 0);
    assert_eq!(prop.name, "prop");
    assert_eq!(
      Location::new(Position::new(2, 12), Position::new(2, 16)),
      prop.name_location
    );
    assert!(prop.ty.as_node::<AstTypeReference>().is_some());
    assert_eq!(
      Location::new(Position::new(2, 12), Position::new(2, 24)),
      prop.location
    );

    let method = elem(&declared_extern_type.props, 1);
    assert_eq!(method.name, "method");
    assert_eq!(
      Location::new(Position::new(3, 21), Position::new(3, 27)),
      method.name_location
    );
    assert!(method.ty.as_node::<AstTypeFunction>().is_some());
    assert_eq!(
      Location::new(Position::new(3, 12), Position::new(3, 54)),
      method.location
    );
    assert!(method.is_method);

    let subclass = as_node_at::<AstStatDeclareExternType, _>(&stat.body, 1)
      .expect("body[1] 应为 extern 类型声明");
    assert_eq!(subclass.name, "Bar");
    assert_eq!(subclass.super_name.expect("子类必带父类型名"), "Foo");
    assert_eq!(1, subclass.props.size);

    let prop2 = elem(&subclass.props, 0);
    assert_eq!(prop2.name, "prop2");
    assert_eq!(
      Location::new(Position::new(7, 12), Position::new(7, 17)),
      prop2.name_location
    );
    assert!(prop2.ty.as_node::<AstTypeReference>().is_some());
    assert_eq!(
      Location::new(Position::new(7, 12), Position::new(7, 25)),
      prop2.location
    );
  }
}

mod parser_parse_extern_type_declarations_missing_with {
  //! `cpp/tests/Parser.test.cpp:2293` `parse_extern_type_declarations_missing_with`

  use ulua_ast::records::{
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_type_function::AstTypeFunction,
    ast_type_reference::AstTypeReference, location::Location, parse_options::ParseOptions,
    position::Position,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at, elem},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_extern_type_declarations_missing_with() {
    let mut fixture = Fixture::default();
    let result = fixture.try_parse(
      r#"
        declare extern type Foo
            prop: number
            function method(self, foo: number): string
        end

        declare extern type Bar extends Foo
            prop2: string
        end
    "#,
      &ParseOptions::default(),
    );

    assert_eq!(2, result.errors.len());
    assert_eq!(
      "Expected `with` keyword before listing properties of the external type, but got prop instead",
      result.errors[0].get_message()
    );
    assert_eq!(
      "Expected `with` keyword before listing properties of the external type, but got prop2 instead",
      result.errors[1].get_message()
    );

    let stat = result.root_block().expect("错误恢复后根块仍存在");
    assert_eq!(2, stat.body.size);

    let declared_extern_type = as_node_at::<AstStatDeclareExternType, _>(&stat.body, 0)
      .expect("body[0] 应为 extern 类型声明");
    assert_eq!(declared_extern_type.name, "Foo");
    assert!(declared_extern_type.super_name.is_none());
    assert_eq!(2, declared_extern_type.props.size);

    let prop = elem(&declared_extern_type.props, 0);
    assert_eq!(prop.name, "prop");
    assert_eq!(
      Location::new(Position::new(2, 12), Position::new(2, 16)),
      prop.name_location
    );
    assert!(prop.ty.as_node::<AstTypeReference>().is_some());
    assert_eq!(
      Location::new(Position::new(2, 12), Position::new(2, 24)),
      prop.location
    );

    let method = elem(&declared_extern_type.props, 1);
    assert_eq!(method.name, "method");
    assert_eq!(
      Location::new(Position::new(3, 21), Position::new(3, 27)),
      method.name_location
    );
    assert!(method.ty.as_node::<AstTypeFunction>().is_some());
    assert_eq!(
      Location::new(Position::new(3, 12), Position::new(3, 54)),
      method.location
    );
    assert!(method.is_method);

    let subclass = as_node_at::<AstStatDeclareExternType, _>(&stat.body, 1)
      .expect("body[1] 应为 extern 类型声明");
    assert_eq!(subclass.name, "Bar");
    assert_eq!(subclass.super_name.expect("子类必带父类型名"), "Foo");
    assert_eq!(1, subclass.props.size);

    let prop2 = elem(&subclass.props, 0);
    assert_eq!(prop2.name, "prop2");
    assert_eq!(
      Location::new(Position::new(7, 12), Position::new(7, 17)),
      prop2.name_location
    );
    assert!(prop2.ty.as_node::<AstTypeReference>().is_some());
    assert_eq!(
      Location::new(Position::new(7, 12), Position::new(7, 25)),
      prop2.location
    );
  }
}

mod parser_parse_global_declaration_called_class {
  //! `cpp/tests/Parser.test.cpp:2156` `parse_global_declaration_called_class`

  use ulua_ast::records::{
    ast_stat_declare_global::AstStatDeclareGlobal, ast_type_table::AstTypeTable,
    parse_options::ParseOptions,
  };
  use ulua_common::fflag::LuauAllowGlobalDeclarationToBeCalledClass;
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  #[test]
  fn parser_parse_global_declaration_called_class() {
    let _sff = ScopedFastFlag::new(&LuauAllowGlobalDeclarationToBeCalledClass, true);

    let mut fixture = Fixture::default();
    let stat = fixture
      .parse_ex(
        "\n        declare class: { x: number }\n    ",
        &ParseOptions::default(),
      )
      .root_block()
      .expect("解析成功时根块必存在");

    assert_eq!(1, stat.body.size);
    let global =
      as_node_at::<AstStatDeclareGlobal, _>(&stat.body, 0).expect("body[0] 应为全局变量声明");
    assert_eq!(global.name, "class");
    assert!(global.type_.as_node::<AstTypeTable>().is_some());
  }
}

mod parser_parse_if_else_expression {
  //! `cpp/tests/Parser.test.cpp:2783` `parse_if_else_expression`

  use ulua_ast::records::{
    ast_expr_if_else::AstExprIfElse, ast_stat_return::AstStatReturn, parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at},
    records::fixture::Fixture,
  };

  /// cpp 四处共用的前奏：`parse(源)` 后取 `body[0]` 的 `AstStatReturn`，
  /// 断言 `list.size == 1` 并返回 `list[0]` 的 if-else 表达式。
  fn parse_head<'a>(fixture: &'a mut Fixture, code: &str) -> &'a AstExprIfElse {
    let stat = fixture.parse(code, &ParseOptions::default());

    let ret = as_node_at::<AstStatReturn, _>(&stat.body, 0).expect("body[0] 应为 return 语句");
    assert_eq!(1, ret.list.size);
    as_node_at::<AstExprIfElse, _>(&ret.list, 0).expect("list[0] 应为 if-else 表达式")
  }

  #[test]
  fn parser_parse_if_else_expression() {
    let mut fixture = Fixture::default();

    parse_head(&mut fixture, "return if true then 1 else 2");

    let if_else_expr1 = parse_head(
      &mut fixture,
      "return if true then 1 elseif true then 2 else 3",
    );
    assert!(
      if_else_expr1
        .false_expr
        .as_node::<AstExprIfElse>()
        .is_some()
    );

    // 用 "else if" 而非 "elseif"
    let if_else_expr2 = parse_head(
      &mut fixture,
      "return if true then 1 else if true then 2 else 3",
    );
    assert!(
      if_else_expr2
        .false_expr
        .as_node::<AstExprIfElse>()
        .is_some()
    );

    // if-else 表达式作另一个 if-else 表达式的条件
    let if_else_expr3 = parse_head(
      &mut fixture,
      "return if if true then false else true then 1 else 2",
    );
    assert!(if_else_expr3.condition.as_node::<AstExprIfElse>().is_some());
  }
}

mod parser_parse_interpolated_string_as_type_fail {
  //! `cpp/tests/Parser.test.cpp:1220` `parse_interpolated_string_as_type_fail`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_interpolated_string_as_type_fail() {
    let mut fixture = Fixture::default();
    let result = fixture.try_parse(
      "\n            local a: `what` = `???`\n            local b: `what {\"the\"}` = `???`\n            local c: `what {\"the\"} heck` = `???`\n        ",
      &ParseOptions::default(),
    );

    assert_eq!(3, result.errors.len());
    for error in &result.errors {
      assert_eq!(
        "Interpolated string literals cannot be used as types",
        error.get_message()
      );
    }
  }
}

mod parser_parse_interpolated_string_call_without_parens {
  //! `cpp/tests/Parser.test.cpp:1240` `parse_interpolated_string_call_without_parens`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_interpolated_string_call_without_parens() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "_ = print `{42}`",
      "Expected identifier when parsing expression, got `{",
      None,
    );
  }
}

mod parser_parse_interpolated_string_double_brace_begin {
  //! `cpp/tests/Parser.test.cpp:1128` `parse_interpolated_string_double_brace_begin`

  use ulua_unit_test::records::fixture::Fixture;

  use super::DOUBLE_BRACE_MESSAGE;

  #[test]
  fn parser_parse_interpolated_string_double_brace_begin() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "\n            _ = `{{oops}}`\n        ",
      DOUBLE_BRACE_MESSAGE,
      None,
    );
  }
}

mod parser_parse_interpolated_string_double_brace_mid {
  //! `cpp/tests/Parser.test.cpp:1143` `parse_interpolated_string_double_brace_mid`

  use ulua_unit_test::records::fixture::Fixture;

  use super::DOUBLE_BRACE_MESSAGE;

  #[test]
  fn parser_parse_interpolated_string_double_brace_mid() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "\n            _ = `{nice} {{oops}}`\n        ",
      DOUBLE_BRACE_MESSAGE,
      None,
    );
  }
}

mod parser_parse_interpolated_string_malformed_escape {
  //! `cpp/tests/Parser.test.cpp:1282` `parse_interpolated_string_malformed_escape`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_interpolated_string_malformed_escape() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "\n            local a = `???\\xQQ {1}`\n        ",
      "Interpolated string literal contains malformed escape sequence",
      None,
    );
  }
}

mod parser_parse_interpolated_string_mid_without_end_brace_in_table {
  //! `cpp/tests/Parser.test.cpp:1202` `parse_interpolated_string_mid_without_end_brace_in_table`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  use super::{MISSING_END_BRACE_MESSAGE, UNCLOSED_TABLE_MESSAGE};

  #[test]
  fn parser_parse_interpolated_string_mid_without_end_brace_in_table() {
    let mut fixture = Fixture::default();
    let result = fixture.try_parse(
      "\n            _ = { `x {\"y\"} {z` }\n        ",
      &ParseOptions::default(),
    );

    assert_eq!(2, result.errors.len());
    assert_eq!(MISSING_END_BRACE_MESSAGE, result.errors[0].get_message());
    assert_eq!(UNCLOSED_TABLE_MESSAGE, result.errors[1].get_message());
  }
}

mod parser_parse_interpolated_string_weird_token {
  //! `cpp/tests/Parser.test.cpp:1297` `parse_interpolated_string_weird_token`

  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_interpolated_string_weird_token() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      "\n            local a = `??? {42 !!}`\n        ",
      "Malformed interpolated string, got '!'",
      None,
    );
  }
}

mod parser_parse_interpolated_string_with_lookahead_involved {
  //! `cpp/tests/Parser.test.cpp:4980` `parse_interpolated_string_with_lookahead_involved`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_interpolated_string_with_lookahead_involved() {
    let mut fixture = Fixture::default();
    let result = fixture.try_parse(
      "\n        local x = `{ {y} }`\n    ",
      &ParseOptions::default(),
    );
    assert!(result.errors.is_empty());
  }
}

mod parser_parse_interpolated_string_with_lookahead_involved2 {
  //! `cpp/tests/Parser.test.cpp:4989` `parse_interpolated_string_with_lookahead_involved2`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_interpolated_string_with_lookahead_involved2() {
    let mut fixture = Fixture::default();
    let result = fixture.try_parse(
      "\n        local x = `{ { y{} } }`\n    ",
      &ParseOptions::default(),
    );
    assert!(result.errors.is_empty());
  }
}

mod parser_parse_interpolated_string_without_end_brace {
  //! `cpp/tests/Parser.test.cpp:1158` `parse_interpolated_string_without_end_brace`
  //!
  //! cpp 的 `columnOfEndBraceError` lambda：解析失败时取错误位置起始列。

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  use super::MISSING_END_BRACE_MESSAGE;

  fn column_of_end_brace_error(fixture: &mut Fixture, code: &str) -> u32 {
    let result = fixture.try_parse(code, &ParseOptions::default());
    assert_eq!(1, result.errors.len());

    let error = result
      .errors
      .first()
      .expect("Expected ParseErrors to be thrown");
    assert_eq!(MISSING_END_BRACE_MESSAGE, error.get_message());
    error.get_location().begin.column
  }

  #[test]
  fn parser_parse_interpolated_string_without_end_brace() {
    let mut fixture = Fixture::default();

    // 确认错误来自闭合花括号本身
    assert_eq!(7, column_of_end_brace_error(&mut fixture, "_ = `{a`"));
    assert_eq!(
      13,
      column_of_end_brace_error(&mut fixture, "_ = `{abcdefg`")
    );
    assert_eq!(
      column_of_end_brace_error(&mut fixture, "_ =       `{a`"),
      column_of_end_brace_error(&mut fixture, "_ = `{abcdefg`")
    );
  }
}

mod parser_parse_interpolated_string_without_end_brace_in_table {
  //! `cpp/tests/Parser.test.cpp:1184` `parse_interpolated_string_without_end_brace_in_table`

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  use super::{MISSING_END_BRACE_MESSAGE, UNCLOSED_TABLE_MESSAGE};

  #[test]
  fn parser_parse_interpolated_string_without_end_brace_in_table() {
    let mut fixture = Fixture::default();
    let result = fixture.try_parse(
      "\n            _ = { `{a` }\n        ",
      &ParseOptions::default(),
    );

    assert_eq!(2, result.errors.len());
    assert_eq!(MISSING_END_BRACE_MESSAGE, result.errors[0].get_message());
    assert_eq!(UNCLOSED_TABLE_MESSAGE, result.errors[1].get_message());
  }
}

mod parser_parse_interpolated_string_without_expression {
  //! `cpp/tests/Parser.test.cpp:1255` `parse_interpolated_string_without_expression`

  use ulua_unit_test::records::fixture::Fixture;

  const MESSAGE: &str = "Malformed interpolated string, expected expression inside '{}'";

  #[test]
  fn parser_parse_interpolated_string_without_expression() {
    let mut fixture = Fixture::default();
    fixture.match_parse_error("print(`{}`)", MESSAGE, None);
    fixture.match_parse_error("print(`{}{1}`)", MESSAGE, None);
  }
}

mod parser_parse_local_const {
  //! `cpp/tests/Parser.test.cpp:3222` `parse_local_const`：`const` 作普通局部名。

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  #[test]
  fn parser_parse_local_const() {
    let mut fixture = Fixture::default();
    fixture.parse("local const", &ParseOptions::default());
  }
}

mod parser_parse_nested_ast_type_group {
  //! `cpp/tests/Parser.test.cpp:2942` `parse_nested_ast_type_group`

  use ulua_ast::records::{
    ast_stat_type_alias::AstStatTypeAlias, ast_type_group::AstTypeGroup,
    ast_type_reference::AstTypeReference, parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at},
    records::fixture::Fixture,
  };

  #[test]
  fn parser_parse_nested_ast_type_group() {
    let mut fixture = Fixture::default();
    let stat = fixture.parse("type Foo = ((string))", &ParseOptions::default());

    assert_eq!(1, stat.body.size);
    let alias1 = as_node_at::<AstStatTypeAlias, _>(&stat.body, 0).expect("body[0] 应为类型别名");

    let group1 = alias1
      .type_ptr
      .as_node::<AstTypeGroup>()
      .expect("别名右侧应为分组类型");
    let group2 = group1
      .type_
      .as_node::<AstTypeGroup>()
      .expect("内层仍应为分组类型");
    assert!(group2.type_.as_node::<AstTypeReference>().is_some());
  }
}
