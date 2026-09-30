//! Pretty-print golden 测试：对照 `cpp/tests/PrettyPrinter.test.cpp`。
//!
//! Luau 的 PrettyPrinter 是「按 source location 回填原始 token」的打印器：
//! 解析结果与源码列位一致时，`prettyPrint(code).code` 与 `code` 逐字符相等
//! （C++ 侧大量用例即 `CHECK_EQ(code, prettyPrint(code).code)`）。本文件把这类
//! 回环用例落到 crate 级，作为可视化 `visualize_*` 热路径的 golden 快照；
//! 任一 token 回填或空格/换行推进逻辑退化都会在此暴露。
//!
//! 覆盖维度（每项均取自 cpp 同名 TEST_CASE）：
//! - 语句/块结构：local、do、while+continue、repeat、function、多行函数
//! - 表达式：一元（`-`/`not`/`#`）、二元优先级与 `and`/`or`、表构造（record/list/general/分号）
//! - token 空格策略：`~=` 紧贴、`..`、比较运算符
//! - 数字字面量：hex（`0xFF`）、下划线分隔（`1_000`）
//! - 字符串：单引号原文回填、fixup 后任意字节逐字节保真
//! - 带类型注解回环（with_types）

use ulua_ast::{
  functions::pretty_print_pretty_printer::{
    pretty_print_ast_stat_block_cst_node_map_bytes,
    pretty_print_string_view_parse_options_bool_bool,
  },
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
  },
};

/// `prettyPrint(source)` 的 Rust 对应：默认选项、不带类型、不容错。
fn pretty_print(source: &str) -> String {
  let r =
    pretty_print_string_view_parse_options_bool_bool(source, ParseOptions::default(), false, false);
  assert!(
    r.parse_error.is_empty(),
    "unexpected parse error for {source:?}: {}",
    r.parse_error
  );
  r.code
}

/// 断言 `source` 经解析后逐字符回环（cpp 主用例形态）。
fn round_trip(source: &str) {
  assert_eq!(
    source,
    pretty_print(source),
    "round-trip mismatch for {source:?}"
  );
}

// cpp: test_1 —— 多行函数体、`~=` 紧贴、空行保留
#[test]
fn golden_multiline_function_with_loose_operators() {
  round_trip(
    "local function isPortal(element)
    if type(element)~='table'then
        return false
    end

    return element.component == Core.Portal
end",
  );
}

// cpp: prettyPrint_AstStatBlock_overload / string_literals 系列基础回环
#[test]
fn golden_statements() {
  round_trip("local a = 1");
  round_trip("if x then y() else z() end");
  round_trip("for i = 1, 10, 2 do end");
  round_trip("while true do continue end");
  round_trip("repeat x = 1 until x > 0");
  round_trip("do local q = 2 end");
  round_trip("function t.f:g(a) end");
  round_trip("return");
}

#[test]
fn golden_expressions() {
  // 一元运算符
  round_trip("local x = -1\nlocal y = not true\nlocal z = #\"abc\"");
  // 二元优先级：and/or 与比较混排
  round_trip("local ok = a and b or c");
}

// cpp: string_literals —— 单引号原文按 CST 回填
#[test]
fn golden_string_literal_preserves_quotes() {
  round_trip("local s = 'hello'");
}

// cpp: pretty_printer_binary_numbers / hexadecimal_numbers：hex 与下划线字面量回填
#[test]
fn golden_number_literals() {
  round_trip("local n = 1_000");
  round_trip("local m = 0xFF");
}

// 表构造：record / list / general / 分号分隔混排（cpp string_literals 系列）
#[test]
fn golden_table_mixed_items() {
  round_trip("local t = {a = 1, 2, [k] = v; 3}");
}

// cpp: pretty_printer_attach_types —— with_types=true 时类型注解回环
#[test]
fn golden_with_type_annotations() {
  let src = "local function f(a: number, b): (number) -> number return a end";
  let r =
    pretty_print_string_view_parse_options_bool_bool(src, ParseOptions::default(), true, false);
  assert!(r.parse_error.is_empty(), "parse error: {}", r.parse_error);
  assert_eq!(src, r.code, "typed round-trip mismatch");
}

// 带类型的记录表：cpp table-type 系列，锁定 `{x: number}` 回环
#[test]
fn golden_typed_table_type() {
  let src = "local function f(t: {x: number}): number return t.x end";
  let r =
    pretty_print_string_view_parse_options_bool_bool(src, ParseOptions::default(), true, false);
  assert!(r.parse_error.is_empty(), "parse error: {}", r.parse_error);
  assert_eq!(src, r.code, "typed table round-trip mismatch");
}

