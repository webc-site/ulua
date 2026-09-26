//! IO-focused end-to-end tests: reading from files and STDIN, writing output
//! files (bytecode summary, profiler dump), require-by-string across temp dirs,
//! and the hostile path cases (nonexistent / empty / directory-as-file /
//! unicode). Every case must reach a defined outcome — never a Rust panic.
//! 同形流水线经 `common::cli_case!` 夹具收口；绑定 output 续查的用例保持显式写法。

use std::{
  fs,
  fs::{create_dir, read_to_string, write},
  str::from_utf8,
};

#[macro_use]
mod common;
#[path = "common/missing_paths.rs"]
mod missing_paths;

use common::{bin, write_script};
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// File input
// ---------------------------------------------------------------------------

#[test]
fn reads_script_from_file() {
  let (_dir, path) = write_script("f.luau", "print('from-file')\n");
  cli_case!(bin("ulua").arg(&path) => success, stdout(predicate::str::contains("from-file")));
}

#[test]
fn extensionless_script_resolves_luau_then_lua() {
  // cpp Repl.cpp:570 `getFilePath`：依次试 name、name.luau、name.lua
  let (dir, _script) = write_script("prog.luau", "print('via-luau')\n");
  cli_case!(bin("ulua").arg(dir.path().join("prog")) => success,
    stdout(predicate::str::contains("via-luau")));

  let (dir, _script) = write_script("prog.lua", "print('via-lua')\n");
  cli_case!(bin("ulua").arg(dir.path().join("prog")) => success,
    stdout(predicate::str::contains("via-lua")));
}

#[test]
fn shebang_preserves_diagnostic_line() {
  let (_dir, path) = write_script("shebang.luau", "#!/usr/bin/env luau\nlocal =\n");
  for name in ["ulua", "ulua-ast", "ulua-compile", "ulua-analyze"] {
    cli_case!(bin(name).arg(&path) => failure,
      stderr(predicate::str::contains(":2:").or(predicate::str::contains("(2,"))));
  }
}

#[test]
fn shebang_without_newline_is_empty_source() {
  let (dir, path) = write_script("shebang.luau", "#!/usr/bin/env luau");
  for name in [
    "ulua",
    "ulua-ast",
    "ulua-compile",
    "ulua-analyze",
    "ulua-bytecode",
  ] {
    cli_case!(bin(name).current_dir(dir.path()).arg(&path) => success);
  }
}

#[test]
fn empty_file_runs_cleanly() {
  let (_dir, path) = write_script("empty.luau", "");
  cli_case!(bin("ulua").arg(&path) => success, stdout(""));
}

#[test]
fn whitespace_only_file_runs_cleanly() {
  let (_dir, path) = write_script("ws.luau", "   \n\t\n  \n");
  cli_case!(bin("ulua").arg(&path) => success, stdout(""));
}

// ---------------------------------------------------------------------------
// STDIN
// ---------------------------------------------------------------------------

#[test]
fn reads_and_runs_script_from_stdin() {
  // Piping into `ulua` with no file args drives the REPL loop, which reads
  // each line, evaluates it, and prints expression results. rustyline falls
  // back to plain line reading for a non-TTY stdin.
  cli_case!(bin("ulua").write_stdin("print('from-stdin')\n") => success,
    stdout(predicate::str::contains("from-stdin")));
}

#[test]
fn stdin_expression_result_is_echoed() {
  // The REPL tries `return <line>` first, so a bare expression prints its value.
  cli_case!(bin("ulua").write_stdin("2 + 3\n") => success,
  stdout(predicate::function(|out: &str| {
    out.lines().any(|line| line.trim() == "5")
  })));
}

#[test]
fn ast_reads_source_from_stdin_dash() {
  // CLI/src/Ast.cpp：`-` 参数表示从 stdin 读源码
  let out = cli_case!(bin("ulua-ast").arg("-").write_stdin("return 1\n") => success);
  let stdout = from_utf8(&out.get_output().stdout).expect("utf8 stdout");
  let parsed: sonic_rs::Value = sonic_rs::from_str(stdout).expect("valid JSON from stdin AST");
  assert_eq!(parsed["root"]["type"], "AstStatBlock");
}

// ---------------------------------------------------------------------------
// File output
// ---------------------------------------------------------------------------

