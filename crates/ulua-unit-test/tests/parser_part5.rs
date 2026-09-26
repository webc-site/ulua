//! Port of `cpp/tests/Parser.test.cpp`（TEST_CASE_FIXTURE 逐条对应，第 5 部分）。

use std::sync::atomic::Ordering;

use ulua_ast::{
  records::{
    allocator::Allocator,
    ast_attr::{AstAttr, AstAttrType},
    ast_expr_call::AstExprCall,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_interp_string::AstExprInterpString,
    ast_name_table::AstNameTable,
    ast_stat_block::AstStatBlock,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_expr::AstStatExpr,
    ast_stat_function::AstStatFunction,
    ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction,
    ast_type::AstType,
    ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference,
    location::Location,
    parse_node_result::ParseNodeResult,
    parse_options::ParseOptions,
    parser::Parser,
    position::Position,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{dfflag, fflag};
use ulua_unit_test::{
  functions::{
    ast_node_ref::{NodePtr, PtrRef, as_node_at},
    check_attribute::check_attribute,
  },
  records::fixture::Fixture,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

/// cpp `parse_nesting_based_end_detection` / `_local_function` / `_nested` 三条
/// 用例共用的期望消息：'function' 未闭合、'else' 在第 8 行漏闭合。
const MISSING_END_ELSE_LINE8: &str = "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?";

/// cpp 插值串用例的源：末段 `{e.a` 既没闭合 '}' 也没收尾反引号。
const INTERP_MISSING_CURLY: &str = "print(`{e.x} {e.a";
/// cpp 插值串用例的源：'}' 已闭合，缺收尾反引号。
const INTERP_MISSING_BACKTICK: &str = "print(`{e.x} {e.a}";
/// cpp 插值串用例的源：末段以反引号收尾但缺 '}'。
const INTERP_CURLY_WITH_BACKTICK: &str = "print(`{e.x} {e.a`";

/// cpp 报错消息：插值串缺 '}'。
const MALFORMED_MISSING_CURLY: &str = "Malformed interpolated string; did you forget to add a '}'?";
/// cpp 报错消息：插值串缺收尾 '`'。
const MALFORMED_MISSING_BACKTICK: &str =
  "Malformed interpolated string; did you forget to add a '`'?";

/// cpp `recover_function_return_type_annotations` 前两条错误的期望消息。
const FUNCTION_TYPE_ARROW: &str =
  "Return types in function type annotations are written after '->' instead of ':'";

/// cpp `try { parse(src); FAIL(...); } catch (const ParseErrors& e) { ... }`：
/// 用不抛错的 `try_parse` 取首条错误，未抛错即判定用例失败。
fn assert_first_error(fixture: &mut Fixture, source: &str, message: &str) {
  let result = fixture.try_parse(source, &ParseOptions::new());
  let first = result
    .errors
    .first()
    .expect("Expected ParseErrors to be thrown");
  assert_eq!(message, first.get_message());
}

/// cpp `root->body.data[0]->as<AstStatExpr>()->expr->as<AstExprCall>()
/// ->args.data[0]->as<AstExprInterpString>()`：取 `print(`...`)` 里的插值串。
fn interpolation_of(root: &AstStatBlock) -> &AstExprInterpString {
  let stat = as_node_at::<AstStatExpr, _>(&root.body, 0).expect("body[0] 应为表达式语句");
  let call = stat
    .expr
    .as_node::<AstExprCall>()
    .expect("expr 应为调用表达式");
  as_node_at::<AstExprInterpString, _>(&call.args, 0).expect("args[0] 应为插值串")
}

/// cpp `parsing_incomplete_string_interpolation_*` 六条用例的共同断言：
/// 2 条错误、首条消息与位置，以及插值串的表达式数与区间。
fn check_incomplete_interpolation(
  source: &str,
  message: &str,
  error_location: Location,
  interpolation_location: Location,
) {
  let mut fixture = Fixture::default();
  let result = fixture.try_parse(source, &ParseOptions::new());

  assert_eq!(2, result.errors.len(), "errors: {:?}", result.errors);
  let first_error = &result.errors[0];
  assert_eq!(message, first_error.get_message());
  assert_eq!(&error_location, first_error.get_location());

  let root = result.root_block().expect("根块必须存在");
  let interpolation = interpolation_of(root);
  assert_eq!(2, interpolation.expressions.size);
  assert_eq!(&interpolation_location, &interpolation.base.base.location);
}

/// cpp 源码尾补一个换行即成 "broken string" 版（插值串未结束就换行）。
fn broken(source: &str) -> String {
  format!("{source}\n")
}

/// 数字字面量用例的公共形态：`return a, b, ...` 的 return 语句。
fn return_stat<'a>(fixture: &'a mut Fixture, source: &str) -> &'a AstStatReturn {
  let block = fixture.parse(source, &ParseOptions::default());
  as_node_at::<AstStatReturn, _>(&block.body, 0).expect("body[0] 应为 return 语句")
}

#[test]
fn parser_parse_nested_type_function() {
  // cpp `REQUIRE(stat != nullptr)`：parse 出错即 panic，非空由返回引用担保。
  let mut fixture = Fixture::default();
  fixture.parse(
    r#"
        local v1 = 1
        type function foo()
            local v2 = 2
            local function bar()
                v2 += 1
                type function inner() end
                v2 += 2
            end
            local function bar2()
                v2 += 3
            end
        end
        local function bar() v1 += 1 end
    "#,
    &ParseOptions::default(),
  );
}

#[test]
fn parser_parse_nesting_based_end_detection() {
  let mut fixture = Fixture::default();
  assert_first_error(
    &mut fixture,
    r#"-- i am line 1
function BottomUpTree(item, depth)
  if depth > 0 then
    local i = item + item
    depth = depth - 1
    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
    return { item, left, right }
  else
    return { item }
end

function ItemCheck(tree)
  if tree[2] then
    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
  else
    return tree[1]
  end
end
        "#,
    MISSING_END_ELSE_LINE8,
  );
}

#[test]
fn parser_parse_nesting_based_end_detection_failsafe_earlier() {
  let mut fixture = Fixture::default();
  assert_first_error(
    &mut fixture,
    r#"-- i am line 1
local function ItemCheck(tree)
  if tree[2] then
    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
  else
    return tree[1]
      end
end

local function BottomUpTree(item, depth)
  if depth > 0 then
    local i = item + item
    depth = depth - 1
    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
    return { item, left, right }
  else
    return { item }
  end
        "#,
    "Expected 'end' (to close 'function' at line 10), got <eof>",
  );
}

#[test]
fn parser_parse_nesting_based_end_detection_local_function() {
  let mut fixture = Fixture::default();
  assert_first_error(
    &mut fixture,
    r#"-- i am line 1
local function BottomUpTree(item, depth)
  if depth > 0 then
    local i = item + item
    depth = depth - 1
    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
    return { item, left, right }
  else
    return { item }
end

local function ItemCheck(tree)
  if tree[2] then
    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
  else
    return tree[1]
  end
end
        "#,
    MISSING_END_ELSE_LINE8,
  );
}

#[test]
fn parser_parse_nesting_based_end_detection_local_repeat() {
  let mut fixture = Fixture::default();
  assert_first_error(
    &mut fixture,
    r#"-- i am line 1
repeat
  print(1)
  repeat
    print(2)
  print(3)
until false
        "#,
    "Expected 'until' (to close 'repeat' at line 2), got <eof>; did you forget to close 'repeat' at line 4?",
  );
}

#[test]
fn parser_parse_nesting_based_end_detection_nested() {
  let mut fixture = Fixture::default();
  assert_first_error(
    &mut fixture,
    r#"-- i am line 1
function stringifyTable(t)
    local entries = {}
    for k, v in pairs(t) do
        -- if we find a nested table, convert that recursively
        if type(v) == "table" then
            v = stringifyTable(v)
        else
            v = tostring(v)
        k = tostring(k)

        -- add another entry to our stringified table
        entries[#entries + 1] = ("s = s"):format(k, v)
    end

    -- the memory location of the table
    local id = tostring(t):sub(8)

    return ("{s}@s"):format(table.concat(entries, ", "), id)
end
        "#,
    MISSING_END_ELSE_LINE8,
  );
}

#[test]
fn parser_parse_nesting_based_end_detection_single_line() {
  let mut fixture = Fixture::default();
  assert_first_error(
    &mut fixture,
    r#"-- i am line 1
function ItemCheck(tree)
  if tree[2] then return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3]) else return tree[1]
end

function BottomUpTree(item, depth)
  if depth > 0 then
    local i = item + item
    depth = depth - 1
    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
    return { item, left, right }
  else
    return { item }
  end
end
        "#,
    "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 3?",
  );
}

