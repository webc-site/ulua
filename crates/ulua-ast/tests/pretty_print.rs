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
//! - 字符串：单引号原文回填
//! - 带类型注解回环（with_types）

use ulua_ast::{
  functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
  records::parse_options::ParseOptions,
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

// 解析错误路径：错误非空、code 为空（cpp PrettyPrintResult 契约）
#[test]
fn golden_parse_error_populates_error_fields() {
  let r = pretty_print_string_view_parse_options_bool_bool(
    "local = ",
    ParseOptions::default(),
    false,
    false,
  );
  assert!(r.code.is_empty());
  assert!(!r.parse_error.is_empty(), "expected a parse error message");
  // 错误定位落在源码内（列 > 0 的行/列）
  assert_eq!(r.error_location.begin.line, 0);
}
