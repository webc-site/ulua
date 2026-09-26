//! `ulua-ast` 的输出契约测试：JSON 结构逐字快照、`-`（stdin）入口、解析错误的退出码。
//!
//! 对照上游 `CLI/src/Ast.cpp:24-90`：`toJson(parseResult.root, parseResult.commentLocations)`
//! 打到 stdout，解析错误逐条打到 stderr 后额外一个空行，最后
//! `return parseResult.errors.size() > 0 ? 1 : 0`。

use ulua_cli_lib::test_utils::{Workspace, code, stderr_of, stdout_of};

const BIN: &str = env!("CARGO_BIN_EXE_ulua-ast");

/// 每个用例独占的工作目录（共享夹具，temp 目录以本 crate 名前缀隔离）
fn ws(name: &str) -> Workspace {
  Workspace::new(BIN, "ulua-ast-cli", name)
}

/// `local x = 1` 的 AST JSON 完全确定，直接逐字快照（键序即 `toJson` 的输出契约）
#[test]
fn valid_source_dumps_expected_json() {
  let ws = ws("valid");
  ws.write("a.luau", "local x = 1\n");

  let output = ws.run(&["a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert_eq!(
    stdout_of(&output),
    r#"{"root":{"type":"AstStatBlock","location":"0,0 - 1,0","hasEnd":true,"body":[{"type":"AstStatLocal","location":"0,0 - 0,11","vars":[{"luauType":null,"name":"x","isConst":false,"type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprConstantNumber","location":"0,10 - 0,11","value":1}]}]},"commentLocations":[]}"#
  );
  assert_eq!(stderr_of(&output), "");
}

/// `-` 走 `readStdin()`，与文件入口产出同一份 JSON
#[test]
fn dash_reads_source_from_stdin() {
  let ws = ws("stdin");
  let output = ws.run_with_stdin(&["-"], "local x = 1\n");

  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert!(
    stdout_of(&output).contains("\"name\":\"x\""),
    "JSON 未含变量名"
  );
}

/// cpp Ast.cpp:31-40：解析错误逐条报告后仍打印 JSON，并以 1 退出
#[test]
fn parse_error_is_reported_and_exit_code_is_one() {
  let ws = ws("parse-error");
  ws.write("bad.luau", "return ~1\n");

  let output = ws.run(&["bad.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "Parse errors were encountered:\n  (0, 7) - (0, 8) - Expected identifier when parsing expression, got '~'\n\n"
  );
  // 出错部分被 `AstExprError` 取代，但整棵 AST 仍然导出
  assert!(
    stdout_of(&output).contains("\"type\":\"AstExprError\""),
    "stdout: {}",
    stdout_of(&output)
  );
}

/// 热注释（`--!strict`）不进 AST 主体，只出现在 `commentLocations`
#[test]
fn comment_locations_are_exported() {
  let ws = ws("comments");
  ws.write("a.luau", "-- hi\nlocal x = 1\n");

  let output = ws.run(&["a.luau"]);
  assert_eq!(code(&output), 0);
  // 注释单独成表，不进 AST 主体
  assert!(
    stdout_of(&output)
      .ends_with(r#""commentLocations":[{"type":"Comment","location":"0,0 - 0,5"}]}"#),
    "stdout: {}",
    stdout_of(&output)
  );
  assert!(
    stdout_of(&output).contains(r#""location":"1,0 - 1,11""#),
    "注释后的语句行号应右移: stdout: {}",
    stdout_of(&output)
  );
}

/// cpp Ast.cpp:26-30：`--help` 打印用法并退出 0
#[test]
fn help_exits_zero_with_usage() {
  let ws = ws("help");
  let output = ws.run(&["--help"]);

  assert_eq!(code(&output), 0);
  assert!(
    stdout_of(&output).starts_with("Usage: "),
    "stdout: {output:?}"
  );
}