#[test]
fn parser_parse_numbers_binary() {
  let mut fixture = Fixture::default();
  let list = return_stat(
    &mut fixture,
    "return 0b1, 0b0, 0b101010, 0b1111111111111111111111111111111111111111111111111111111111111111",
  )
  .list;
  assert_eq!(4, list.size);
  for (index, expected) in [(0, 1.0), (1, 0.0), (2, 42.0), (3, u64::MAX as f64)] {
    let number = as_node_at::<AstExprConstantNumber, _>(&list, index).expect("应为浮点常量");
    assert_eq!(expected, number.value);
  }

  if fflag::LuauIntegerType2.get() {
    // 与 cpp 同形的旗标保护：默认关闭，打开后整数后缀分支亦已验证通过。
    let list = return_stat(
        &mut fixture,
        "return 0b1i, 0b0i, 0b101010i, 0b111111111111111111111111111111111111111111111111111111111111111i, \
         0b1000000000000000000000000000000000000000000000000000000000000000i, \
         0b1111111111111111111111111111111111111111111111111111111111111111i",
      )
      .list;
    assert_eq!(6, list.size);
    for (index, expected) in [
      (0, 1),
      (1, 0),
      (2, 42),
      (3, i64::MAX),
      (4, i64::MIN),
      (5, -1),
    ] {
      let integer = as_node_at::<AstExprConstantInteger, _>(&list, index).expect("应为整数常量");
      assert_eq!(expected, integer.value);
    }
  }
}

