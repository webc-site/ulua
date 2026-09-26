// Port of `cpp/tests/Parser.test.cpp`.
// Automatically aggregated test suite.

// cpp `cstNode->as<T>()`：CST 侧下转入口与 AST 侧同在 `ast_node_ref` 收口。
use ulua_unit_test::functions::ast_node_ref::CstNodePtr;

#[test]
fn parser_aligns_things() {
  use core::mem::align_of;

  use ulua_ast::records::allocator::Allocator;

  let mut alloc = Allocator::new();
  let _one = alloc.alloc(0u8);
  let two = alloc.alloc(0.0_f64);
  let align_mask = align_of::<f64>() - 1;
  let two_addr = two as usize;
  assert_eq!(0, two_addr & align_mask);
}
#[test]
fn parser_all_disallowed_metamethods() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag::DebugLuauUserDefinedClasses;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let code = r#"class Foo
      function __index() end
      function __newindex() end
      function __mode() end
      function __metatable() end
      function __type() end
      function __doesnotexist() end
  end"#;

  let result = fix.try_parse(code, &ParseOptions::default());

  assert_eq!(result.errors.len(), 6);
  assert_eq!(
    result.errors[5].get_message(),
    "Cannot use '__doesnotexist' as a method name: names starting with '__' are reserved"
  );
}
#[test]
fn parser_allocator_can_be_moved() {
  use core::ptr::null_mut;

  use ulua_ast::records::allocator::Allocator;
  use ulua_unit_test::{functions::ast_node_ref::PtrRef, records::counter::Counter};

  let mut c: *mut Counter = null_mut();

  let mut inner = || {
    let mut allocator = Allocator::new();
    c = allocator.alloc(Counter::counter_counter());

    Allocator::move_from(&mut allocator)
  };

  Counter::reset_instance_count();
  let _a = Allocator::move_from(&mut inner());

  assert_eq!(1, c.as_ref_opt().expect("counter 必须存活").id);
}
#[test]
fn parser_allow_unicode_in_string() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  // C++ source is `local snowman = "☃"` (U+2603); the ported literal was
  // double-encoded UTF-8 mojibake. Use a \u escape (encoding-safe).
  let source = String::from("local snowman = \"\u{2603}\"");
  let options = ParseOptions::new();
  let result = fixture.parse_ex(&source, &options);
  assert!(result.errors.is_empty());
}
#[test]
fn parser_allowed_metamethods_still_work() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag::DebugLuauUserDefinedClasses;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let mut fix = Fixture::default();
  let code = r"class Foo
      function __tostring(self) end
      function __add(self, other) end
      function __eq(self, other) end
      -- Silly, but allowed.
      function _(self) end
  end";

  let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);
  let result = fix.try_parse(code, &ParseOptions::new());

  assert_eq!(result.errors.len(), 0);
}
#[test]
fn parser_annotations_can_be_tables() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  // cpp `REQUIRE(stat != nullptr)`：`parse` 返回引用，非空由类型系统担保。
  let _stat = fixture.parse(
    "local zero: number\n\
           local one: {x: number, y: string}",
    &ParseOptions::default(),
  );
}
#[test]
fn parser_ast_name_comparison() {
  use ulua_ast::records::ast_name::AstName;

  let empty1 = AstName::new();
  let empty2 = AstName::new();
  assert!(!(empty1 < empty2));

  let one = AstName::ast_name_u8(c"one".as_ptr().cast());
  let two = AstName::ast_name_u8(c"two".as_ptr().cast());

  let one_lt_two = one < two;
  let two_lt_one = two < one;
  assert_ne!(one_lt_two, two_lt_one);
}
#[test]
fn parser_attributes_cannot_be_duplicated() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture,
  };

  let mut fix = Fixture::default();
  let code = "\n@checked\n    @checked\nfunction hello(x, y)\n    return x + y\nend";

  let result = fix.try_parse(code, &ParseOptions::default());

  let expected_location = Location::new(Position::new(2, 4), Position::new(2, 12));
  let expected_message = "Cannot duplicate attribute '@checked'";

  check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
}
#[test]
fn parser_basic_less_than_check_no_explicit_type_instantiaton() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let _stat = fixture.parse(r#"local a = b.c < d"#, &ParseOptions::default());
}
#[test]
fn parser_basic_parse() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let _stat = fixture.parse(r#"print("Hello World!")"#, &ParseOptions::new());
}
#[test]
fn parser_break_return_not_last_error() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  fixture.match_parse_error("return 0 print(5)", "Expected <eof>, got 'print'", None);
  fixture.match_parse_error(
    "while true do break print(5) end",
    "Expected 'end' (to close 'do' at column 12), got 'print'",
    None,
  );
}
#[test]
fn parser_can_haz_annotations() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  // cpp `REQUIRE(block != nullptr)`：`parse` 返回引用，非空由类型系统担保。
  let _block = fixture.parse(
    "local foo: string = \"Hello Types!\"",
    &ParseOptions::default(),
  );
}
#[test]
fn parser_can_parse_complex_unions_successfully() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  let mut fixture = Fixture::fixture_bool(false);

  // Set recursion limit and type length limit to 10 for all tests in this fixture
  let _sfis = [
    ScopedFastInt::new(&fint::LuauRecursionLimit, 10),
    ScopedFastInt::new(&fint::LuauTypeLengthLimit, 10),
  ];

  // Test 1: Complex union with multiple function types, table types, parenthesized types, and intersection types
  fixture.parse(
    r#"local f:
  () -> ()
  |
  () -> ()
  |
  {a: number}
  |
  {b: number}
  |
  ((number))
  |
  ((number))
  |
  (a & (b & nil))
  |
  (a & (b & nil))
  "#,
    &ParseOptions::default(),
  );

  // Test 2: Union of nullable types
  fixture.parse(
    r#"local f: a? | b? | c? | d? | e? | f? | g? | h?
  "#,
    &ParseOptions::default(),
  );

  // Test 3: Exceeded type length limit
  fixture.match_parse_error(
    "local t: a & b & c & d & e & f & g & h & i & j & nil",
    "Exceeded allowed type length; simplify your type annotation to make the code compile",
    None,
  );
}
#[test]
fn parser_can_parse_leading_ampersand_intersections_successfully() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  fixture.parse("type A = & { string } & { number }", &ParseOptions::new());
}
#[test]
fn parser_can_parse_leading_bar_unions_successfully() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from("type A = | \"Hello\" | \"World\"");
  let options = ParseOptions::new();
  let _result = fixture.parse_ex(&source, &options);
}
#[test]
fn parser_cannot_use_as_variable_name() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from(
    "local @blah = 3
  ",
  );
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let result = fixture.try_parse(&source, &opts);

  assert!(!result.errors.is_empty());
}
#[test]
fn parser_cannot_write_multiple_values_in_type_groups() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "type F = ((string, number))",
    "Expected '->' when parsing function type, got ')'",
    None,
  );
  fixture.match_parse_error(
    "type F = () -> ((string, number))",
    "Expected '->' when parsing function type, got ')'",
    None,
  );
}
#[test]
fn parser_capture_broken_comment() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  let source = String::from("\n        local a = \"test\"\n\n        --[[broken!\n    ");
  let mut options = ParseOptions::new();
  options.capture_comments = true;

  let result = fixture.try_parse(&source, &options);

  assert_eq!(result.comment_locations.len(), 1);
  let expected_location = Location::new(Position::new(3, 8), Position::new(4, 4));
  assert_eq!(result.comment_locations[0].location, expected_location);
}
#[test]
fn parser_capture_broken_comment_at_the_start_of_the_file() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fix = Fixture::default();
  let code = String::from("\n        --[[\n    ");
  let mut options = ParseOptions::new();
  options.capture_comments = true;

  let result = fix.try_parse(&code, &options);

  assert_eq!(1, result.comment_locations.len());
  let expected_location = Location::new(Position::new(1, 8), Position::new(2, 4));
  assert_eq!(expected_location, result.comment_locations[0].location);
}
#[test]
fn parser_capture_comments() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from(
    "\n        --!strict\n\n        local a = 5 -- comment one\n        local b = 8 -- comment two\n        --[[\n            Multi line comment\n        ]]\n        local c = 'see'\n    ",
  );
  let mut options = ParseOptions::new();
  options.capture_comments = true;

  let result = fixture.parse_ex(&source, &options);

  assert!(result.errors.is_empty());
  assert_eq!(result.comment_locations.len(), 4);

  let loc0 = Location::new(Position::new(1, 8), Position::new(1, 17));
  assert_eq!(result.comment_locations[0].location, loc0);

  let loc1 = Location::new(Position::new(3, 20), Position::new(3, 34));
  assert_eq!(result.comment_locations[1].location, loc1);

  let loc2 = Location::new(Position::new(4, 20), Position::new(4, 34));
  assert_eq!(result.comment_locations[2].location, loc2);

  let loc3 = Location::new(Position::new(5, 8), Position::new(7, 10));
  assert_eq!(result.comment_locations[3].location, loc3);
}
#[test]
fn parser_class_declaration() {
  use ulua_ast::records::{
    ast_class_property::AstClassProperty, ast_expr_call::AstExprCall,
    ast_expr_global::AstExprGlobal, ast_stat_class::AstStatClass, ast_stat_expr::AstStatExpr,
    parse_options::ParseOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, PtrRef, as_node_at, elem},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src = String::from(
    "\n        class Point2\n            public x: number\n            public y: number\n        end\n        print(Point2)\n    ",
  );
  let res = fix.try_parse(&src, &ParseOptions::default());
  assert!(res.errors.is_empty());

  let root = res.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 2);

  let first = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  let first_name = first.name.as_ref_opt().expect("类名 AstLocal 必须存在");
  assert_eq!(first_name.name.as_str(), Some("Point2"));

  assert_eq!(first.members.size, 2);

  let m1 = elem(&first.members, 0)
    .get_if::<AstClassProperty>()
    .expect("members[0] 应为属性");
  assert_eq!(m1.name.as_str(), Some("x"));

  let m2 = elem(&first.members, 1)
    .get_if::<AstClassProperty>()
    .expect("members[1] 应为属性");
  assert_eq!(m2.name.as_str(), Some("y"));

  let second = as_node_at::<AstStatExpr, _>(&root.body, 1).expect("body[1] 应为 AstStatExpr");
  let call = second
    .expr
    .as_node::<AstExprCall>()
    .expect("expr 应为 AstExprCall");

  assert_eq!(call.args.size, 1);
  // cpp:3333 `print(Point2)` 的实参解析为 AstExprGlobal（类名不入局部作用域）。
  let global = as_node_at::<AstExprGlobal, _>(&call.args, 0).expect("args[0] 应为 AstExprGlobal");
  assert_eq!(global.name.as_str(), first_name.name.as_str());
}
// Source: `tests/Parser.test.cpp` `class_exported_open`：
// `open` 之后缺 `class` 关键字必须报 Incomplete statement。
#[test]
fn parser_class_exported_open() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let message = "Incomplete statement: expected a class definition after 'open'";

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    r#"
export open Animal
    public species: string
