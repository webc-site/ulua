//! `ulua-bytecode --bytecode-summary` 的输出契约测试。
//!
//! 对照上游 `CLI/src/Bytecode.cpp`：`parseArgs`（47-97）、`serializeFunctionSummary`
//! /`serializeScriptSummary`/`serializeSummaries`（184-267）。这些函数用 `fprintf`
//! 逐字符固定缩进与分隔符，改 `get_counts`/循环边界就会静默产出坏 JSON，故此处锁全文。

use ulua_cli_lib::test_utils::{Workspace, code, normalize_digits, stderr_of, stdout_of};
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

const BIN: &str = env!("CARGO_BIN_EXE_ulua-bytecode");

/// cpp `Bytecode.cpp:273` 的默认汇总文件名（= 本 crate `DEFAULT_SUMMARY_FILE`，
/// 该常量为 `pub(crate)`，测试侧按上游同名值断言）
const DEFAULT_SUMMARY_FILE: &str = "bytecode-summary.json";

/// 单个函数一行计数的项数：与 `getOpLimit()` 同源，避免测试里再抄一遍魔数
const OP_LIMIT: usize = FunctionBytecodeSummary::LOP__COUNT as usize;

/// 每个用例独占的工作目录（共享夹具，temp 目录以本 crate 名前缀隔离）
fn ws(name: &str) -> Workspace {
  Workspace::new(BIN, "ulua-bytecode-cli", name)
}

/// `serializeFunctionSummary` 的 `counts` 行：`OP_LIMIT` 项以 `", "` 分隔，行首 16 空格
fn counts_row() -> String {
  let mut row = String::from("                [");
  for i in 0..OP_LIMIT {
    if i > 0 {
      row.push_str(", ");
    }
    row.push_str("<n>");
  }
  row.push(']');
  row
}

/// 单个函数对象（缩进 8 空格，与 cpp `fprintf` 逐字一致）
fn function_entry(name: &str) -> String {
  format!(
    r#"        {{
            "source": "[string]",
            "name": "{name}",
            "line": <n>,
            "nestingLimit": <n>,
            "counts": [
{counts}
            ]
        }}"#,
    counts = counts_row()
  )
}

/// 单个脚本条目：`"<file>": [ ...functions... ]`
fn script_entry(file: &str, names: &[&str]) -> String {
  let functions = names
    .iter()
    .map(|name| function_entry(name))
    .collect::<Vec<_>>()
    .join(",\n");

  format!("    \"{file}\": [\n{functions}\n    ]")
}

/// 顶层文档：`{ ...scripts... }`，条目间 `,`
fn document(entries: &[String]) -> String {
  format!("{{\n{}\n}}", entries.join(",\n"))
}

/// cpp `serializeSummaries`：默认汇总文件名与整份 JSON 的形状
#[test]
fn summary_default_file_matches_upstream_layout() {
  let ws = ws("default-file");
  ws.write(
    "a.luau",
    "local function add(a, b)\n  return a + b\nend\n\nreturn add(1, 2)\n",
  );

  let output = ws.run(&["a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert_eq!(
    stdout_of(&output),
    format!("Bytecode summary written to '{DEFAULT_SUMMARY_FILE}'\n")
  );

  let raw = ws.read(DEFAULT_SUMMARY_FILE);
  assert_eq!(
    normalize_digits(&raw),
    document(&[script_entry("./a.luau", &["add", ""])])
  );
  // 结构之外钉住真实内容：函数名取自源码，计数行长度由 `get_counts`/`get_op_limit` 决定
  assert!(raw.contains("\"name\": \"add\","), "raw:\n{raw}");
  let row = raw
    .lines()
    .find(|line| line.trim_start().starts_with('['))
    .expect("counts row");
  assert_eq!(
    row
      .trim()
      .trim_matches(|ch| ch == '[' || ch == ']')
      .split(", ")
      .count(),
    OP_LIMIT,
    "counts 行长度应等于 opLimit"
  );
}

/// 多个脚本 → `serializeSummaries` 的条目分隔符为 `,`，最后一个不带
#[test]
fn summary_maps_every_input_file_in_order() {
  let ws = ws("two-files");
  ws.write("a.luau", "return 1\n");
  ws.write("b.luau", "return 2\n");

  assert_eq!(
    code(&ws.run(&["--summary-file=s.json", "a.luau", "b.luau"])),
    0
  );

  let raw = ws.read("s.json");
  assert_eq!(
    normalize_digits(&raw),
    document(&[
      script_entry("./a.luau", &[""]),
      script_entry("./b.luau", &[""]),
    ])
  );
  assert!(raw.ends_with("\n    ]\n}"), "raw:\n{raw}");
}

/// cpp `parseArgs`：`--summary-file=` 缺文件名 → 退出 1
#[test]
fn summary_file_without_name_exits_one() {
  let ws = ws("no-name");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--summary-file=", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "Error: filename missing for '--summary-file'.\n\n"
  );
}

/// cpp `parseArgs`：级别越界直接退出 1（`?` 短路到 `main` 的 return 1）
#[test]
fn out_of_range_optimization_level_exits_one() {
  let ws = ws("bad-level");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["-O5", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "Error: Optimization level must be between 0 and 2 inclusive.\n"
  );
}

/// 读不到的源文件：`analyzeFile` 返回 nullopt → 退出 1，且不产出汇总
#[test]
fn unreadable_source_exits_one_without_artifact() {
  let ws = ws("missing-source");
  let output = ws.run(&["nosuch.luau"]);

  assert_eq!(code(&output), 1);
  assert_eq!(stderr_of(&output), "Error opening ./nosuch.luau\n");
  assert!(!ws.exists(DEFAULT_SUMMARY_FILE));
}

/// 语法错误：`reportParseError` 打印 `file(line,col): SyntaxError: ...` 后退出 1
#[test]
fn parse_error_exits_one() {
  let ws = ws("parse-error");
  ws.write("bad.luau", "return ~1\n");

  let output = ws.run(&["bad.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "./bad.luau(1,8): SyntaxError: Expected identifier when parsing expression, got '~'\n"
  );
  assert!(!ws.exists(DEFAULT_SUMMARY_FILE));
}

/// cpp `displayHelp` 内部 `exit(0)`：`--help` 退出码为 0
#[test]
fn help_exits_zero() {
  let ws = ws("help");
  let output = ws.run(&["--help"]);

  assert_eq!(code(&output), 0);
  assert!(stdout_of(&output).contains("--summary-file=<filename>"));
}