#[test]
fn parser_parse_numbers_decimal() {
  let mut fixture = Fixture::default();
  let list = return_stat(&mut fixture, "return 1, .5, 1.5, 1e-5, 1.5e-5, 12_345.1_25").list;
  assert_eq!(6, list.size);
  for (index, expected) in [
    (0, 1.0),
    (1, 0.5),
    (2, 1.5),
    (3, 1.0e-5),
    (4, 1.5e-5),
    (5, 12345.125),
  ] {
    let number = as_node_at::<AstExprConstantNumber, _>(&list, index).expect("应为浮点常量");
    assert_eq!(expected, number.value);
  }

  if fflag::LuauIntegerType2.get() {
    let list = return_stat(&mut fixture, "return 1i, 1_000_000i").list;
    assert_eq!(2, list.size);
    for (index, expected) in [(0, 1), (1, 1000000)] {
      let integer =
        as_node_at::<AstExprConstantInteger, _>(&list, index).expect("整数后缀应产出整数常量");
      assert_eq!(expected, integer.value);
    }
  }
}

#[test]
fn parser_parse_numbers_error() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error("return 0b123", "Malformed number", None);
  // cpp 用 ScopedFastFlag{LuauNoDuplicateBinaryPrefix, true} 打开该检查；移植侧
  // 无此旗标（词法器恒定拒绝重复前缀），期望值保持一致。
  fixture.match_parse_error("return 0b0b1", "Malformed number", None);
  fixture.match_parse_error("return 123x", "Malformed number", None);
  fixture.match_parse_error("return 0xg", "Malformed number", None);
  fixture.match_parse_error("return 0x0x123", "Malformed number", None);
  fixture.match_parse_error(
    "return 0xffffffffffffffffffffllllllg",
    "Malformed number",
    None,
  );
  fixture.match_parse_error(
    "return 0x0xffffffffffffffffffffffffffff",
    "Malformed number",
    None,
  );

  if fflag::LuauIntegerType2.get() {
    fixture.match_parse_error("return 0x0xABCi", "Malformed integer", None);
    fixture.match_parse_error("return 0xABCMi", "Malformed integer", None);
    fixture.match_parse_error("return 0b250i", "Malformed integer", None);
    fixture.match_parse_error("return 0bbbbi", "Malformed integer", None);
    fixture.match_parse_error("return 0b0b1i", "Malformed integer", None);
    fixture.match_parse_error("return 123ii", "Malformed integer", None);
    fixture.match_parse_error("return 0xABii", "Malformed integer", None);
    fixture.match_parse_error("return 99999999999999999999i", "Integer overflow", None);
    fixture.match_parse_error("return 0xFFFFFFFFFFFFFFFFFFi", "Integer overflow", None);
    fixture.match_parse_error(
      "return 0b10000000000000000000000000000000000000000000000000000000000000000i",
      "Integer overflow",
      None,
    );
  }
}