end
"#,
    message,
    None,
  );

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    r#"
open Animal
    public species: string
end
"#,
    message,
    None,
  );
}
// Source: `tests/Parser.test.cpp` `class_extends_basic`：
// 验证 `extends` 挂到 `AstStatClass::super_`、`open` 修饰符落位。
#[test]
fn parser_class_extends_basic() {
  use ulua_ast::records::{
    ast_expr_global::AstExprGlobal, ast_stat_class::AstStatClass, parse_options::ParseOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let res = fix.try_parse(
    r#"
open class Animal
    public species: string
end

class Cat extends Animal
    public meowMult: number
end
"#,
    &ParseOptions::default(),
  );
  assert!(res.errors.is_empty(), "{:?}", res.errors);

  let root = res.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 2);

  let animal = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  assert!(animal.super_.is_null(), "Animal 无基类");
  assert!(animal.open);
  assert!(!animal.exported);

  let cat = as_node_at::<AstStatClass, _>(&root.body, 1).expect("body[1] 应为 AstStatClass");
  assert!(!cat.super_.is_null(), "Cat 应有基类引用");
  assert!(!cat.open);
  let super_global = cat
    .super_
    .as_node::<AstExprGlobal>()
    .expect("super 应为 AstExprGlobal");
  assert_eq!(super_global.name.as_str(), Some("Animal"));
}
// Source: `tests/Parser.test.cpp` `no_class_after_open`：
// `export open class` 合法，exported 与 open 两个修饰符都要落位。
#[test]
fn parser_no_class_after_open() {
  use ulua_ast::records::{ast_stat_class::AstStatClass, parse_options::ParseOptions};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::as_node_at, records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let res = fix.try_parse(
    r#"
export open class Animal
    public species: string
end
"#,
    &ParseOptions::default(),
  );
  assert!(res.errors.is_empty(), "{:?}", res.errors);

  let root = res.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 1);

  let animal = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  assert!(animal.super_.is_null());
  assert!(animal.exported);
  assert!(animal.open);
}
#[test]
fn parser_class_indexer() {
  use ulua_ast::records::{
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_type_reference::AstTypeReference,
    parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, PtrRef, as_node_at},
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::default();
  let parse_result = fixture.parse_ex(
    "declare extern type Foo with\n\
               prop: boolean\n\
               [string]: number\n\
               end",
    &ParseOptions::default(),
  );
  let root = parse_result.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 1);

  let declared_extern_type = as_node_at::<AstStatDeclareExternType, _>(&root.body, 0)
    .expect("body[0] 应为 AstStatDeclareExternType");
  let indexer = declared_extern_type
    .indexer
    .as_ref_opt()
    .expect("indexer 必须存在");

  let index_type_ref = indexer
    .index_type
    .as_node::<AstTypeReference>()
    .expect("indexType 应为 AstTypeReference");
  assert_eq!(index_type_ref.name.as_str(), Some("string"));

  let result_type_ref = indexer
    .result_type
    .as_node::<AstTypeReference>()
    .expect("resultType 应为 AstTypeReference");
  assert_eq!(result_type_ref.name.as_str(), Some("number"));

  let error_parse_result = fixture.match_parse_error(
    "declare extern type Foo with\n\
               [string]: number\n\
               -- can only have one indexer\n\
               [number]: number\n\
               end",
    "Cannot have more than one indexer on an extern type",
    None,
  );
  let error_root = error_parse_result.root.as_ref_opt().expect("根块必须存在");
  assert_eq!(error_root.body.len(), 1);

  let error_declared_extern_type = as_node_at::<AstStatDeclareExternType, _>(&error_root.body, 0)
    .expect("body[0] 应为 AstStatDeclareExternType");
  assert!(error_declared_extern_type.indexer.as_ref_opt().is_some());
}
#[test]
fn parser_class_is_still_contextual() {
  use ulua_ast::records::{ast_stat_local::AstStatLocal, parse_options::ParseOptions};
  use ulua_common::fflag::DebugLuauUserDefinedClasses;
  use ulua_unit_test::{
    functions::ast_node_ref::{as_node_at, deref_at},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _scoped_flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

  let mut fixture = Fixture::default();
  // The C++ R"(...)" DELIMITER parens leaked into the raw-string content here
  // (stray leading '(' / trailing ')' = invalid Lua). Drop them.
  let source = "
          local class = 42
          print(class)
      ";

  let result = fixture.try_parse(source, &ParseOptions::default());

  assert!(result.errors.is_empty());
  let root = result.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 2);
  let locals = as_node_at::<AstStatLocal, _>(&root.body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!(locals.vars.size, 1);
  let var_name = deref_at(&locals.vars, 0).expect("vars[0] 必须存在");
  assert_eq!(var_name.name.as_str(), Some("class"));
}
#[test]
fn parser_class_method_missing_end_error() {
  use ulua_common::fflag::DebugLuauUserDefinedClasses;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "\n        class Foo\n            function bar()\n                local x = 1\n    ",
    "Expected 'end' (to close 'function' at line 3), got <eof>",
    None,
  );
}
#[test]
fn parser_class_method_properties() {
  use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
  use ulua_unit_test::{
    functions::ast_node_ref::{PtrRef, as_node_at},
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::default();
  let result1 = fixture.match_parse_error(

      "\n        declare extern type Foo with\n            -- method's first parameter must be 'self'\n            function method(foo: number)\n            function method2(self)\n        end\n        "
,
    "'self' must be present as the unannotated first parameter",
    None,
  );

  let root1 = result1.root.as_ref_opt().expect("根块必须存在");
  assert_eq!(1, root1.body.len());

  let klass = as_node_at::<AstStatDeclareExternType, _>(&root1.body, 0)
    .expect("body[0] 应为 AstStatDeclareExternType");
  assert_eq!(2, klass.props.size);

  let mut fixture2 = Fixture::default();
  let result2 = fixture2.match_parse_error(

      "\n        declare extern type Foo with\n            function method(self, foo)\n            function method2()\n        end\n        "
,
    "All declaration parameters aside from 'self' must be annotated",
    None,
  );

  let root2 = result2.root.as_ref_opt().expect("根块必须存在");
  assert_eq!(1, root2.body.len());

  let klass2 = as_node_at::<AstStatDeclareExternType, _>(&root2.body, 0)
    .expect("body[0] 应为 AstStatDeclareExternType");
  assert_eq!(2, klass2.props.size);
}
#[test]
fn parser_class_parse_errors() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();

  // C++ only checks that the parser does not crash on these malformed inputs.
  let opts = ParseOptions::default();
  fix.try_parse(" class Hello ", &opts);
  fix.try_parse(" class Hello public ", &opts);
  fix.try_parse(" class Hello public x ", &opts);
  fix.try_parse(" class Hello public x: ", &opts);
  fix.try_parse(" class Hello public x: number ", &opts);
  fix.try_parse(" class Hello end ", &opts);
  fix.try_parse(" class Hello public end ", &opts);
  fix.try_parse(" class Hello private end ", &opts);
  fix.try_parse(" class Hello public x end ", &opts);
  fix.try_parse(" class Hello public x: end ", &opts);
  fix.try_parse(" class Hello public x: number end ", &opts);
  fix.try_parse(" class Hello public x: number public x: string end ", &opts);
  fix.try_parse(" class Hello public x: number function x() end end ", &opts);
}
#[test]
fn parser_class_public_function() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src =
    String::from("\n        class Foo\n            public function bar() end\n        end\n    ");
  let result = fix.try_parse(&src, &ParseOptions::default());
  assert!(result.errors.is_empty());
}
#[test]
fn parser_class_recovery_error_in_property_type() {
  use ulua_ast::records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, parse_options::ParseOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{as_node_at, elem},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src = String::from(
    "\nclass Foo\n    public x: { a: number\n    public y: number\n    function bar() end\nend\n    ",
  );
  let result = fix.try_parse(&src, &ParseOptions::default());
  assert!(!result.errors.is_empty());

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 1);
  let cls = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  assert_eq!(cls.members.size, 3);

  let m1 = elem(&cls.members, 0)
    .get_if::<AstClassProperty>()
    .expect("members[0] 应为属性");
  assert_eq!(m1.name.as_str(), Some("x"));

  let m2 = elem(&cls.members, 1)
    .get_if::<AstClassProperty>()
    .expect("members[1] 应为属性");
  assert_eq!(m2.name.as_str(), Some("y"));

  let m3 = elem(&cls.members, 2)
    .get_if::<AstClassMethod>()
    .expect("members[2] 应为方法");
  assert_eq!(m3.function_name.as_str(), Some("bar"));
}
#[test]
fn parser_class_recovery_invalid_body_token() {
  use ulua_ast::records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, parse_options::ParseOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{as_node_at, elem},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src =
    String::from("\nclass Foo\n    public x: number\n    blah\n    function bar() end\nend\n    ");
  let result = fix.try_parse(&src, &ParseOptions::default());
  assert!(!result.errors.is_empty());

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 1);
  let cls = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  assert_eq!(cls.members.size, 2);

  let m1 = elem(&cls.members, 0)
    .get_if::<AstClassProperty>()
    .expect("members[0] 应为属性");
  assert_eq!(m1.name.as_str(), Some("x"));

  let m2 = elem(&cls.members, 1)
    .get_if::<AstClassMethod>()
    .expect("members[1] 应为方法");
  assert_eq!(m2.function_name.as_str(), Some("bar"));
}
#[test]
fn parser_class_recovery_public_no_name_and_invalid_body_token() {
  use ulua_ast::records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, parse_options::ParseOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{PtrRef, as_node_at, elem},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src = String::from(
    "\nclass Foo\n    public propone\n    function methodone()\n    end\n\n    function methodtwo()\n        blah\n\n    public proptwo\n\n    function methodthree()\n    end\nend\n    ",
  );
  let result = fix.try_parse(&src, &ParseOptions::default());
  assert!(!result.errors.is_empty());

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 1);
  let cls = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  assert_eq!(
    cls
      .name
      .as_ref_opt()
      .expect("类名 AstLocal 必须存在")
      .name
      .as_str(),
    Some("Foo")
  );

  assert_eq!(cls.members.size, 3);

  let m1 = elem(&cls.members, 0)
    .get_if::<AstClassProperty>()
    .expect("members[0] 应为属性");
  assert_eq!(m1.name.as_str(), Some("propone"));

  let m2 = elem(&cls.members, 1)
    .get_if::<AstClassMethod>()
    .expect("members[1] 应为方法");
  assert_eq!(m2.function_name.as_str(), Some("methodone"));

  let m3 = elem(&cls.members, 2)
    .get_if::<AstClassMethod>()
    .expect("members[2] 应为方法");
  assert_eq!(m3.function_name.as_str(), Some("methodtwo"));
}
#[test]
fn parser_class_self_cannot_be_annotated() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  fix.match_parse_error(
    "\n        class Foobar\n            function baz(self: number, foobar)\n        end\n    ",
    "The 'self' parameter cannot have a type annotation",
    None,
  );
}
#[test]
fn parser_classes_can_be_shadowed_by_locals() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  // This is legal: exactly one class with a given name, but it may be shadowed by a local.
  let src = String::from(
    "\n        class Foobar\n        end\n\n        -- This is legal: the rule is that there is exactly one class with a\n        -- given name, but we can shadow it with a local.\n        local Foobar\n    ",
  );
  let result = fix.try_parse(&src, &ParseOptions::default());
  assert!(result.errors.is_empty());
}
#[test]
fn parser_classes_can_have_members_named_public() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src = String::from(
    "\n        class Foobar\n            function public() end\n        end\n\n        class Barbaz\n            public public\n        end\n    ",
  );
  let result = fix.try_parse(&src, &ParseOptions::default());
  assert!(result.errors.is_empty());
}
#[test]
fn parser_classes_can_interleave_methods_and_properties() {
  use ulua_ast::records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, parse_options::ParseOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{PtrRef, as_node_at, elem},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let src = String::from(
    "\n        class Student\n            public name: string\n\n            function getname(self): string\n                return self.name:upper()\n            end\n\n            public year: number\n\n            function getyear(self): number\n                assert(self.year >= 1900 and self.year < 2100)\n                return self.year\n            end\n        end\n    ",
  );
  let res = fix.try_parse(&src, &ParseOptions::default());
  assert!(res.errors.is_empty());

  let root = res.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 1);
  let cls = as_node_at::<AstStatClass, _>(&root.body, 0).expect("body[0] 应为 AstStatClass");
  assert_eq!(
    cls
      .name
      .as_ref_opt()
      .expect("类名 AstLocal 必须存在")
      .name
      .as_str(),
    Some("Student")
  );

  assert_eq!(cls.members.size, 4);

  let m1 = elem(&cls.members, 0)
    .get_if::<AstClassProperty>()
    .expect("members[0] 应为属性");
  assert_eq!(m1.name.as_str(), Some("name"));

  let m2 = elem(&cls.members, 1)
    .get_if::<AstClassMethod>()
    .expect("members[1] 应为方法");
  assert_eq!(m2.function_name.as_str(), Some("getname"));

  let m3 = elem(&cls.members, 2)
    .get_if::<AstClassProperty>()
    .expect("members[2] 应为属性");
  assert_eq!(m3.name.as_str(), Some("year"));

  let m4 = elem(&cls.members, 3)
    .get_if::<AstClassMethod>()
    .expect("members[3] 应为方法");
  assert_eq!(m4.function_name.as_str(), Some("getyear"));
}
#[test]
fn parser_classes_can_only_have_functions_and_properties() {
  use ulua_common::fflag::DebugLuauUserDefinedClasses;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

  let source = r#"
          class Bicycle
              while true do
                  cycle()
              end
          end
      "#;

  let expected_message = "Only class properties and functions can be declared within a class";

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.match_parse_error(source, expected_message, None);
}
#[test]
fn parser_classes_cannot_be_shadowed_by_classes() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  fix.match_parse_error(
    "\n        class Foobar\n        end\n\n        class Foobar\n        end\n    ",
    "A class named 'Foobar' has already been declared in this module",
    None,
  );
}
#[test]
fn parser_classes_cannot_be_shadowed_by_classes_with_local_between() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  fix.match_parse_error(

            "\n        class Foobar\n        end\n\n        local Foobar\n\n        class Foobar\n        end\n    "
,

            "A class named 'Foobar' has already been declared in this module"
,
        None,
    );
}
#[test]
fn parser_classes_nested_and_repeated() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _debug_luau_user_defined_classes =
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let code = r#"
          class Foo
          end
          if true then
              class Foo
              end
          end
      "#;

  let result = fix.try_parse(code, &ParseOptions::default());

  assert_eq!(result.errors.len(), 1);
  assert_eq!(
    result.errors[0].get_message(),
    "Cannot declare class 'Foo' inside another statement or expression"
  );
}
#[test]
fn parser_classes_only_work_at_top_level() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  fix.match_parse_error(

            "\n            return function ()\n                class DynamicPlayer\n                    public level: number\n                end\n                return DynamicPlayer\n            end\n        "
,

            "Cannot declare class 'DynamicPlayer' inside another statement or expression"
,
        None,
    );

  fix.match_parse_error(

            "\n            if math.random() > 0.5 then\n                class DynamicPlayer\n                    public level: number\n                end\n            end\n        "
,

            "Cannot declare class 'DynamicPlayer' inside another statement or expression"
,
        None,
    );
}
#[test]
fn parser_classes_work_after_other_statements() {
  use ulua_ast::records::{ast_stat_class::AstStatClass, parse_options::ParseOptions};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::ast_node_ref::{PtrRef, as_node_at},
    records::fixture::Fixture,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // 线程本地覆盖，函数结束恢复；`set(true)` 写进程全局，污染并行测试线程
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fixture = Fixture::default();
  let source = r#"
          if math.random() > 0.5 then
              print("I am a test case!")
          end

          class Player
              public health: number
          end
      "#;

  let result = fixture.try_parse(source, &ParseOptions::new());

  assert_eq!(result.errors.len(), 0);

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(root.body.len(), 2);
  let cls = as_node_at::<AstStatClass, _>(&root.body, 1).expect("body[1] 应为 AstStatClass");
  // cpp `CHECK_EQ(cls->name->name, "Player")` compares AstName content (strcmp),
  // not the interned pointer.
  assert_eq!(
    cls
      .name
      .as_ref_opt()
      .expect("类名 AstLocal 必须存在")
      .name
      .as_str(),
    Some("Player")
  );
}
#[test]
fn parser_complex_union_in_generic_ty() {
  use ulua_ast::records::{
    ast_stat_local::AstStatLocal, ast_type_reference::AstTypeReference,
    ast_type_union::AstTypeUnion, parse_options::ParseOptions,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{NodePtr, as_node_at, deref_at, elem},
    records::fixture::Fixture,
  };

  let mut fix = Fixture::default();
  let code = String::from(
    "type X<T> = T\n\
           local x: X<\n\
               | number\n\
               | boolean\n\
               | string\n\
           >",
  );

  let result = fix.try_parse(&code, &ParseOptions::new());
  assert_eq!(result.errors.len(), 0);

  let block = result.root_block().expect("根块必须存在");
  assert_eq!(block.body.len(), 2);

  let assignment =
    as_node_at::<AstStatLocal, _>(&block.body, 1).expect("body[1] 应为 AstStatLocal");
  assert_eq!(assignment.vars.size, 1);
  assert_eq!(assignment.values.size, 0);

  let var_0 = deref_at(&assignment.vars, 0).expect("vars[0] 必须存在");
  assert_eq!(var_0.name.as_str(), Some("x"));

  let generic_ty = var_0
    .annotation
    .as_node::<AstTypeReference>()
    .expect("annotation 应为 AstTypeReference");
  assert_eq!(generic_ty.parameters.size, 1);

  let param_0 = elem(&generic_ty.parameters, 0);
  assert!(param_0.as_type().is_some());
  let union_ty = param_0
    .as_type()
    .and_then(|ty| ty.as_node::<AstTypeUnion>())
    .expect("参数应为 AstTypeUnion");
  assert_eq!(union_ty.types.size, 3);

  // cpp 用 `const char*` 逐个与 AstName 比对（strcmp 内容语义）。
  let expected_types: [&str; 3] = ["number", "boolean", "string"];
  for (i, &expected) in expected_types.iter().enumerate() {
    let ty_ref = as_node_at::<AstTypeReference, _>(&union_ty.types, i).expect("types[i] 应为引用");
    assert_eq!(ty_ref.name.as_str(), Some(expected));
  }
}
#[test]
fn parser_const_shadow() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let source = String::from(
    "const a = 42\n\
           const a = 43\n\
           \n\
           do\n\
               const a = 44\n\
               do\n\
                   local a = 44.1\n\
                   do\n\
                       const a = 44.2\n\
                   end\n\
                   a = 44.3\n\
               end\n\
           end\n\
           \n\
           function f()\n\
               const a = 45\n\
               local a = 46\n\
               return function(x) a = x end\n\
           end",
  );
  let _stat = fixture.parse(&source, &ParseOptions::default());
}
#[test]
fn parser_continue_not_last_error() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "while true do continue print(5) end",
    "Expected 'end' (to close 'do' at column 12), got 'print'",
    None,
  );
}
#[test]
fn parser_debugnoinline_not_allowed_without_flag() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture,
  };

  let mut fix = Fixture::default();
  let result = fix.try_parse(
    "\n@debugnoinline\nlocal function hello(x, y)\n    return x + y\nend",
    &ParseOptions::default(),
  );

  check_first_error_for_attributes(
    &result.errors,
    1,
    Location::new(Position::new(1, 0), Position::new(1, 14)),
    "Invalid attribute '@debugnoinline'",
  );
}
#[test]
fn parser_disallow_double_underscore_properties() {
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  fix.match_parse_error(
    "\n        class Foo\n            public __add: any\n        end\n    ",
    "Class properties cannot start with '__'",
    None,
  );
}
#[test]
fn parser_do_block_end_location_is_after_end_token() {
  use ulua_ast::records::{
    ast_stat_block::AstStatBlock, location::Location, parse_options::ParseOptions,
    position::Position,
  };
  use ulua_unit_test::{functions::ast_node_ref::as_node_at, records::fixture::Fixture};

  let mut fixture = Fixture::default();
  let stat = fixture.parse(
    "\n        do\n            local x = 1\n        end\n    ",
    &ParseOptions::default(),
  );

  assert_eq!(stat.body.len(), 1);
  let block = as_node_at::<AstStatBlock, _>(&stat.body, 0).expect("body[0] 应为嵌套块");

  let expected_location = Location::new(Position::new(1, 8), Position::new(3, 11));
  assert_eq!(block.base.base.location, expected_location);
}
#[test]
fn parser_do_block_with_no_end() {
  use ulua_ast::records::{ast_stat_block::AstStatBlock, parse_options::ParseOptions};
  use ulua_unit_test::{functions::ast_node_ref::as_node_at, records::fixture::Fixture};

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from("do\n");
  let options = ParseOptions::new();
  let result = fixture.try_parse(&source, &options);

  assert_eq!(1, result.errors.len());

  let root = result.root_block().expect("根块必须存在");
  let stat0_block = as_node_at::<AstStatBlock, _>(&root.body, 0).expect("body[0] 应为嵌套块");
  assert!(!stat0_block.has_end);
}
#[test]
fn parser_do_end_block_with_cst() {
  use ulua_ast::records::{
    ast_stat_block::AstStatBlock, cst_stat_do::CstStatDo, parse_options::ParseOptions,
    position::Position,
  };
  use ulua_unit_test::{
    functions::ast_node_ref::{as_node_at, node_key},
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from("\n        do\n            local hello = \"world\"\n        end\n    ");
  let mut parse_options = ParseOptions::new();
  parse_options.store_cst_data = true;

  let result = fixture.parse_ex(&source, &parse_options);
  let root = result.root_block().expect("根块必须存在");

  // cpp: 只有 do ... end 块才建 CST 节点，模块根块不应有
  assert!(result.cst_node_map.find(&node_key(root)).is_none());
  assert_eq!(1, root.body.len());

  let do_block = as_node_at::<AstStatBlock, _>(&root.body, 0).expect("body[0] 应为 do 块");
  let do_block_cst_node = *result
    .cst_node_map
    .find(&node_key(do_block))
    .expect("do 块应有 CST 节点");
  let do_block_cst = do_block_cst_node
    .as_cst::<CstStatDo>()
    .expect("应为 CstStatDo");

  assert_eq!(Position::new(2, 12), do_block_cst.stats_start_position);
  assert_eq!(Position::new(3, 8), do_block_cst.end_position);
}
#[test]
fn parser_do_not_hang_on_incomplete_attribute_list() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture,
  };

  let mut fix = Fixture::default();

  let code1 = r#"