#[test]
fn bytecode_summary_defaults_to_bytecode_summary_json_in_cwd() {
  // cpp Bytecode.cpp:273 `std::string summaryFile("bytecode-summary.json")`：
  // 不给 `--summary-file=` 时落 CWD 默认文件，:296 打
  // "Bytecode summary written to 'bytecode-summary.json'"。
  // 产物版式按 serializeSummaries/serializeScriptSummary/serializeFunctionSummary
  // 的 fprintf 字面量锁（Bytecode.cpp:190-257）：顶层 `{` 起、`}` 止，
  // 键为 escapeFilename 后的源路径，函数条目五字段成序。
  let (dir, path) = write_script("p.luau", "return 1 + 1\n");
  cli_case!(bin("ulua-bytecode").current_dir(dir.path()).arg(&path) => success,
  stdout(
    predicate::str::is_match(r"Bytecode summary written to 'bytecode-summary\.json'\n")
      .expect("summary 提示行 regex 合法"),
  ));
  let contents =
    read_to_string(dir.path().join("bytecode-summary.json")).expect("默认摘要文件应存在");
  // escapeFilename（Bytecode.cpp:161-180）：`\\`→`/`，`"` 加转义；临时路径无引号
  let escaped = path.to_string_lossy().replace('\\', "/");
  assert!(
    contents.starts_with("{\n"),
    "摘要应以 '{{' 起始: {contents}"
  );
  assert!(
    contents.contains(&format!("    \"{escaped}\": [\n")),
    "顶层键应为转义后的源路径: {contents}"
  );
  for key in [
    "\"source\":",
    "\"name\":",
    "\"line\":",
    "\"nestingLimit\":",
    "\"counts\": [",
  ] {
    assert!(contents.contains(key), "函数条目缺字段 {key}: {contents}");
  }
  assert!(
    contents.ends_with("    ]\n}"),
    "摘要应以 ']\\n}}' 收束: {contents}"
  );
}

#[test]
fn profile_writes_profile_out_in_cwd() {
  // CLI/src/Repl.cpp:869 总是把 profiler 转储到 CWD 下的 "profile.out"。
  // stdout 汇总行按 cpp Profiler.cpp:135-140 的 printf 逐字段锁格式
  //（%.3f → 三位小数；%lld → 十进制计数），原实现只断言文件存在、内容
  // 读了扔掉（`let _ =` 零断言）。
  let (dir, path) = write_script(
    "work.luau",
    "local s=0\nfor i=1,50000 do s=s+i end\nreturn s\n",
  );
  cli_case!(bin("ulua").current_dir(dir.path()).arg("--profile").arg(&path) => success,
    stdout(predicate::str::is_match(
      r"Profiler dump written to profile\.out \(total runtime \d+\.\d{3} seconds, \d+ samples, \d+ stacks\)"
    ).expect("profile 汇总行 regex 合法")));
  let profile = dir.path().join("profile.out");
  assert!(profile.exists(), "profile.out not written");
  // cpp Profiler.cpp:127 每条栈写一行 `"%lld %s\n"`（计数 + 空格 + 栈）。
  // 采样行数依赖时序（可以合法为 0 行），行数不设下限，但凡出现的行必须合此格式。
  for line in read_to_string(&profile)
    .expect("profile.out 应可读")
    .lines()
  {
    let (count, stack) = line
      .split_once(' ')
      .unwrap_or_else(|| panic!("profile 行缺空格分隔: {line:?}"));
    assert!(
      !count.is_empty() && count.chars().all(|c| c.is_ascii_digit()),
      "profile 行首字段应为十进制计数: {line:?}"
    );
    assert!(!stack.is_empty(), "profile 行栈字段不应为空: {line:?}");
  }
}