#[test]
fn parser_parse_numbers_hexadecimal() {
  let mut fixture = Fixture::default();
  let list = return_stat(
    &mut fixture,
    "return 0xab, 0xAB05, 0xff_ff, 0xffffffffffffffff",
  )
  .list;
  assert_eq!(4, list.size);
  // cpp `double(ULLONG_MAX)` —— u64::MAX 转浮点，非 f64::MAX。
  for (index, expected) in [
    (0, 0xab as f64),
    (1, 0xAB05 as f64),
    (2, 0xFFFF as f64),
    (3, u64::MAX as f64),
  ] {
    let number = as_node_at::<AstExprConstantNumber, _>(&list, index).expect("应为浮点常量");
    assert_eq!(expected, number.value);
  }

  if fflag::LuauIntegerType2.get() {
    let list = return_stat(
      &mut fixture,
      "return 0xabi, 0XAB05i, 0xff_ffi, 0x7fffffffffffffffi, 0x8000000000000000i, 0xffffffffffffffffi",
    )
    .list;
    assert_eq!(6, list.size);
    for (index, expected) in [
      (0, 0xab),
      (1, 0xAB05),
      (2, 0xFFFF),
      (3, i64::MAX),
      (4, i64::MIN),
      (5, -1),
    ] {
      let integer = as_node_at::<AstExprConstantInteger, _>(&list, index).expect("应为整数常量");
      assert_eq!(expected, integer.value);
    }
  }
}

#[test]
fn parser_parse_parametrized_attribute_on_function_stat() {
  let mut fixture = Fixture::default();
  let block = fixture.parse(
    r#"
@[deprecated{ use = "greetng", reason = "Using <hello> is too causal"}]
function hello(x, y)
    return x + y
end"#,
    &ParseOptions::default(),
  );

  let stat_fun = as_node_at::<AstStatFunction, _>(&block.body, 0).expect("body[0] 应为函数声明");
  let func = stat_fun.func.as_ref_opt().expect("函数表达式必须存在");
  assert_eq!(1, func.attributes.len());

  let attr = as_node_at::<AstAttr, _>(&func.attributes, 0).expect("attributes[0] 应为属性节点");
  check_attribute(
    attr,
    AstAttrType::Deprecated,
    Location::new(Position::new(1, 2), Position::new(1, 70)),
  );
}

