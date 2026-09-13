//! End-to-end CLI integration tests for the six shipping binaries.
//!
//! For each binary we exercise: `--help` / `-h`, an unknown flag, a real
//! invocation on a temp `.luau` file (asserting output + exit code), and a
//! bad / missing path. Exit codes and messages match the faithful C++ ports
//! (e.g. `ulua-reduce --help` exits 1 like the upstream `help()` which calls
//! `exit(1)`; `ulua-analyze` / `ulua-bytecode` silently skip unknown
//! `-`-prefixed args, matching `CLI/src/Analyze.cpp` / `CLI/src/Bytecode.cpp`).

use std::fs::read_to_string;
mod common;

use common::{bin, write_script};
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// ulua  (REPL / script runner)
// ---------------------------------------------------------------------------

#[test]
fn repl_help_long_exits_zero_with_usage() {
  bin("ulua")
    .arg("--help")
    .assert()
    .success()
    .stdout(predicate::str::contains("Usage:").and(predicate::str::contains("interactive REPL")));
}

#[test]
fn repl_help_short_exits_zero() {
  bin("ulua")
    .arg("-h")
    .assert()
    .success()
    .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn repl_unknown_flag_errors_nonzero() {
  bin("ulua")
    .arg("--definitely-not-a-flag")
    .assert()
    .failure()
    .stderr(predicate::str::contains("Unrecognized option"));
}

#[test]
fn repl_runs_script_file_and_prints_stdout() {
  let (_dir, path) = write_script("hello.luau", "print('hello-from-script')\n");
  bin("ulua")
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("hello-from-script"));
}

#[test]
fn repl_missing_file_errors_nonzero() {
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("nope.luau");
  bin("ulua")
    .arg(&missing)
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error opening"));
}

// ---------------------------------------------------------------------------
// ulua-analyze  (type-checker)
// ---------------------------------------------------------------------------

#[test]
fn analyze_help_exits_zero_with_usage() {
  bin("ulua-analyze")
    .arg("--help")
    .assert()
    .success()
    .stdout(predicate::str::contains("Usage:").and(predicate::str::contains("typecheck")));
}

#[test]
fn analyze_reports_type_error_on_bad_strict_file() {
  let (_dir, path) = write_script(
    "bad.luau",
    "--!strict\nlocal x: number = \"not a number\"\nreturn x\n",
  );
  // CLI/src/Analyze.cpp's reportError writes diagnostics to stderr (fprintf to
  // stderr); the process exits non-zero when there are errors.
  bin("ulua-analyze")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("TypeError").and(predicate::str::contains("number")));
}

#[test]
fn analyze_clean_on_good_strict_file() {
  let (_dir, path) = write_script("good.luau", "--!strict\nlocal x: number = 42\nreturn x\n");
  bin("ulua-analyze").arg(&path).assert().success();
}

#[test]
fn analyze_unknown_flag_is_skipped_faithfully() {
  // CLI/src/Analyze.cpp：未知 `-` 前缀选项打印错误 + 帮助并 exit 1
  bin("ulua-analyze")
    .arg("--no-such-flag")
    .assert()
    .failure()
    .stderr(predicate::str::contains(
      "Unrecognized option '--no-such-flag'.",
    ));
}

#[test]
fn analyze_missing_file_is_defined_outcome() {
  // Faithful behavior (CLI/src/Analyze.cpp): an unreadable file never produces
  // a checked-module result, so `failed` stays 0 and the process exits 0 with
  // no diagnostics. The point is a DEFINED outcome with no Rust panic — not a
  // crash on a bad path.
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("nope.luau");
  bin("ulua-analyze")
    .arg(&missing)
    .assert()
    .stderr(predicate::str::contains("panicked").not());
}

// ---------------------------------------------------------------------------
// ulua-ast  (AST dump as JSON)
// ---------------------------------------------------------------------------

#[test]
fn ast_help_exits_zero() {
  bin("ulua-ast").arg("--help").assert().success();
}

#[test]
fn ast_emits_valid_json_for_real_file() {
  let (_dir, path) = write_script("prog.luau", "return 1 + 2\n");
  let out = bin("ulua-ast").arg(&path).assert().success();
  let stdout = String::from_utf8(out.get_output().stdout.clone()).expect("utf8 stdout");
  let parsed: serde_json::Value =
    serde_json::from_str(&stdout).expect("AST output must be valid JSON");
  // The root node is an AstStatBlock.
  assert_eq!(
    parsed["root"]["type"], "AstStatBlock",
    "unexpected AST root: {parsed}"
  );
}

#[test]
fn ast_no_args_prints_help_and_exits_one() {
  // CLI/src/Ast.cpp: argc < 2 -> displayHelp + return 1.
  bin("ulua-ast").assert().failure();
}