// cpp: prettyPrint_parse_error（PrettyPrinter.test.cpp:1723）—— 解析失败时
// code 为空串、parseError 携带完整错误消息。
#[test]
fn golden_parse_error_populates_error_fields() {
  let r = pretty_print_string_view_parse_options_bool_bool(
    "local a = -",
    ParseOptions::default(),
    false,
    false,
  );
  assert_eq!(r.code, "");
  assert_eq!(
    r.parse_error,
    "Expected identifier when parsing expression, got <eof>"
  );
  // 错误定位落在源码内（PrettyPrintResult 契约：取自 error.getLocation()）
  assert_eq!(r.error_location.begin.line, 0);
}

/// 回归：`"\xff\x02"` 经词法器 fixup 产生非法 UTF-8 字节序列。pretty print
/// 必须逐字节保留（cpp `std::string_view` 语义）：`StringWriter::string` 无
/// 单引号时选 `'` 引号，`0xFF` 可打印直接透传，`0x02` 控制字符走 `%03u`
/// 十进制转义 `\002`。对应 cpp PrettyPrinter.cpp StringWriter::string +
/// Luau::escape。
#[test]
fn pretty_print_preserves_arbitrary_bytes_in_string_value() {
  // Box 钉堆：AstNameTable/Parser 内部存 `*mut Allocator`（捕获宿主地址），
  // 宿主移动即悬垂（同 pretty_print_string_view_parse_options_bool_bool 先例）。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  // 源码文本 `print("\xff\x02")`：`\xff`/`\x02` 是 Lua 转义序列（源 &str
  // 本身合法 UTF-8），lexer fixup_quoted_bytes 展开后 value = [0xFF, 0x02]。
  let source = "print(\"\\xff\\x02\")";
  let parse_result = Parser::parse(
    source,
    &mut names,
    &mut allocator,
    // store_cst_data = false：无 CST 时走 StringWriter::string 路径。
    ParseOptions::default(),
  );
  assert!(
    parse_result.errors.is_empty(),
    "parse failed: {:?}",
    parse_result.errors.first().map(|e| e.what())
  );

  // Safety: parse 成功后 root 指向 arena 中存活的 AstStatBlock
  let root = unsafe { &mut *parse_result.root };
  let bytes = pretty_print_ast_stat_block_cst_node_map_bytes(root, &parse_result.cst_node_map);

  // 逐字节断言（cpp StringWriter::string 语义推演）：
  // - 引号：value 不含单引号 → 默认 `'`（仅含 `'` 时 cpp 才换 `"`）；
  // - 0xFF：`>= ' '` 且非特殊符号 → 原样透传（非法 UTF-8 不替换、不丢字节）；
  // - 0x02：控制字符 → `\` + `%03u` 十进制 `\002`；
  // - 尾部 3 空格：`advance` 按源码列位补齐列差（cpp 同款 `std::string(col, ' ')`）。
  assert_eq!(bytes, b"print('\xFF\\002')   ".as_slice());
}

// b3-T3（守护 b3-H3）：无 CST 公开 API（store_cst_data=false → lookup_cst_node
// 必 miss）下，单例字符串类型标注回退必须走 writer.string 的引号+转义通道。
#[test]
fn fallback_singleton_string_type_annotation_is_quoted_and_escaped() {
  use ulua_ast::functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block;

  // Box 钉堆：Parser 捕获 allocator 宿主地址（同上例先例）。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);

  let mut print_typed = |source: &str| -> String {
    let parse_result = Parser::parse(
      source,
      &mut names,
      &mut allocator,
      // store_cst_data = false（默认）：类型标注无 CST 节点，走回退分支。
      ParseOptions::default(),
    );
    assert!(
      parse_result.errors.is_empty(),
      "parse failed for {source:?}: {:?}",
      parse_result.errors.first().map(|e| e.what())
    );
    // Safety: parse 成功后 root 指向 arena 中存活的 AstStatBlock
    let root = unsafe { &mut *parse_result.root };
    pretty_print_with_types_ast_stat_block(root)
  };

  // 注解源码 `"a\"b"`：fixup 后 value = `a"b`，含双引号不含单引号 →
  // cpp string() 选单引号定界、escapeString 对 `"` 仍转义（cpp 同款）；
  // 未加引号的裸输出（b3-H3 旧行为）在此暴露。
  let code = print_typed("local a: \"a\\\"b\" = 1");
  assert_eq!(code, "local a: 'a\\\"b' = 1");

  // 注解源码 `"x\ny"`：value 含真实换行 → 转义为两字符 `\n`；
  // 旧字节通道会原样吐出裸换行。
  let code = print_typed("local b: \"x\\ny\" = 2");
  assert_eq!(code, "local b: 'x\\ny' = 2");
}