#[test]
fn parser_parse_return_type_ast_type_pack_explicit() {
  let _sff = ScopedFastFlag::new(
    &fflag::LuauSingleTypeOptionalPackReturnsAttributeParens,
    true,
  );

  let mut fixture = Fixture::default();
  let block = fixture.parse("type Foo = () -> (string)", &ParseOptions::default());
  assert_eq!(1, block.body.len());

  let alias = as_node_at::<AstStatTypeAlias, _>(&block.body, 0).expect("body[0] 应为类型别名");
  let func_type = alias
    .type_ptr
    .as_node::<AstTypeFunction>()
    .expect("别名右侧应为函数类型");
  let return_pack = func_type
    .return_types
    .as_node::<AstTypePackExplicit>()
    .expect("返回类型应为显式 type pack");

  assert_eq!(1, return_pack.type_list.types.size);
  assert!(
    return_pack.type_list.tail_type.as_ref_opt().is_none(),
    "tail_type 应为空"
  );
  assert!(as_node_at::<AstTypeReference, _>(&return_pack.type_list.types, 0).is_some());
}

#[test]
fn parser_parse_simple_ast_type_group() {
  let mut fixture = Fixture::default();
  let block = fixture.parse("type Foo = (string)", &ParseOptions::default());
  assert_eq!(1, block.body.len());

  let alias = as_node_at::<AstStatTypeAlias, _>(&block.body, 0).expect("body[0] 应为类型别名");
  let group = alias
    .type_ptr
    .as_node::<AstTypeGroup>()
    .expect("别名右侧应为类型分组");
  assert!(group.type_.as_node::<AstTypeReference>().is_some());
}

#[test]
fn parser_parse_top_level_checked_fn() {
  let mut fixture = Fixture::default();
  let options = ParseOptions {
    allow_declaration_syntax: true,
    ..ParseOptions::new()
  };

  let result = fixture.try_parse(
    "@checked declare function abs(n: number): number\n",
    &options,
  );
  assert_eq!(0, result.errors.len());

  let root = result.root_block().expect("根块必须存在");
  assert_eq!(1, root.body.len());
  let func = as_node_at::<AstStatDeclareFunction, _>(&root.body, 0).expect("body[0] 应为声明函数");
  assert!(func.is_checked_function());
}

#[test]
fn parser_parse_type_alias_default_type() {
  // cpp `REQUIRE(stat != nullptr)`：parse 出错即 panic。
  let mut fixture = Fixture::default();
  fixture.parse(
    r#"
type A<T = string> = {}
type B<T... = ...number> = {}
type C<T..., U... = T...> = {}
type D<T..., U... = ()> = {}
type E<T... = (), U... = ()> = {}
type F<T... = (string), U... = ()> = (T...) -> U...
type G<T... = ...number, U... = (string, number, boolean)> = (U...) -> T...
    "#,
    &ParseOptions::default(),
  );
}

#[test]
fn parser_parse_type_alias_default_type_errors() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "type Y<T = number, U> = {}",
    "Expected default type after type name",
    Some(Location::new(Position::new(0, 20), Position::new(0, 21))),
  );
  fixture.match_parse_error(
    "type Y<T... = ...number, U...> = {}",
    "Expected default type pack after type pack name",
    Some(Location::new(Position::new(0, 29), Position::new(0, 30))),
  );
  fixture.match_parse_error(
    "type Y<T... = (string) -> number> = {}",
    "Expected type pack after '=', got type",
    Some(Location::new(Position::new(0, 14), Position::new(0, 32))),
  );
}

#[test]
fn parser_parse_type_name() {
  let code = "<A>(A, string, boolean?) -> number";
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);

  let result: ParseNodeResult<AstType> =
    Parser::parse_type_source(code, &mut names, &mut allocator, ParseOptions::new());

  assert!(result.errors.is_empty());
  let root = result.root.as_ref_opt().expect("根类型必须存在");

  let fun = ast_node_try_as::<AstTypeFunction>(&root.base).expect("根类型应为函数类型");
  assert_eq!(1, fun.generics.size);
  assert_eq!(3, fun.arg_types.types.size);

  let return_pack = fun
    .return_types
    .as_node::<AstTypePackExplicit>()
    .expect("返回类型应为显式 type pack");
  assert_eq!(1, return_pack.type_list.types.size);
}