#[test]
fn coverage_writes_coverage_out_in_cwd() {
  // Repl.cpp:873 `coverageDump("coverage.out")`。stdout 汇总行按
  // Coverage.cpp:87 `printf("Coverage dump written to %s (%d functions)\n")` 锁；
  // 转储文件版式按 Coverage.cpp:69/76/78 的 fprintf 字面量锁：首行 "TN:"，
  // 每条记录以 "SF:" 起、"end_of_record" 止；depth 0 主块函数名恒为 "<main>"
  //（Coverage.cpp:35-36/44），本脚本恰有一条主块记录，FN/FNDA 行必现
  //（Coverage.cpp:44-50）。原断言只盯 "functions)" 子串与「非空」。
  let (dir, path) = write_script("cov.luau", "local s=0\nfor i=1,10 do s=s+i end\nreturn s\n");
  cli_case!(bin("ulua").current_dir(dir.path()).arg("--coverage").arg(&path) => success,
    stdout(predicate::str::is_match(r"Coverage dump written to coverage\.out \(\d+ functions\)").expect("coverage 汇总行 regex 合法")));
  let dump = dir.path().join("coverage.out");
  let text = read_to_string(&dump).expect("coverage.out 应存在且可读");
  assert!(
    text.starts_with("TN:\n"),
    "coverage 转储应以 TN: 起始: {text}"
  );
  assert!(
    text.lines().nth(1).is_some_and(|l| l.starts_with("SF:")),
    "TN: 后应紧跟 SF: 记录头: {text}"
  );
  assert!(text.contains(",<main>"), "主块函数名应为 <main>: {text}");
  assert!(text.contains("FNDA:"), "已执行函数应有 FNDA 命中行: {text}");
  assert!(
    text.ends_with("end_of_record\n"),
    "记录应以 end_of_record 收束: {text}"
  );
}

#[test]
fn counters_writes_callgrind_out_in_cwd() {
  // Repl.cpp:876 `countersDump("callgrind.out")`；Counters.cpp:115-117 的
  // callgrind 头三行是逐字契约（version/creator/events）
  let (dir, path) = write_script("cnt.luau", "local s=0\nfor i=1,10 do s=s+i end\nreturn s\n");
  cli_case!(bin("ulua").current_dir(dir.path()).arg("--counters").arg(&path) => success,
    stdout(predicate::str::contains("Counters data written to callgrind.out")));
  let text = fs::read_to_string(dir.path().join("callgrind.out")).expect("callgrind.out 可读");
  assert!(
    text.starts_with("version: 1\ncreator: Luau REPL\nevents: Regular Fallback VmExit\n"),
    "callgrind 头不符: {text}"
  );
}

// ---------------------------------------------------------------------------
// require-by-string across a temp dir
// ---------------------------------------------------------------------------

#[test]
fn require_relative_module_across_temp_dir() {
  let dir = tempfile::tempdir().unwrap();
  write(dir.path().join("dep.luau"), "return { value = 99 }\n").unwrap();
  let main = dir.path().join("main.luau");
  write(
    &main,
    "local d = require('./dep')\nprint('got ' .. tostring(d.value))\n",
  )
  .unwrap();
  cli_case!(bin("ulua").current_dir(dir.path()).arg(&main) => success,
    stdout(predicate::str::contains("got 99")));
}

// ---------------------------------------------------------------------------
// Hostile paths — defined outcome, never a panic
// ---------------------------------------------------------------------------

#[test]
fn nonexistent_file_errors_without_panic() {
  let (_dir, missing) = missing_paths::not_exist("ghost.luau");
  cli_case!(bin("ulua").arg(&missing) => failure,
  stderr(
    predicate::str::contains("Error opening").and(predicate::str::contains("panicked").not())
  ));
}

#[test]
fn directory_as_file_is_defined_outcome() {
  // get_source_files() traverses a directory for .lua/.luau files; an empty
  // directory simply yields no work. Faithful, and must not crash.
  let dir = tempfile::tempdir().unwrap();
  let as_file = dir.path().join("adir.luau");
  create_dir(&as_file).unwrap();
  cli_case!(bin("ulua").arg(&as_file) => success, stderr(predicate::str::contains("panicked").not()));
}

#[test]
fn unicode_path_runs_cleanly() {
  let dir = tempfile::tempdir().unwrap();
  let path = dir.path().join("café_файл_🦀.luau");
  write(&path, "print('unicode-path-ok')\n").unwrap();
  cli_case!(bin("ulua").arg(&path) => success, stdout(predicate::str::contains("unicode-path-ok")));
}

#[test]
fn compile_directory_as_file_no_panic() {
  let dir = tempfile::tempdir().unwrap();
  let as_file = dir.path().join("d.luau");
  create_dir(&as_file).unwrap();
  // Defined outcome (no .luau files inside) — must not panic.
  cli_case!(bin("ulua-compile").arg(&as_file) => success,
    stderr(predicate::str::contains("panicked").not()));
}
