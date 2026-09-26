//! `ulua-reduce` 的参数与早退路径测试。
//!
//! 对照上游 `CLI/src/Reduce.cpp:476-513`：`args.size() != 4` 或任一参数为 `--help`
//! 都走 `help()`（**退出码 1**，不是 0——上游 `help` 是 `[[noreturn]] exit(1)`）；
//! 读不到脚本 → "Could not read source" + 退出 1；原始脚本里找不到 bug 特征 → 退出 2。
//! `run()` 内部直接 `process::exit`，故只能子进程测。

use ulua_cli_lib::test_utils::{Workspace, code, stdout_of};

const BIN: &str = env!("CARGO_BIN_EXE_ulua-reduce");

/// 把脚本原样打回 stdout 的命令模板（`{}` 由 reducer 替换成脚本路径）
#[cfg(windows)]
const SHOW_CMD: &str = "type {}";
#[cfg(not(windows))]
const SHOW_CMD: &str = "cat {}";

/// 每个用例独占的工作目录（共享夹具，temp 目录以本 crate 名前缀隔离）
fn ws(name: &str) -> Workspace {
  Workspace::new(BIN, "ulua-reduce-cli", name)
}

/// cpp `Reduce.cpp:489-490`：参数个数不是 4 → Syntax，退出 1
#[test]
fn wrong_argument_count_prints_syntax_and_exits_one() {
  let ws = ws("argc");

  for args in [
    vec![],
    vec!["a.lua".to_string()],
    vec!["a.lua".to_string(), "cat {}".to_string()],
  ] {
    let strs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output = ws.run(&strs);
    assert_eq!(code(&output), 1, "{args:?} 应退出 1");
    // cpp 用 printf（stdout），且第一行含 argv[0]
    let printed = stdout_of(&output);
    assert!(printed.starts_with("Syntax: "), "stdout: {printed}");
    assert!(
      printed.contains("script command \"search text\""),
      "stdout: {printed}"
    );
    assert!(printed.contains("{} as a stand-in"), "stdout: {printed}");
  }
}

/// cpp `Reduce.cpp:492-497`：`--help` 也走同一个 `help()`，退出码同样是 1
#[test]
fn help_flag_exits_one_like_upstream() {
  let ws = ws("help");
  let output = ws.run(&["--help"]);

  assert_eq!(code(&output), 1);
  assert!(stdout_of(&output).starts_with("Syntax: "));
}

/// 四元组里混入 `--help`：仍按 Syntax 处理（cpp 的逐参数扫描）
#[test]
fn help_flag_among_four_arguments_exits_one() {
  let ws = ws("help-in-args");
  ws.write("a.lua", "print(1)\n");

  let output = ws.run(&["a.lua", "--help", "REDUCE_ME"]);
  assert_eq!(code(&output), 1);
  assert!(stdout_of(&output).starts_with("Syntax: "));
  // 不应产生任何缩减动作
  assert_eq!(ws.read("a.lua"), "print(1)\n");
}

/// cpp `Reduce.cpp:503-507`：读不到脚本 → stdout 提示 + 退出 1
#[test]
fn unreadable_script_exits_one() {
  let ws = ws("missing-script");
  let output = ws.run(&["nosuch.lua", SHOW_CMD, "REDUCE_ME"]);

  assert_eq!(code(&output), 1);
  assert_eq!(stdout_of(&output), "Could not read source nosuch.lua\n");
}

/// cpp `Reduce.cpp:510-513` 前置检查：原脚本里找不到特征串 → 退出 2
#[test]
fn missing_bug_marker_exits_two() {
  let ws = ws("no-bug");
  ws.write("a.lua", "print(1)\n");

  let output = ws.run(&["a.lua", SHOW_CMD, "REDUCE_ME"]);
  assert_eq!(code(&output), 2);
  assert!(
    stdout_of(&output).contains("Could not find failure string"),
    "stdout: {}",
    stdout_of(&output)
  );
}