#[test]
fn parser_parse_type_pack_errors() {
  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "type Y<T...> = {a: T..., b: number}",
    "Unexpected '...' after type name; type pack is not allowed in this context",
    Some(Location::new(Position::new(0, 20), Position::new(0, 23))),
  );
  fixture.match_parse_error(
    "type Y<T...> = {a: (number | string)...",
    "Unexpected '...' after type annotation",
    Some(Location::new(Position::new(0, 36), Position::new(0, 39))),
  );
}

#[test]
fn parser_parse_type_pack_type_parameters() {
  // cpp `REQUIRE(stat != nullptr)`：parse 出错即 panic。
  let mut fixture = Fixture::default();
  fixture.parse(
    r#"
type Packed<T...> = () -> T...

type A<X...> = Packed<X...>
type B<X...> = Packed<...number>
type C<X...> = Packed<(number, X...)>
    "#,
    &ParseOptions::default(),
  );
}

#[test]
fn parser_parse_user_defined_type_functions() {
  let mut fixture = Fixture::default();
  let block = fixture.parse(
    "type function foo()\n\
     return types.number\n\
     end\n\
     \n\
     export type function bar()\n\
     return types.string\n\
     end",
    &ParseOptions::default(),
  );
  assert_eq!(2, block.body.len());

  let type_function =
    as_node_at::<AstStatTypeFunction, _>(&block.body, 0).expect("body[0] 应为 type function");
  // cpp `REQUIRE(f->name == "foo")` —— AstName 与字面量比较是内容比较。
  assert_eq!(Some("foo"), type_function.name.as_str());
}

#[test]
fn parser_parse_variadics() {
  let mut fixture = Fixture::default();
  let result = fixture.parse_ex(
    "function foo(bar, ...: number): ...string\n\
     end\n\
     \n\
     type Foo = (string, number, ...number) -> ...boolean\n\
     type Bar = () -> (number, ...boolean)",
    &ParseOptions::default(),
  );
  let root = result.root_block().expect("根块必须存在");
  assert_eq!(3, root.body.len());

  let fn_stat = as_node_at::<AstStatFunction, _>(&root.body, 0).expect("body[0] 应为函数声明");
  let func = fn_stat.func.as_ref_opt().expect("函数表达式必须存在");
  assert!(func.vararg);
  assert!(func.vararg_annotation.as_ref_opt().is_some());

  let foo = as_node_at::<AstStatTypeAlias, _>(&root.body, 1).expect("body[1] 应为类型别名");
  let foo_fn = foo
    .type_ptr
    .as_node::<AstTypeFunction>()
    .expect("Foo 应为函数类型");
  assert_eq!(2, foo_fn.arg_types.types.size);
  assert!(foo_fn.arg_types.tail_type.as_ref_opt().is_some());
  assert!(
    foo_fn
      .return_types
      .as_node::<AstTypePackVariadic>()
      .is_some()
  );

  let bar = as_node_at::<AstStatTypeAlias, _>(&root.body, 2).expect("body[2] 应为类型别名");
  let bar_fn = bar
    .type_ptr
    .as_node::<AstTypeFunction>()
    .expect("Bar 应为函数类型");
  assert_eq!(0, bar_fn.arg_types.types.size);
  assert!(bar_fn.arg_types.tail_type.as_ref_opt().is_none());
  let return_pack = bar_fn
    .return_types
    .as_node::<AstTypePackExplicit>()
    .expect("Bar 返回类型应为显式 type pack");
  assert_eq!(1, return_pack.type_list.types.size);
  assert!(return_pack.type_list.tail_type.as_ref_opt().is_some());
}