#[test]
fn ast_missing_file_errors_nonzero() {
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("nope.luau");
  bin("ulua-ast")
    .arg(&missing)
    .assert()
    .failure()
    .stderr(predicate::str::contains("Couldn't read source"));
}

#[test]
fn ast_reports_parse_errors_nonzero() {
  let (_dir, path) = write_script("syntax.luau", "local = = =\n");
  bin("ulua-ast")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("Parse errors"));
}

// ---------------------------------------------------------------------------
// ulua-compile  (bytecode / disassembly)
// ---------------------------------------------------------------------------

#[test]
fn compile_help_exits_zero() {
  bin("ulua-compile")
    .arg("--help")
    .assert()
    .success()
    .stdout(predicate::str::contains("Available modes"));
}

#[test]
fn compile_short_help_exits_zero() {
  bin("ulua-compile").arg("-h").assert().success();
}

#[test]
fn compile_disasm_contains_return_opcode() {
  let (_dir, path) = write_script("ret.luau", "return 1 + 2\n");
  bin("ulua-compile")
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("RETURN"));
}

#[test]
fn compile_unknown_flag_errors_nonzero() {
  bin("ulua-compile")
    .arg("--definitely-not-a-flag")
    .assert()
    .failure()
    .stderr(predicate::str::contains("Unrecognized option"));
}

#[test]
fn compile_bad_optimization_level_errors() {
  let (_dir, path) = write_script("ret.luau", "return 1\n");
  bin("ulua-compile")
    .arg("-O9")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("Optimization level"));
}

#[test]
fn compile_missing_file_errors_nonzero() {
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("nope.luau");
  bin("ulua-compile").arg(&missing).assert().failure();
}

// ---------------------------------------------------------------------------
// ulua-bytecode  (bytecode summary JSON)
// ---------------------------------------------------------------------------

#[test]
fn bytecode_help_exits_zero() {
  bin("ulua-bytecode").arg("--help").assert().success();
}

#[test]
fn bytecode_writes_summary_json_file() {
  let (dir, path) = write_script("prog.luau", "local function f() return 1 end\nreturn f()\n");
  let summary = dir.path().join("summary.json");
  bin("ulua-bytecode")
    .arg(format!("--summary-file={}", summary.display()))
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("Bytecode summary written"));
  let contents = read_to_string(&summary).expect("summary file must exist");
  let parsed: serde_json::Value =
    serde_json::from_str(&contents).expect("summary must be valid JSON");
  assert!(
    parsed.is_object(),
    "summary JSON should be an object: {parsed}"
  );
}

#[test]
fn bytecode_missing_file_errors_nonzero() {
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("nope.luau");
  bin("ulua-bytecode")
    .arg(&missing)
    .current_dir(dir.path())
    .assert()
    .failure();
}

// ---------------------------------------------------------------------------
// ulua-reduce  (test-case reducer)
// ---------------------------------------------------------------------------

#[test]
fn reduce_help_exits_one_faithfully() {
  // CLI/src/Reduce.cpp `help()` calls exit(1) — faithful non-zero exit.
  bin("ulua-reduce")
    .arg("--help")
    .assert()
    .failure()
    .stdout(predicate::str::contains("Syntax:"));
}

#[test]
fn reduce_wrong_arg_count_prints_syntax_and_exits_one() {
  // Fewer than 3 positional args -> help() -> exit(1).
  bin("ulua-reduce")
    .arg("only-one-arg")
    .assert()
    .failure()
    .stdout(predicate::str::contains("Syntax:"));
}

#[test]
fn reduce_runs_three_arg_reduction_to_marker() {
  // A real reduction: the "command" greps the (rewritten) script for a marker
  // string that the reducer must preserve. The reducer overwrites the input
  // file in place, so it lives in a private temp dir.
  let (dir, path) = write_script(
    "case.luau",
    "local UNIQUE_MARKER_42 = 1\nlocal unused = 2\nlocal also_unused = 3\nreturn UNIQUE_MARKER_42\n",
  );
  // `command` uses {} as the script-path placeholder; it echoes the (reduced)
  // script to stdout, and the search text is the marker the reducer must keep
  // present for the "bug" to reproduce. The reducer runs the command through
  // the platform shell, so use each shell's file-dumping builtin: `type` under
  // cmd.exe on Windows, `cat` under sh elsewhere.
  let dump_command = if cfg!(windows) { "type {}" } else { "cat {}" };
  let assert = bin("ulua-reduce")
    .current_dir(dir.path())
    .arg(&path)
    .arg(dump_command)
    .arg("UNIQUE_MARKER_42")
    .assert()
    .success();
  assert.stdout(predicate::str::contains("Done!"));

  // The reduced script (written back to `path`) must still contain the marker.
  let reduced = read_to_string(&path).expect("reduced script must exist");
  assert!(
    reduced.contains("UNIQUE_MARKER_42"),
    "reducer dropped the marker line; reduced output:\n{reduced}"
  );
}
