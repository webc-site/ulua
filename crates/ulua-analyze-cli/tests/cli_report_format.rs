//! `ulua-analyze` 报告格式与退出码的行为级测试。
//!
//! 对照上游 `CLI/src/Analyze.cpp`：`report`（52-70 三种 ReportFormat 的排版）、
//! `reportModuleResult`（91-129）、`main` 末尾的 checkedNames/skippedNames 统计与
//! `return format == Luacheck ? 0 : (failed != 0)`。子进程执行以拿到 stdout/stderr 分流。

use ulua_cli_lib::test_utils::{Workspace, code, stderr_of, stdout_of};

const BIN: &str = env!("CARGO_BIN_EXE_ulua-analyze");

/// lint 警告（单行）：`Variable 'unused' ...`
const LINT_SOURCE: &str =
  "local function f(a)\n  local unused = 1\n  return a\nend\n\nlocal g = f\n";
/// strict 模式下的类型错误，且跨越多个源行（触发 luacheck 的「end column 100」分支）
const TYPE_SOURCE: &str =
  "--!strict\nlocal function f(a: number)\n  return a + \"x\"\nend\n\nreturn f(1)\n";

/// 每个用例独占的工作目录（共享夹具，temp 目录以本 crate 名前缀隔离）
fn ws(name: &str) -> Workspace {
  Workspace::new(BIN, "ulua-analyze-cli", name)
}