#[test]
fn parser_parsing_incomplete_string_interpolation_missing_backtick_at_eof() {
  check_incomplete_interpolation(
    INTERP_MISSING_BACKTICK,
    MALFORMED_MISSING_BACKTICK,
    Location::new(Position::new(0, 17), Position::new(0, 18)),
    Location::new(Position::new(0, 6), Position::new(0, 18)),
  );
}

#[test]
fn parser_parsing_incomplete_string_interpolation_missing_backtick_broken_string() {
  check_incomplete_interpolation(
    &broken(INTERP_MISSING_BACKTICK),
    MALFORMED_MISSING_BACKTICK,
    Location::new(Position::new(0, 17), Position::new(0, 18)),
    Location::new(Position::new(0, 6), Position::new(0, 18)),
  );
}

#[test]
fn parser_parsing_incomplete_string_interpolation_missing_curly_at_eof() {
  check_incomplete_interpolation(
    INTERP_MISSING_CURLY,
    MALFORMED_MISSING_CURLY,
    Location::new(Position::new(0, 16), Position::new(0, 17)),
    Location::new(Position::new(0, 6), Position::new(0, 17)),
  );
}

#[test]
fn parser_parsing_incomplete_string_interpolation_missing_curly_broken_string() {
  check_incomplete_interpolation(
    &broken(INTERP_MISSING_CURLY),
    MALFORMED_MISSING_CURLY,
    Location::new(Position::new(0, 16), Position::new(0, 17)),
    Location::new(Position::new(0, 6), Position::new(0, 17)),
  );
}

#[test]
fn parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_at_eof() {
  check_incomplete_interpolation(
    INTERP_CURLY_WITH_BACKTICK,
    MALFORMED_MISSING_CURLY,
    Location::new(Position::new(0, 17), Position::new(0, 18)),
    Location::new(Position::new(0, 6), Position::new(0, 18)),
  );
}

#[test]
fn parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_broken_string() {
  check_incomplete_interpolation(
    &broken(INTERP_CURLY_WITH_BACKTICK),
    MALFORMED_MISSING_CURLY,
    Location::new(Position::new(0, 17), Position::new(0, 18)),
    Location::new(Position::new(0, 6), Position::new(0, 18)),
  );
}

#[test]
fn parser_parsing_string_union_indexers() {
  // cpp `parse(...)`：出错即 panic，故无需再断言。
  let mut fixture = Fixture::default();
  fixture.parse(
    r#"type foo = { ["bar" | "baz"]: number }"#,
    &ParseOptions::default(),
  );
}

#[test]
fn parser_parsing_type_suffix_for_return_type_with_variadic() {
  // C++ sets the DFFlag (which gates the parser's telemetry side-effect) and then
  // checks the telemetry GLOBAL, not the flag itself.
  let _sff = ScopedFastFlag::new(
    &dfflag::DebugLuauReportReturnTypeVariadicWithTypeSuffix,
    true,
  );
  ulua_ast::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX
    .store(false, Ordering::Relaxed);

  let mut fixture = Fixture::default();
  let result = fixture.try_parse(
    "function foo(): (string, ...number) | boolean\nend",
    &ParseOptions::new(),
  );

  // TODO(CLI-140667): this should produce a ParseError in future when we fix the invalid syntax
  assert_eq!(0, result.errors.len());
  assert!(
    ulua_ast::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX.load(Ordering::Relaxed)
  );
}

#[test]
fn parser_read_write_table_properties() {
  let mut fixture = Fixture::default();
  let result = fixture.try_parse(
    "type A = {read x: number}\n\
     type B = {write x: number}\n\
     type C = {read x: number, write x: number}\n\
     type D = {read: () -> string}\n\
     type E = {write: (string) -> ()}\n\
     type F = {read read: () -> string}\n\
     type G = {read write: (string) -> ()}\n\
     type H = {read [\"A\"]: number}\n\
     type I = {write [\"A\"]: string}\n\
     type J = {read [number]: number}\n\
     type K = {write [number]: string}",
    &ParseOptions::new(),
  );
  assert_eq!(0, result.errors.len());
}