@[]
function hello(x, y)
    return x + y
end"#;
  let result1 = fix.try_parse(code1, &ParseOptions::default());
  let expected_location1 = Location::new(Position::new(1, 0), Position::new(1, 3));
  let expected_message1 = "Attribute list cannot be empty";
  check_first_error_for_attributes(&result1.errors, 1, expected_location1, expected_message1);

  let code2 = r"@[";
  let result2 = fix.try_parse(code2, &ParseOptions::default());
  let expected_location2 = Location::new(Position::new(0, 2), Position::new(0, 2));
  let expected_message2 = "Expected identifier when parsing attribute name, got <eof>";
  check_first_error_for_attributes(&result2.errors, 1, expected_location2, expected_message2);

  let code3 = "@[\n        function foo() end\n    ]";
  let result3 = fix.try_parse(code3, &ParseOptions::default());
  let expected_location3 = Location::new(Position::new(1, 8), Position::new(1, 16));
  let expected_message3 = "Expected identifier when parsing attribute name, got 'function'";
  check_first_error_for_attributes(&result3.errors, 1, expected_location3, expected_message3);

  let code4 = "@[deprecated\n        local function foo() end\n    ]";
  let result4 = fix.try_parse(code4, &ParseOptions::default());
  let expected_location4 = Location::new(Position::new(1, 8), Position::new(1, 13));
  let expected_message4 = "Expected ']' (to close '@[' at line 1), got 'local'";
  check_first_error_for_attributes(&result4.errors, 1, expected_location4, expected_message4);
}
#[test]
fn parser_dont_parse_attribute_on_argument_non_function() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from(
    "\nlocal function invoker(f, y)\n    return f(y)\nend\n\ninvoker(function(x) return (x + 2) end, @checked 1)\n",
  );
  let options = ParseOptions::new();
  let result = fixture.try_parse(&source, &options);

  let expected_location = Location::new(Position::new(5, 40), Position::new(5, 48));
  let expected_message = "Expected 'function' declaration after attribute, but got '1' instead";

  check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
}
#[test]
fn parser_dont_parse_attributes_on_non_function_stat() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let mut fix = Fixture::default();

  let pr1 = fix.try_parse(
    "\n@checked\nif a<0 then a = 0 end",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(2, 0), Position::new(2, 2));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'if' instead";
  check_first_error_for_attributes(&pr1.errors, 1, expected_location, expected_message);

  let pr2 = fix.try_parse(
    "\nlocal i = 1\n@checked\nwhile a[i] do\n    print(a[i])\n    i = i + 1\nend",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(3, 0), Position::new(3, 5));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'while' instead";
  check_first_error_for_attributes(&pr2.errors, 1, expected_location, expected_message);

  let pr3 = fix.try_parse(
        "\n@checked\ndo\n    local a2 = 2*a\n    local d = sqrt(b^2 - 4*a*c)\n    x1 = (-b + d)/a2\n    x2 = (-b - d)/a2\nend",
        &ParseOptions::default(),
    );
  let expected_location = Location::new(Position::new(2, 0), Position::new(2, 2));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'do' instead";
  check_first_error_for_attributes(&pr3.errors, 1, expected_location, expected_message);

  let pr4 = fix.try_parse(
    "\n@checked\nfor i=1,10 do print(i) end\n",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(2, 0), Position::new(2, 3));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'for' instead";
  check_first_error_for_attributes(&pr4.errors, 1, expected_location, expected_message);

  let pr5 = fix.try_parse(
    "\n@checked\nrepeat\n    line = io.read()\nuntil line ~= \"\"\n",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(2, 0), Position::new(2, 6));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'repeat' instead";
  check_first_error_for_attributes(&pr5.errors, 1, expected_location, expected_message);

  let pr6 = fix.try_parse("\n@checked\nlocal x = 10\n", &ParseOptions::default());
  let expected_location = Location::new(Position::new(2, 6), Position::new(2, 7));
  check_first_error_for_attributes(
    &pr6.errors,
    1,
    expected_location,
    "Expected 'function' after local declaration with attribute, but got 'x' instead",
  );

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}};
  // These live through pr7..pr9; without LuauExportValueSyntax the `export local`
  // case below takes a different parse path and reports a different error.
  let _sff_export = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);

  let pr7 = fix.try_parse(
    "\n@checked\nexport local x = 10\n",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(2, 7), Position::new(2, 12));
  check_first_error_for_attributes(
    &pr7.errors,
    1,
    expected_location,
    "Expected 'function' after export declaration with attribute, but got 'local' instead",
  );

  let pr8 = fix.try_parse(
    "\nlocal i = 1\nwhile a[i] do\n    if a[i] == v then @checked break end\n    i = i + 1\nend\n",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(3, 31), Position::new(3, 36));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'break' instead";
  check_first_error_for_attributes(&pr8.errors, 1, expected_location, expected_message);

  let pr9 = fix.try_parse(
    "\nfunction foo1 () @checked return 'a' end\n",
    &ParseOptions::default(),
  );
  let expected_location = Location::new(Position::new(1, 26), Position::new(1, 32));
  let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'return' instead";
  check_first_error_for_attributes(&pr9.errors, 1, expected_location, expected_message);
}
#[test]
fn parser_dont_parse_attributes_on_non_function_type_declarations() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture,
  };

  let mut fixture = Fixture::fixture_bool(false);
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let source1 = String::from("\n@checked declare foo: number\n");
  let result1 = fixture.try_parse(&source1, &opts);

  let expected_location1 = Location::new(Position::new(1, 17), Position::new(1, 20));
  let expected_message1 =
    "Expected a function type declaration after attribute, but got 'foo' instead";
  check_first_error_for_attributes(&result1.errors, 1, expected_location1, expected_message1);

  let source2 = String::from(
    "\n@checked declare extern type Foo with\n    prop: number\n    function method(self, foo: number): string\nend",
  );
  let result2 = fixture.try_parse(&source2, &opts);

  let expected_location2 = Location::new(Position::new(1, 17), Position::new(1, 23));
  let expected_message2 =
    "Expected a function type declaration after attribute, but got 'extern' instead";
  check_first_error_for_attributes(&result2.errors, 1, expected_location2, expected_message2);

  let source3 = String::from("\ndeclare bit32: {\n    band: @checked number\n})");
  let result3 = fixture.try_parse(&source3, &opts);

  let expected_location3 = Location::new(Position::new(2, 19), Position::new(2, 25));
  let expected_message3 = "Expected '(' when parsing function parameters, got 'number'";
  check_first_error_for_attributes(&result3.errors, 1, expected_location3, expected_message3);
}
#[test]
fn parser_duplicate_class_methods() {
  use ulua_common::fflag::DebugLuauUserDefinedClasses;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "class Hello\n    function hi() end\n    function hi() end\nend",
    "Duplicate class member 'hi'",
    None,
  );
}
#[test]
fn parser_duplicate_unnamed_class_methods() {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fixture = Fixture::default();
  let source = "\nclass Hello\n    function () end\n    function () end\nend\n        ";

  let result = fixture.try_parse(source, &ParseOptions::default());

  assert_eq!(result.errors.len(), 3);
  assert_eq!(
    result.errors[2].get_message(),
    "Duplicate class member '%error-id%'"
  );
}
#[test]
fn parser_empty_attribute_name_is_not_allowed() {
  use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
  use ulua_unit_test::{
    functions::check_first_error_for_attributes::check_first_error_for_attributes,
    records::fixture::Fixture,
  };

  let mut fix = Fixture::default();
  let code = "\n@\nfunction hello(x, y)\n    return x + y\nend";

  let result = fix.try_parse(code, &ParseOptions::default());

  let expected_location = Location::new(Position::new(1, 0), Position::new(1, 1));
  let expected_message = "Attribute name is missing";

  check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
}
#[test]
fn parser_empty_function_type_error_recovery() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::default();

  // C++ uses `parse(...)` (which throws ParseErrors) and checks the first error
  // message. The port called the unrelated `parseType` frontend stub and ignored
  // the result entirely. Use `match_parse_error`, which parses and asserts the
  // first error message.

  // Case 1: empty `()` in a union — special-cased error message.
  fixture.match_parse_error(
    "\ntype Fn = (\n    any,\n    string | number | ()\n) -> any\n",
    "Expected '->' after '()' when parsing function type; did you mean 'nil'?",
    None,
  );

  // Case 2: arguments present, no special case.
  fixture.match_parse_error(
    "type Fn = (any, string | number | (number, number)) -> any",
    "Expected '->' when parsing function type, got ')'",
    None,
  );

  // Case 3: generic arguments present, no special case.
  fixture.match_parse_error(
    "type Fn = (any, string | number | <a>()) -> any",
    "Expected '->' when parsing function type, got ')'",
    None,
  );

  // Case 4: variadic generic arguments present, no special case.
  fixture.match_parse_error(
    "type Fn = (any, string | number | <a...>()) -> any",
    "Expected '->' when parsing function type, got ')'",
    None,
  );
}