/// cpp `ReportFormat::Default`：`name(line,col): Type: message`，写 stderr
#[test]
fn default_format_reports_start_position_on_stderr() {
  let ws = ws("default");
  ws.write("a.luau", LINT_SOURCE);

  let output = ws.run(&["a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert_eq!(
    stderr_of(&output),
    "./a.luau(2,9): LocalUnused: Variable 'unused' is never used; prefix with '_' to silence\n\
     ./a.luau(6,7): LocalUnused: Variable 'g' is never used; prefix with '_' to silence\n"
  );
  assert_eq!(stdout_of(&output), "");
}

/// cpp `ReportFormat::Gnu`：`name:L.C-E.C: Type: message`（end column 为闭区间）
#[test]
fn gnu_format_reports_inclusive_range_on_stderr() {
  let ws = ws("gnu");
  ws.write("a.luau", LINT_SOURCE);

  let output = ws.run(&["--formatter=gnu", "a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert_eq!(
    stderr_of(&output),
    "./a.luau:2.9-2.14: LocalUnused: Variable 'unused' is never used; prefix with '_' to silence\n\
     ./a.luau:6.7-6.7: LocalUnused: Variable 'g' is never used; prefix with '_' to silence\n"
  );
}

/// cpp `ReportFormat::Luacheck`（`--formatter=plain`）：走 **stdout**，格式 `L:C-E: (W0) ...`
#[test]
fn luacheck_format_reports_on_stdout() {
  let ws = ws("luacheck");
  ws.write("a.luau", LINT_SOURCE);

  let output = ws.run(&["--formatter=plain", "a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert_eq!(stderr_of(&output), "");
  assert_eq!(
    stdout_of(&output),
    "./a.luau:2:9-14: (W0) LocalUnused: Variable 'unused' is never used; prefix with '_' to silence\n\
     ./a.luau:6:7-7: (W0) LocalUnused: Variable 'g' is never used; prefix with '_' to silence\n"
  );
}

/// cpp Analyze.cpp:264-268：luacheck 格式下无论多少错误都退出 0（供流水线消费）
#[test]
fn luacheck_format_always_exits_zero() {
  let ws = ws("luacheck-zero");
  ws.write("a.luau", TYPE_SOURCE);

  let strict = ws.run(&["--mode=strict", "a.luau"]);
  assert_eq!(code(&strict), 1, "stderr: {}", stderr_of(&strict));

  let plain = ws.run(&["--mode=strict", "--formatter=plain", "a.luau"]);
  assert_eq!(code(&plain), 0);
  assert!(!stdout_of(&plain).is_empty(), "应仍报告错误");
}

/// cpp `report`：跨行错误的 luacheck end column 伪造为 100
#[test]
fn luacheck_format_fakes_end_column_for_multiline_error() {
  let ws = ws("luacheck-multiline");
  ws.write("a.luau", TYPE_SOURCE);

  let output = ws.run(&["--mode=strict", "--formatter=plain", "a.luau"]);
  assert!(
    stdout_of(&output).starts_with("./a.luau:2:1-100: (W0) TypeError: "),
    "stdout: {}",
    stdout_of(&output)
  );
  assert!(
    stdout_of(&output).contains("./a.luau:3:10-16: (W0) TypeError: "),
    "stdout: {}",
    stdout_of(&output)
  );
}

/// cpp `reportModuleResult` 的返回值决定退出码：有 TypeError → 1
#[test]
fn type_errors_make_default_format_exit_one() {
  let ws = ws("type-error");
  ws.write("a.luau", TYPE_SOURCE);

  let output = ws.run(&["--mode=strict", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "./a.luau(2,1): TypeError: Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add\n\
     ./a.luau(3,10): TypeError: Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add\n\
     ./a.luau(2,1): TypeError: Consider annotating the return with number\n\
     ./a.luau(6,8): TypeError: Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add\n"
  );
}

/// cpp `main` 的 skippedNames 分支：读不到的文件报 `Error opening` 并计入 failed
#[test]
fn unreadable_file_is_counted_as_failure() {
  let ws = ws("unreadable");

  let output = ws.run(&["nosuch.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(stderr_of(&output), "Error opening ./nosuch.luau\n");
}

/// cpp `main`：部分成功（一个可检查、一个打不开）仍整体退出 1
#[test]
fn partially_successful_run_exits_one() {
  let ws = ws("partial");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["a.luau", "nosuch.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(stderr_of(&output), "Error opening ./nosuch.luau\n");
}

/// cpp `main`：`checkedNames` 是 `std::set`，重复命令行路径不会被算成打开失败
#[test]
fn duplicated_file_argument_is_deduplicated() {
  let ws = ws("duplicate");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["a.luau", "a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert!(!stderr_of(&output).contains("Error opening"));
}

/// cpp `main:131-141`：`--annotate` 走 `attachTypeData` + `prettyPrintWithTypes`，
/// 结果打到 stdout，且不影响退出码
#[test]
fn annotate_prints_typed_pretty_printed_source() {
  let ws = ws("annotate");
  ws.write("a.luau", LINT_SOURCE);

  let output = ws.run(&["--annotate", "a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));

  let annotated = stdout_of(&output);
  // 类型注记出现在 pretty print 结果里（`local unused:number=1`）
  assert!(
    annotated.contains("local unused:number=1"),
    "stdout: {annotated}"
  );
  assert!(
    annotated.contains("local function f(a:a):"),
    "stdout: {annotated}"
  );
  // 默认格式的诊断仍走 stderr
  assert!(stderr_of(&output).contains("./a.luau(2,9): LocalUnused:"));
}

/// cpp `main`：`-j` 在本 build 不决定并发度，必须显式说明未生效
#[test]
fn thread_count_note_is_reported_when_above_one() {
  let ws = ws("threads");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["-j4", "a.luau"]);
  assert_eq!(code(&output), 0);
  assert_eq!(
    stderr_of(&output),
    "note: -j is not effective in this build; typechecking runs single-threaded\n"
  );

  // `-j1` 与未给 `-j` 一样安静
  assert_eq!(stderr_of(&ws.run(&["-j1", "a.luau"])), "");
}

/// cpp `main:120-127`：未知选项 → 报错 + help + 退出 1
#[test]
fn unrecognized_option_exits_one() {
  let ws = ws("unknown-option");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--definitely-not-an-option", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert!(
    stderr_of(&output).starts_with("Error: Unrecognized option '--definitely-not-an-option'.\n\n"),
    "stderr: {}",
    stderr_of(&output)
  );
  assert!(stdout_of(&output).contains("Usage:"));
}