#[test]
fn parser_reassigned_class() {
  let _g = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _g_export = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);

  let mut fixture = Fixture::default();
  fixture.match_parse_error(
    "\nclass Animal end\nAnimal = nil\n",
    // cpp Parser.test.cpp:3729：类名全局不可作为变量名赋值。
    "'Animal' refers to a class and cannot be used as a variable name (defined on line 2)",
    None,
  );
}

#[test]
fn parser_recover_confusables() {
  let mut fixture = Fixture::default();

  // Binary
  fixture.match_parse_error(
    "local a = 4 != 10",
    "Unexpected '!='; did you mean '~='?",
    None,
  );
  fixture.match_parse_error(
    "local a = true && false",
    "Unexpected '&&'; did you mean 'and'?",
    None,
  );
  fixture.match_parse_error(
    "local a = false || true",
    "Unexpected '||'; did you mean 'or'?",
    None,
  );

  // Unary
  fixture.match_parse_error(
    "local a = !false",
    "Unexpected '!'; did you mean 'not'?",
    None,
  );

  // Check that separate tokens are not considered as a single one
  fixture.match_parse_error(
    "local a = 4 ! = 10",
    "Expected identifier when parsing expression, got '!'",
    None,
  );
  fixture.match_parse_error(
    "local a = true & & false",
    "Expected identifier when parsing expression, got '&'",
    None,
  );
  fixture.match_parse_error(
    "local a = false | | true",
    "Expected identifier when parsing expression, got '|'",
    None,
  );
}

#[test]
fn parser_recover_expected_type_pack() {
  let mut fixture = Fixture::default();
  let result = fixture.try_parse(
    "type Y<T..., U = T...> = (T...) -> U...\n",
    &ParseOptions::new(),
  );
  assert_eq!(1, result.errors.len());
}

#[test]
fn parser_recover_from_bad_table_type() {
  let mut fixture = Fixture::default();
  let options = ParseOptions {
    allow_declaration_syntax: true,
    ..ParseOptions::new()
  };
  let result = fixture.try_parse(
    "declare extern type Widget with\n    state: {string: function(string, Widget)}\nend\n",
    &options,
  );
  assert_eq!(2, result.errors.len());
}

#[test]
fn parser_recover_function_return_type_annotations() {
  let mut fixture = Fixture::default();
  let result = fixture.try_parse(
    "type Custom<A, B, C> = { x: A, y: B, z: C }\n\
     type Packed<A...> = { x: (A...) -> () }\n\
     type F = (number): Custom<boolean, number, string>\n\
     type G = Packed<(number): (string, number, boolean)>\n\
     local function f(x: number) -> Custom<string, boolean, number>\n\
     end",
    &ParseOptions::new(),
  );

  assert_eq!(3, result.errors.len());
  assert_eq!(FUNCTION_TYPE_ARROW, result.errors[0].get_message());
  assert_eq!(FUNCTION_TYPE_ARROW, result.errors[1].get_message());
  assert_eq!(
    "Function return type annotations are written after ':' instead of '->'",
    result.errors[2].get_message()
  );
}

#[test]
fn parser_recover_index_name_keyword() {
  let mut fixture = Fixture::default();
  let options = ParseOptions::new();

  let result = fixture.try_parse("local b\nlocal a = b.do\n", &options);
  assert_eq!(1, result.errors.len());

  let result = fixture.try_parse("local b\nlocal a = b.\ndo end\n", &options);
  assert_eq!(1, result.errors.len());
}

#[test]
fn parser_recover_self_call_keyword() {
  let mut fixture = Fixture::default();
  let options = ParseOptions::new();

  let result = fixture.try_parse("local b\nlocal a = b:do\n", &options);
  assert_eq!(2, result.errors.len());

  let result = fixture.try_parse("local b\nlocal a = b:\ndo end\n", &options);
  assert_eq!(2, result.errors.len());
}
