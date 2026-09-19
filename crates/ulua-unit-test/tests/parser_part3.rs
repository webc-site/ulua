//! Port of `cpp/tests/Parser.test.cpp`（`AllocatorTests` + `ParserTests` 的一部分）。
//!
//! 断言逐条对应 cpp 的 `REQUIRE_*` / `CHECK_*`；AST 下转与解引用统一走
//! [`ulua_unit_test::functions::ast_node_ref`]（`NodePtr::as_node` /
//! `PtrRef::as_ref_opt` / `as_node_at` / `deref_at` / `elem`），调用点零 `unsafe`、零裸指针判空。

use core::{ffi::c_char, mem::align_of};

use ulua_analysis::functions::parse_mode::parse_mode;
use ulua_ast::{
  enums::{mode::Mode, type_lexer::Type},
  records::{
    allocator::Allocator, ast_attr::AstAttrType, ast_expr_call::AstExprCall,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_index_name::AstExprIndexName,
    ast_expr_table::AstExprTable, ast_name_table::AstNameTable, ast_stat_class::AstStatClass,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional, ast_type_reference::AstTypeReference,
    ast_type_table::AstTypeTable, ast_type_union::AstTypeUnion, lexeme::Lexeme, lexer::Lexer,
    location::Location, parse_error::ParseError, parse_options::ParseOptions, parser::Parser,
    position::Position,
  },
};
use ulua_common::fflag;
use ulua_unit_test::{
  functions::{
    ast_node_ref::{NodePtr, PtrRef, as_node_at, deref_at, elem},
    check_attribute::check_attribute,
    check_first_error_for_attributes::check_first_error_for_attributes,
    string_at_location::string_at_location,
  },
  records::fixture::Fixture,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

/// cpp `REQUIRE(1 == result.errors.size())` + `CHECK_*` 首条错误定位/消息的前奏。
fn assert_single_error(errors: &[ParseError], location: Location, message: &str) {
  assert_eq!(1, errors.len());
  assert_eq!(&location, errors[0].get_location());
  assert_eq!(message, errors[0].get_message());
}

/// `Location{{l, c}, {l, c}}` 的简写，对应 cpp 同名构造。
fn loc(begin: (u32, u32), end: (u32, u32)) -> Location {
  Location::new(Position::new(begin.0, begin.1), Position::new(end.0, end.1))
}

// ---------------------------------------------------------------- AllocatorTests

/// cpp `TEST_CASE("initial_double_is_aligned")`。
#[test]
fn parser_initial_double_is_aligned() {
  let mut alloc = Allocator::new();
  let one = alloc.alloc::<f64>(0.0);
  assert_eq!(0, (one as usize) & (align_of::<f64>() - 1));
}

/// cpp `TEST_CASE("moved_out_Allocator_can_still_be_used")`。
#[test]
fn parser_moved_out_allocator_can_still_be_used() {
  let mut outer = Allocator::new();
  let _inner = Allocator::allocator_allocator(&mut outer);

  // NOLINTNEXTLINE(bugprone-use-after-move) -- verifying moved-from state
  let i = outer.alloc::<i32>(55);
  assert_eq!(&55, i.as_ref_opt().expect("moved-from allocator 仍可分配"));
}

// ------------------------------------------------------------------ ParserTests

/// cpp `TEST_CASE_FIXTURE(Fixture, "generic_type_list_recovery")`：
/// `parse` 抛出 `ParseErrors`，用例校验其中的两条错误（Rust 侧免 catch 直读 errors）。
#[test]
fn parser_generic_type_list_recovery() {
  let mut fix = Fixture::default();
  let code = "local function foo<T..., U>(a: U, ...: T...): (U, ...T) return a, ... end\n\
              return foo(1, 2 -- to check for a second error after recovery";

  let result = fix.try_parse(code, &ParseOptions::new());

  assert_eq!(2, result.errors.len());
  assert_eq!(
    "Generic types come before generic type packs",
    result.errors[0].get_message()
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "get_a_nice_error_when_there_is_an_extra_comma_\
/// at_the_end_of_a_function_argument_list")`。
#[test]
fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_argument_list() {
  let mut fixture = Fixture::default();
  let source = r"
        foo(a, b, c,)
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_single_error(
    &result.errors,
    loc((1, 20), (1, 21)),
    "Expected expression after ',' but got ')' instead",
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "get_a_nice_error_when_there_is_an_extra_comma_\
/// at_the_end_of_a_function_parameter_list")`。
#[test]
fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_parameter_list() {
  let mut fixture = Fixture::default();
  let source = r"
        export type VisitFn = (
            any,
            Array<TAnyNode | Array<TAnyNode>>, -- extra comma here
        ) -> any
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_single_error(
    &result.errors,
    loc((4, 8), (4, 9)),
    "Expected type after ',' but got ')' instead",
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "get_a_nice_error_when_there_is_an_extra_comma_\
/// at_the_end_of_a_generic_parameter_list")`。
#[test]
fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_generic_parameter_list() {
  let mut fixture = Fixture::default();
  let source = r"
        export type VisitFn = <A, B,>(a: A, b: B) -> ()
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_single_error(
    &result.errors,
    loc((1, 36), (1, 37)),
    "Expected type after ',' but got '>' instead",
  );

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(1, root.body.size);

  let alias = as_node_at::<AstStatTypeAlias, _>(&root.body, 0).expect("body[0] 应为 type alias");
  // cpp 走的是 alias 的右侧类型（`t->type`），即函数类型本身。
  let func_type = alias
    .type_ptr
    .as_node::<AstTypeFunction>()
    .expect("alias 类型应为函数类型");
  assert_eq!(2, func_type.generics.size);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "get_a_nice_error_when_there_is_no_comma_\
/// after_last_table_member")`。
#[test]
fn parser_get_a_nice_error_when_there_is_no_comma_after_last_table_member() {
  let mut fixture = Fixture::default();
  let source = r"
        local t = {
            first = 1

        local ok = true
        local good = ok == true
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_single_error(
    &result.errors,
    loc((4, 8), (4, 13)),
    "Expected '}' (to close '{' at line 2), got 'local'",
  );

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(3, root.body.size);

  // cpp `Luau::query<AstExprTable>(root)` 递归取首个表：这里是 `local t = {...}`
  // 的值，即 body[0]（AstStatLocal）.values[0]（AstQueryDsl 尚未移植）。
  let local = as_node_at::<AstStatLocal, _>(&root.body, 0).expect("body[0] 应为 local");
  let table = as_node_at::<AstExprTable, _>(&local.values, 0).expect("values[0] 应为表");
  assert_eq!(1, table.items.size);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "get_a_nice_error_when_there_is_no_comma_\
/// between_table_members")`。
#[test]
fn parser_get_a_nice_error_when_there_is_no_comma_between_table_members() {
  let mut fixture = Fixture::default();
  let source = r"
        local t = {
            first = 1
            second = 2,
            third = 3,
            fouth = 4,
        }
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_single_error(
    &result.errors,
    loc((3, 12), (3, 18)),
    "Expected ',' after table constructor element",
  );

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(1, root.body.size);

  let local = as_node_at::<AstStatLocal, _>(&root.body, 0).expect("body[0] 应为 local");
  let table = as_node_at::<AstExprTable, _>(&local.values, 0).expect("values[0] 应为表");
  assert_eq!(4, table.items.size);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "grouped_function_type")`。
#[test]
fn parser_grouped_function_type() {
  let mut fixture = Fixture::default();
  let source = r"
        type X<T> = T
        local x: X<(() -> ())?>
    ";

  let root = fixture.parse(source, &ParseOptions::new());
  assert_eq!(2, root.body.size);

  let assignment = as_node_at::<AstStatLocal, _>(&root.body, 1).expect("body[1] 应为 local");
  assert_eq!(1, assignment.vars.size);
  assert_eq!(0, assignment.values.size);

  let binding = deref_at(&assignment.vars, 0).expect("vars[0] 应为声明");
  assert_eq!(Some("x"), binding.name.as_str());

  let generic_ty = binding
    .annotation
    .as_node::<AstTypeReference>()
    .expect("标注应为类型引用");
  assert_eq!(1, generic_ty.parameters.size);

  let param_ty = elem(&generic_ty.parameters, 0);
  let union_ty = param_ty
    .r#type
    .as_node::<AstTypeUnion>()
    .expect("参数应为 union 类型");
  assert_eq!(2, union_ty.types.size);

  // cpp：`groupTy` 是 `(() -> ())`，`types[1]` 是 `?`。
  let group_ty = as_node_at::<AstTypeGroup, _>(&union_ty.types, 0).expect("types[0] 应为分组");
  assert!(group_ty.type_.as_node::<AstTypeFunction>().is_some());
  assert!(as_node_at::<AstTypeOptional, _>(&union_ty.types, 1).is_some());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "incomplete_method_call")`。
#[test]
fn parser_incomplete_method_call() {
  let source = r"
        function howdy()
            return game:
        end
    ";

  // Box 钉堆：AstNameTable/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(source, &mut names, &mut allocator, ParseOptions::new());

  let root = result.root.as_ref_opt().expect("根块必须存在");
  assert_eq!(1, root.body.size);

  let howdy = as_node_at::<AstStatFunction, _>(&root.body, 0).expect("body[0] 应为函数声明");
  let body = howdy
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .body
    .as_ref_opt()
    .expect("函数体必须存在");
  assert_eq!(1, body.body.size);
  assert!(as_node_at::<AstStatReturn, _>(&body.body, 0).is_some());

  assert!(howdy.base.base.location.end > body.base.base.location.end);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "incomplete_method_call_2")`。
#[test]
fn parser_incomplete_method_call_2() {
  let source = r"
        local game = { GetService=function(s) return 'hello' end }

        function a()
            game:a
        end
    ";

  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(source, &mut names, &mut allocator, ParseOptions::new());

  let root = result.root.as_ref_opt().expect("根块必须存在");
  assert_eq!(2, root.body.size);

  let howdy = as_node_at::<AstStatFunction, _>(&root.body, 1).expect("body[1] 应为函数声明");
  let body = howdy
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .body
    .as_ref_opt()
    .expect("函数体必须存在");
  assert_eq!(1, body.body.size);
  assert!(as_node_at::<AstStatError, _>(&body.body, 0).is_some());

  assert!(howdy.base.base.location.end > body.base.base.location.end);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "incomplete_method_call_still_yields_an_AstExprIndexName")`。
#[test]
fn parser_incomplete_method_call_still_yields_an_ast_expr_index_name() {
  let mut fixture = Fixture::default();
  let source = r"
        game:
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  let root = result.root_block().expect("根块必须存在");
  assert_eq!(1, root.body.size);

  let stat = as_node_at::<AstStatError, _>(&root.body, 0).expect("body[0] 应为错误语句");
  let expr = as_node_at::<AstExprError, _>(&stat.expressions, 0).expect("应为错误表达式");
  assert!(as_node_at::<AstExprIndexName, _>(&expr.expressions, 0).is_some());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "incomplete_statement_error")`。
#[test]
fn parser_incomplete_statement_error() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "fiddlesticks",
    "Incomplete statement: expected assignment or a function call",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "inner_and_outer_scope_of_functions_have_correct_end_position")`。
#[test]
fn parser_inner_and_outer_scope_of_functions_have_correct_end_position() {
  let mut fixture = Fixture::default();
  let source = r"
        local function foo()
            local x = 1
        end
    ";

  let stat = fixture.parse(source, &ParseOptions::new());
  assert_eq!(1, stat.body.size);

  let func =
    as_node_at::<AstStatLocalFunction, _>(&stat.body, 0).expect("body[0] 应为 local function");
  let body = func
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .body
    .as_ref_opt()
    .expect("函数体必须存在");

  assert_eq!(loc((1, 28), (3, 8)), body.base.base.location);
  assert_eq!(loc((1, 8), (3, 11)), func.base.base.location);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "intersection_of_two_function_types_if_no_returns")`。
#[test]
fn parser_intersection_of_two_function_types_if_no_returns() {
  let mut fixture = Fixture::default();
  let source = r"
        local f: (string) -> () & (number) -> ()
    ";

  let block = fixture.parse(source, &ParseOptions::default());
  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 local");
  let annotation = deref_at(&local.vars, 0)
    .expect("vars[0] 应为声明")
    .annotation;
  let intersection = annotation
    .as_node::<AstTypeIntersection>()
    .expect("标注应为交叉类型");

  assert!(as_node_at::<AstTypeFunction, _>(&intersection.types, 0).is_some());
  assert!(as_node_at::<AstTypeFunction, _>(&intersection.types, 1).is_some());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "intersection_of_two_function_types_if_two_or_more_returns")`。
#[test]
fn parser_intersection_of_two_function_types_if_two_or_more_returns() {
  let mut fixture = Fixture::default();
  let source = r"
        local f: (string) -> (string, number) & (number) -> (number, string)
    ";

  let block = fixture.parse(source, &ParseOptions::default());
  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 local");
  let annotation = deref_at(&local.vars, 0)
    .expect("vars[0] 应为声明")
    .annotation;
  let intersection = annotation
    .as_node::<AstTypeIntersection>()
    .expect("标注应为交叉类型");
  assert_eq!(2, intersection.types.size);

  assert!(as_node_at::<AstTypeFunction, _>(&intersection.types, 0).is_some());
  assert!(as_node_at::<AstTypeFunction, _>(&intersection.types, 1).is_some());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "invalid_escape_literals_get_reported_but_parsing_continues")`。
#[test]
fn parser_invalid_escape_literals_get_reported_but_parsing_continues() {
  let mut fixture = Fixture::default();
  let source = r#"
        local foo = "\xQQ"
        print(foo)
    "#;

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_single_error(
    &result.errors,
    loc((1, 20), (1, 26)),
    "String literal contains malformed escape sequence",
  );

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(2, root.body.size);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "invalid_type_forms")`。
#[test]
fn parser_invalid_type_forms() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "type A = (b: number)",
    "Expected '->' when parsing function type, got <eof>",
    None,
  );
  fixture.match_parse_error(
    "type P<T...> = () -> T... type B = P<(x: number, y: string)>",
    "Expected '->' when parsing function type, got '>'",
    None,
  );
  fixture.match_parse_error(
    "type F<T... = (a: string)> = (T...) -> ()",
    "Expected '->' when parsing function type, got '>'",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "invalid_user_defined_type_functions")`。
#[test]
fn parser_invalid_user_defined_type_functions() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "local foo = 1; type function bar() print(foo) end",
    "Type function cannot reference outer local 'foo'",
    None,
  );
  fixture.match_parse_error(
    "type function foo() local v1 = 1; type function bar() print(v1) end end",
    "Type function cannot reference outer local 'v1'",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "large_classes_example")`。
#[test]
fn parser_large_classes_example() {
  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  let source = r#"
        class PlayerStats
            public name: string
            public health: number
            public level: number

            function __init(self, name: string)
                self.name = name
                self.health = 100
                self.level = 1
            end

            -- Method
            function heal(self, amount: number)
                self.health = math.min(100, self.health + amount)
                print(self.name .. " healed to " .. self.health)
            end

            -- Metamethod for printing
            function __tostring(self)
                return self.name .. " (Level " .. self.level .. ") - Health: " .. self.health
            end
        end

        local player = PlayerStats.new("John Doe")
        print(player.name)
        player:heal(20)
        print(player.name)
    "#;

  let result = fix.try_parse(source, &ParseOptions::default());
  assert_eq!(0, result.errors.len());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "last_line_does_not_have_to_be_blank")`。
#[test]
fn parser_last_line_does_not_have_to_be_blank() {
  let mut fixture = Fixture::default();
  fixture.parse("-- print('hello')", &ParseOptions::new());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "leading_union_intersection_with_single_type_\
/// preserves_the_union_intersection_ast_node")`。
#[test]
fn parser_leading_union_intersection_with_single_type_preserves_the_union_intersection_ast_node() {
  let mut fixture = Fixture::default();
  let source = r"
        type Foo = | string
        type Bar = & number
    ";

  let block = fixture.parse(source, &ParseOptions::new());
  assert_eq!(2, block.body.size);

  let alias1 = as_node_at::<AstStatTypeAlias, _>(&block.body, 0).expect("body[0] 应为 alias");
  let union_type = alias1
    .type_ptr
    .as_node::<AstTypeUnion>()
    .expect("应为 union 类型");
  assert_eq!(1, union_type.types.size);

  let alias2 = as_node_at::<AstStatTypeAlias, _>(&block.body, 1).expect("body[1] 应为 alias");
  let intersection_type = alias2
    .type_ptr
    .as_node::<AstTypeIntersection>()
    .expect("应为交叉类型");
  assert_eq!(1, intersection_type.types.size);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "lex_broken_unicode")` 的单条断言组：
/// `next()` 后的词素必须是 BrokenUnicode + 指定码点与位置。
fn assert_broken_unicode(lexer: &mut Lexer, codepoint: u32, location: Location) {
  let lexeme = lexer.next_lexeme();
  assert_eq!(Type::BROKEN_UNICODE, lexeme.r#type);
  assert_eq!(codepoint, codepoint_of(lexeme));
  assert_eq!(location, lexeme.location);
}

/// cpp `lexeme.codepoint`：union 活跃分支由 `type == BrokenUnicode` 决定。
///
/// # Safety
/// `lexeme` 的词素类型为 `BROKEN_UNICODE`，此时 `data.codepoint` 为活跃分支。
fn codepoint_of(lexeme: &Lexeme) -> u32 {
  // SAFETY: 见函数级 # Safety
  unsafe { lexeme.data.codepoint }
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "lex_broken_unicode")`。
#[test]
fn parser_lex_broken_unicode() {
  // cpp "\xFF\xFE☃․"：两个坏字节 + 两个合法多字节码点。
  let test_input: &[u8] = b"\xff\xfe\xe2\x98\x83\xe2\x80\xa4";

  // Box 钉堆：AstNameTable/Lexer 捕获宿主地址，宿主移动即悬垂。
  let mut alloc = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut alloc);
  let mut lexer = Lexer::new(
    test_input.as_ptr() as *const c_char,
    test_input.len(),
    &mut names,
    Position::default(),
  );

  assert_broken_unicode(&mut lexer, 0, loc((0, 0), (0, 1)));
  assert_broken_unicode(&mut lexer, 0, loc((0, 1), (0, 2)));
  assert_broken_unicode(&mut lexer, 0x2603, loc((0, 2), (0, 5)));
  assert_broken_unicode(&mut lexer, 0x2024, loc((0, 5), (0, 8)));
  assert_eq!(Type::EOF, lexer.next_lexeme().r#type);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "local_with_annotation")`。
#[test]
fn parser_local_with_annotation() {
  let mut fixture = Fixture::default();
  let code = r#"
        local foo: string = "Hello Types!"
    "#;

  let block = fixture.parse(code, &ParseOptions::default());
  assert!(block.body.size > 0);

  let local = as_node_at::<AstStatLocal, _>(&block.body, 0).expect("body[0] 应为 local");
  assert_eq!(1, local.vars.size);

  let l = deref_at(&local.vars, 0).expect("vars[0] 应为声明");
  assert!(l.annotation.as_ref_opt().is_some(), "标注必须存在");

  assert_eq!(1, local.values.size);
  assert_eq!("foo", string_at_location(code, &l.location));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "missing_declaration_prop")`。
#[test]
fn parser_missing_declaration_prop() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    r"
        declare extern type Foo with
            a: number,
        end
    ",
    "Expected identifier when parsing property name, got ','",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "missing_default_type_pack_argument_after_variadic_type_parameter")`。
#[test]
fn parser_missing_default_type_pack_argument_after_variadic_type_parameter() {
  let mut fix = Fixture::default();
  let source = r"
        type Foo<T... = > = nil
    ";

  let result = fix.try_parse(source, &ParseOptions::new());
  assert_eq!(2, result.errors.len());

  assert_eq!(&loc((1, 23), (1, 25)), result.errors[0].get_location());
  assert_eq!("Expected type, got '>'", result.errors[0].get_message());

  assert_eq!(&loc((1, 23), (1, 24)), result.errors[1].get_location());
  assert_eq!(
    "Expected type pack after '=', got type",
    result.errors[1].get_message()
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "mixed_intersection_and_union_allowed_when_parenthesized")`。
#[test]
fn parser_mixed_intersection_and_union_allowed_when_parenthesized() {
  let mut fixture = Fixture::default();
  fixture.parse("type A = (number & string) | boolean", &ParseOptions::new());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "mixed_intersection_and_union_not_allowed")`。
#[test]
fn parser_mixed_intersection_and_union_not_allowed() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "type A = number & string | boolean",
    "Mixing union and intersection types is not allowed; consider wrapping in parentheses.",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "mixed_leading_intersection_and_union_not_allowed")`。
#[test]
fn parser_mixed_leading_intersection_and_union_not_allowed() {
  let mut fixture = Fixture::default();
  let mixing =
    "Mixing union and intersection types is not allowed; consider wrapping in parentheses.";
  fixture.match_parse_error("type A = & number | string | boolean", mixing, None);
  fixture.match_parse_error("type A = | number & string & boolean", mixing, None);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "mode_is_unset_if_no_hot_comment")`。
#[test]
fn parser_mode_is_unset_if_no_hot_comment() {
  let mut fixture = Fixture::default();
  let result = fixture.parse_ex("print('Hello World!')", &ParseOptions::new());
  assert!(result.hotcomments.is_empty());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "multiline_strings_newlines")`。
#[test]
fn parser_multiline_strings_newlines() {
  let mut fixture = Fixture::default();
  let stat = fixture.parse(
    "return [=[\nfoo\r\nbar\n\nbaz\n]=]",
    &ParseOptions::default(),
  );

  let ret = as_node_at::<AstStatReturn, _>(&stat.body, 0).expect("body[0] 应为 return");
  let str_expr =
    as_node_at::<AstExprConstantString, _>(&ret.list, 0).expect("list[0] 应为字符串常量");
  assert_eq!("foo\nbar\n\nbaz\n".as_bytes(), str_expr.value.as_bytes());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "multiple_parse_errors")`。
#[test]
fn parser_multiple_parse_errors() {
  let mut fix = Fixture::default();
  let source = "local a = 3 * (\nreturn a +\n";

  let result = fix.try_parse(source, &ParseOptions::default());
  assert_eq!(2, result.errors.len());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "nil_can_not_be_a_field_name")`。
#[test]
fn parser_nil_can_not_be_a_field_name() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "local n: {nil: number}",
    "Expected '}' (to close '{' at column 10), got ':'",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "nil_is_a_valid_type_name")`。
#[test]
fn parser_nil_is_a_valid_type_name() {
  let mut fixture = Fixture::default();
  fixture.parse(
    r"
        local n: nil
    ",
    &ParseOptions::default(),
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "nocheck_mode")`。
#[test]
fn parser_nocheck_mode() {
  let mut fixture = Fixture::default();
  let mut options = ParseOptions::new();
  options.capture_comments = true;

  let result = fixture.parse_ex("--!nocheck", &options);
  assert!(result.errors.is_empty());
  assert_eq!(Some(Mode::NoCheck), parse_mode(&result.hotcomments));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "non_exported_class")`。
#[test]
fn parser_non_exported_class() {
  let _flag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fixture = Fixture::default();
  let source = r"
        class Foo
        end
    ";

  let result = fixture.try_parse(source, &ParseOptions::new());
  assert_eq!(0, result.errors.len());

  let block = result.root_block().expect("根块必须存在");
  assert_eq!(1, block.body.size);

  let class_decl = as_node_at::<AstStatClass, _>(&block.body, 0).expect("body[0] 应为 class");
  assert!(!class_decl.exported);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "non_header_hot_comments")`。
#[test]
fn parser_non_header_hot_comments() {
  let mut fixture = Fixture::default();
  let mut options = ParseOptions::new();
  options.capture_comments = true;

  let result = fixture.parse_ex("do end --!strict", &options);
  assert_eq!(None, parse_mode(&result.hotcomments));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "non_literal_attribute_arguments_is_not_allowed")`。
#[test]
fn parser_non_literal_attribute_arguments_is_not_allowed() {
  let mut fix = Fixture::default();
  let code = r"
@[deprecated{ reason = reasonString }]
function hello(x, y)
    return x + y
end";

  let result = fix.try_parse(code, &ParseOptions::default());
  check_first_error_for_attributes(
    &result.errors,
    1,
    loc((1, 13), (1, 37)),
    "Only literals can be passed as arguments for attributes",
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "nonstrict_mode")`。
#[test]
fn parser_nonstrict_mode() {
  let mut fixture = Fixture::default();
  let mut options = ParseOptions::new();
  options.capture_comments = true;

  let result = fixture.parse_ex("--!nonstrict", &options);
  assert!(result.errors.is_empty());
  assert_eq!(Some(Mode::Nonstrict), parse_mode(&result.hotcomments));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "number_literals")`：六条字面量逐个回读值。
const NUMBER_LITERALS: [f64; 6] = [1.0, 1.5, 0.5, 12_3456.0, 0x1234 as f64, 0x15 as f64];

/// cpp `TEST_CASE_FIXTURE(Fixture, "number_literals")`。
#[test]
fn parser_number_literals() {
  let mut fixture = Fixture::default();
  let source = "return\n1,\n1.5,\n.5,\n12_34_56,\n0x1234,\n 0b010101\n";

  let stat = fixture.parse(source, &ParseOptions::default());
  let ret = as_node_at::<AstStatReturn, _>(&stat.body, 0).expect("body[0] 应为 return");
  assert_eq!(NUMBER_LITERALS.len(), ret.list.size);

  for (&expr, want) in ret.list.iter().zip(NUMBER_LITERALS) {
    let num = expr
      .as_node::<AstExprConstantNumber>()
      .expect("应为数字字面量");
    assert_eq!(want, num.value);
  }
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "other_places_where_type_annotations_are_allowed")`。
#[test]
fn parser_other_places_where_type_annotations_are_allowed() {
  let mut fixture = Fixture::default();
  fixture.parse(
    r"
        for i: number = 0, 50 do end
        for i: number, s: string in expr() do end
    ",
    &ParseOptions::default(),
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "overlapping_property_and_method_names")`。
#[test]
fn parser_overlapping_property_and_method_names() {
  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fix = Fixture::default();
  fix.match_parse_error(
    r"
class Hello
    public helloagain
    function helloagain() end
end
        ",
    "Duplicate class member 'helloagain'",
    None,
  );
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_attribute_for_function_expression")`。
#[test]
fn parser_parse_attribute_for_function_expression() {
  let mut fixture = Fixture::default();
  let options = ParseOptions::new();

  let source1 = r"
local function invoker(f)
    return f(1)
end

invoker(@checked function(x) return (x + 2) end)
";
  let stat1 = fixture.parse(source1, &options);
  let expr_stmt = as_node_at::<AstStatExpr, _>(&stat1.body, 1).expect("body[1] 应为表达式语句");
  let call = expr_stmt
    .expr
    .as_node::<AstExprCall>()
    .expect("expr 应为调用");
  let func1 = as_node_at::<AstExprFunction, _>(&call.args, 0).expect("args[0] 应为函数表达式");
  assert_eq!(1, func1.attributes.size);

  let attr0 = deref_at(&func1.attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr0, AstAttrType::Checked, loc((5, 8), (5, 16)));

  let source2 = r"
local f = @checked function(x) return (x + 2) end
";
  let stat2 = fixture.parse(source2, &options);
  let local_stmt = as_node_at::<AstStatLocal, _>(&stat2.body, 0).expect("body[0] 应为 local");
  let func2 =
    as_node_at::<AstExprFunction, _>(&local_stmt.values, 0).expect("values[0] 应为函数表达式");
  assert_eq!(1, func2.attributes.size);

  let attr0_2 = deref_at(&func2.attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr0_2, AstAttrType::Checked, loc((1, 10), (1, 18)));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_attribute_on_export_function_stat")`。
#[test]
fn parser_parse_attribute_on_export_function_stat() {
  let _sff_export_value_syntax = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);

  let mut fixture = Fixture::default();
  let source = r"
@checked
export function hello(x, y)
    return x + y
end";

  let stat = fixture.parse(source, &ParseOptions::default());
  let stat_fun =
    as_node_at::<AstStatLocalFunction, _>(&stat.body, 0).expect("body[0] 应为 local function");

  assert_eq!(Position::new(1, 0), stat_fun.base.base.location.begin);

  let name = stat_fun.name.as_ref_opt().expect("name 必须存在");
  assert!(name.is_exported);
  assert!(name.is_const);

  let attributes = stat_fun
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .attributes;
  assert_eq!(1, attributes.size);

  let attr = deref_at(&attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr, AstAttrType::Checked, loc((1, 0), (1, 8)));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_attribute_on_function_stat")`。
#[test]
fn parser_parse_attribute_on_function_stat() {
  let mut fix = Fixture::default();
  let code = r"
@checked
function hello(x, y)
    return x + y
end";

  let stat = fix.parse(code, &ParseOptions::default());
  let stat_fun = as_node_at::<AstStatFunction, _>(&stat.body, 0).expect("body[0] 应为函数声明");

  let attributes = stat_fun
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .attributes;
  assert_eq!(1, attributes.size);

  let attr = deref_at(&attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr, AstAttrType::Checked, loc((1, 0), (1, 8)));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_attribute_on_function_type_declaration")`。
#[test]
fn parser_parse_attribute_on_function_type_declaration() {
  let mut fix = Fixture::default();
  let code = r"
@checked declare function abs(n: number): number
";
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let result = fix.try_parse(code, &opts);
  assert_eq!(0, result.errors.len());

  let root_block = result.root_block().expect("根块必须存在");
  assert_eq!(1, root_block.body.size);

  let func = as_node_at::<AstStatDeclareFunction, _>(&root_block.body, 0)
    .expect("body[0] 应为 declare function");
  assert!(func.is_checked_function());

  assert_eq!(1, func.attributes.size);
  let attr = deref_at(&func.attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr, AstAttrType::Checked, loc((1, 0), (1, 8)));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_attribute_on_local_function_stat")`。
#[test]
fn parser_parse_attribute_on_local_function_stat() {
  let mut fix = Fixture::default();
  let code = r"
    @checked
local function hello(x, y)
    return x + y
end";

  let stat = fix.parse(code, &ParseOptions::default());
  let stat_fun =
    as_node_at::<AstStatLocalFunction, _>(&stat.body, 0).expect("body[0] 应为 local function");

  let attributes = stat_fun
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .attributes;
  assert_eq!(1, attributes.size);

  let attr = deref_at(&attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr, AstAttrType::Checked, loc((1, 4), (1, 12)));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_attributes_on_function_type_declaration_in_table")`。
#[test]
fn parser_parse_attributes_on_function_type_declaration_in_table() {
  let mut fixture = Fixture::default();
  let source = r"
declare bit32: {
    band: @checked (...number) -> number
}";
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let result = fixture.try_parse(source, &opts);
  assert_eq!(0, result.errors.len());

  let root_block = result.root_block().expect("根块必须存在");
  assert_eq!(1, root_block.body.size);

  let glob = as_node_at::<AstStatDeclareGlobal, _>(&root_block.body, 0)
    .expect("body[0] 应为 declare global");
  let tbl = glob
    .type_
    .as_node::<AstTypeTable>()
    .expect("类型应为表类型");
  assert_eq!(1, tbl.props.size);

  let prop = elem(&tbl.props, 0);
  let func = prop
    .r#type
    .as_node::<AstTypeFunction>()
    .expect("属性类型应为函数类型");

  assert_eq!(1, func.attributes.size);
  let attr = deref_at(&func.attributes, 0).expect("attributes[0] 应为属性");
  check_attribute(attr, AstAttrType::Checked, loc((2, 10), (2, 18)));
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_checked_as_function_name_fails")`。
#[test]
fn parser_parse_checked_as_function_name_fails() {
  let mut fix = Fixture::default();
  let code = r"
    @checked function(x: number) : number
    end
";
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let result = fix.try_parse(code, &opts);
  assert!(!result.errors.is_empty());
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_checked_in_and_out_of_decl_fails")`。
#[test]
fn parser_parse_checked_in_and_out_of_decl_fails() {
  let mut fix = Fixture::default();
  let code = r"
    local @checked = 3
    @checked declare function abs(n: number): number
";
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let result = fix.try_parse(code, &opts);
  assert_eq!(2, result.errors.len());
  assert_eq!(1, result.errors[0].get_location().begin.line);
  assert_eq!(1, result.errors[1].get_location().begin.line);
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_checked_outside_decl_fails")`。
#[test]
fn parser_parse_checked_outside_decl_fails() {
  let mut fixture = Fixture::default();
  let source = r"
    local @checked = 3
";
  let mut opts = ParseOptions::new();
  opts.allow_declaration_syntax = true;

  let result = fixture.try_parse(source, &opts);
  assert!(!result.errors.is_empty());
  let _ts = result.errors[1].get_message();
}

/// cpp `TEST_CASE_FIXTURE(Fixture, "parse_class_declarations")`。
#[test]
fn parser_parse_class_declarations() {
  let mut fixture = Fixture::default();
  let source = r"
        declare extern type Foo with
            prop: number
            function method(self, foo: number): string
        end

        declare extern type Bar extends Foo with
            prop2: string
        end
    ";

  let result = fixture.parse_ex(source, &ParseOptions::default());
  let root = result.root_block().expect("根块必须存在");
  assert_eq!(2, root.body.size);

  let foo =
    as_node_at::<AstStatDeclareExternType, _>(&root.body, 0).expect("body[0] 应为 extern type");
  assert_eq!(Some("Foo"), foo.name.as_str());
  assert!(foo.super_name.is_none());
  assert_eq!(2, foo.props.size);

  let prop = elem(&foo.props, 0);
  assert_eq!(Some("prop"), prop.name.as_str());
  assert_eq!(loc((2, 12), (2, 16)), prop.name_location);
  assert_eq!(loc((2, 12), (2, 24)), prop.location);
  assert!(prop.ty.as_node::<AstTypeReference>().is_some());

  let method = elem(&foo.props, 1);
  assert_eq!(Some("method"), method.name.as_str());
  assert_eq!(loc((3, 21), (3, 27)), method.name_location);
  assert_eq!(loc((3, 12), (3, 54)), method.location);
  assert!(method.is_method);
  assert!(method.ty.as_node::<AstTypeFunction>().is_some());

  let bar =
    as_node_at::<AstStatDeclareExternType, _>(&root.body, 1).expect("body[1] 应为 extern type");
  assert!(bar.super_name.is_some());
  assert_eq!(Some("Bar"), bar.name.as_str());
  assert_eq!(
    Some("Foo"),
    bar.super_name.expect("superName 必须存在").as_str()
  );

  assert_eq!(1, bar.props.size);
  let prop2 = elem(&bar.props, 0);
  assert_eq!(Some("prop2"), prop2.name.as_str());
  assert_eq!(loc((7, 12), (7, 17)), prop2.name_location);
  assert_eq!(loc((7, 12), (7, 25)), prop2.location);
  assert!(prop2.ty.as_node::<AstTypeReference>().is_some());
}