// 以下用例逐字取自 cpp/tests/PrettyPrinter.test.cpp 同名 TEST_CASE（opt-r17 T4
// 补覆盖）：输入与期望文本均以 cpp 出处为准，禁止凭印象改写。

// cpp: TEST_CASE("strips_type_annotations")（PrettyPrinter.test.cpp:231）
// write_types=false 时注解整段蒸发了，但源列位推进仍按原 token 位置回填空格。
#[test]
fn golden_strips_type_annotations() {
  let code = " local s: string= 'hello there' ";
  let expected = " local s        = 'hello there' ";
  assert_eq!(expected, pretty_print(code));
}

// cpp: TEST_CASE("strips_type_assertion_expressions")（PrettyPrinter.test.cpp:238）
#[test]
fn golden_strips_type_assertion_expressions() {
  let code = " local s= some_function() :: any+ something_else() :: number ";
  let expected = " local s= some_function()       + something_else()           ";
  assert_eq!(expected, pretty_print(code));
}

// cpp: TEST_CASE("function_taking_ellipsis") / ("omit_decimal_place_for_integers")
// / ("numbers") / ("infinity")（PrettyPrinter.test.cpp:245/252/694/700）
#[test]
fn golden_number_and_ellipsis_forms() {
  round_trip(" function F(...) end ");
  round_trip(" local a=5, 6, 7, 3.141, 1.1290000000000002e+45 ");
  round_trip(" local a=2510238627 ");
  round_trip(" local a = 1e500    local b = 1e400 ");
}

// cpp: TEST_CASE("table_literals") / ("more_table_literals") /
// ("table_literal_preserves_record_vs_general") / ("table_literal_with_numeric_key")
// / ("table_literal_with_keyword_key")（PrettyPrinter.test.cpp:586-616）
#[test]
fn golden_table_literal_kinds() {
  round_trip(" local t={1, 2, 3, foo='bar', baz=99,[5.5]='five point five', 'end'} ");
  round_trip(" local t={['Content-Type']='text/plain'} ");
  round_trip(" local t={['foo']='bar',quux=42} ");
  round_trip(" local t={[5]='five',[6]='six'} ");
  round_trip(" local t={['nil']=nil,['true']=true} ");
}

// cpp: TEST_CASE("a_table_key_can_be_the_empty_string")（PrettyPrinter.test.cpp:1067）
#[test]
fn golden_empty_string_table_key() {
  round_trip("local T = {[''] = true}");
}

// cpp: TEST_CASE("method_calls") / ("method_definitions")
// （PrettyPrinter.test.cpp:676/682）
#[test]
fn golden_method_index_chains() {
  round_trip(" foo.bar.baz:quux() ");
  round_trip(" function foo.bar.baz:quux() end ");
}

// cpp: TEST_CASE("spaces_between_keywords_even_if_it_pushes_the_line_estimation_off")
// （PrettyPrinter.test.cpp:688）
#[test]
fn golden_space_after_leading_dot_number() {
  round_trip(" if math.abs(raySlope) < .01 then return 0 end ");
}

// cpp: TEST_CASE("do_blocks") / ("nested_do_block")
// （PrettyPrinter.test.cpp:874/890；含 R"(...)" 的前导换行与 8 空格缩进原样）
#[test]
fn golden_do_blocks() {
  round_trip(
    r#"
        foo()

        do
            local bar=baz()
            quux()
        end

        foo2()
    "#,
  );
  round_trip(
    r#"
        do
            do
                local x = 1
            end
        end
    "#,
  );
}

// cpp: TEST_CASE("emit_a_do_block_in_cases_of_potentially_ambiguous_syntax")
// （PrettyPrinter.test.cpp:903；分号回写 + 歧义消解 do 块，守护 advance_before
// 的 has_semicolon 收口）
#[test]
fn golden_semicolon_forwards_to_next_stat() {
  round_trip(
    r#"
        f();
        (g or f)()
    "#,
  );
}

// cpp: TEST_CASE("always_emit_a_space_after_local_keyword")
// （PrettyPrinter.test.cpp:1076）
#[test]
fn golden_space_after_local_keyword_before_dotted_names() {
  round_trip("do local aZZZZ = Workspace.P1.Shape local bZZZZ = Enum.PartType.Cylinder end");
}
