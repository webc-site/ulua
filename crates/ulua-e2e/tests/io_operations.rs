//! IO-focused end-to-end tests: reading from files and STDIN, writing output
//! files (bytecode summary, profiler dump), require-by-string across temp dirs,
//! and the hostile path cases (nonexistent / empty / directory-as-file /
//! unicode). Every case must reach a defined outcome — never a Rust panic.

use std::fs::{File, create_dir, read_to_string, write};
mod common;

use std::io::Write;

use common::{bin, write_script};
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// File input
// ---------------------------------------------------------------------------

#[test]
fn reads_script_from_file() {
  let (_dir, path) = write_script("f.luau", "print('from-file')\n");
  bin("ulua")
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("from-file"));
}

#[test]
fn empty_file_runs_cleanly() {
  let (_dir, path) = write_script("empty.luau", "");
  bin("ulua").arg(&path).assert().success();
}

#[test]
fn whitespace_only_file_runs_cleanly() {
  let (_dir, path) = write_script("ws.luau", "   \n\t\n  \n");
  bin("ulua").arg(&path).assert().success();
}

// ---------------------------------------------------------------------------
// STDIN
// ---------------------------------------------------------------------------

#[test]
fn reads_and_runs_script_from_stdin() {
  // Piping into `ulua` with no file args drives the REPL loop, which reads
  // each line, evaluates it, and prints expression results. rustyline falls
  // back to plain line reading for a non-TTY stdin.
  bin("ulua")
    .write_stdin("print('from-stdin')\n")
    .assert()
    .success()
    .stdout(predicate::str::contains("from-stdin"));
}

#[test]
fn stdin_expression_result_is_echoed() {
  // The REPL tries `return <line>` first, so a bare expression prints its value.
  bin("ulua")
    .write_stdin("2 + 3\n")
    .assert()
    .success()
    .stdout(predicate::str::contains("5"));
}

#[test]
fn ast_reads_source_from_stdin_dash() {
  // CLI/src/Ast.cpp: a "-" argument reads the source from stdin.
  let out = bin("ulua-ast")
    .arg("-")
    .write_stdin("return 1\n")
    .assert()
    .success();
  let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON from stdin AST");
  assert_eq!(parsed["root"]["type"], "AstStatBlock");
}

// ---------------------------------------------------------------------------
// File output
// ---------------------------------------------------------------------------

#[test]
fn bytecode_summary_writes_to_chosen_path() {
  let (dir, path) = write_script("p.luau", "return 1 + 1\n");
  let summary = dir.path().join("out.json");
  bin("ulua-bytecode")
    .arg(format!("--summary-file={}", summary.display()))
    .arg(&path)
    .assert()
    .success();
  assert!(summary.exists(), "summary file not written");
  let parsed: serde_json::Value = serde_json::from_str(&read_to_string(&summary).unwrap()).unwrap();
  assert!(parsed.is_object());
}

#[test]
fn profile_writes_profile_out_in_cwd() {
  // CLI/src/Repl.cpp always dumps the profiler to "profile.out" in the CWD.
  let dir = tempfile::tempdir().unwrap();
  let path = dir.path().join("work.luau");
  let mut f = File::create(&path).unwrap();
  f.write_all(b"local s=0\nfor i=1,50000 do s=s+i end\nreturn s\n")
    .unwrap();
  f.flush().unwrap();
  bin("ulua")
    .current_dir(dir.path())
    .arg("--profile")
    .arg(&path)
    .assert()
    .success();
  assert!(
    dir.path().join("profile.out").exists(),
    "profile.out not written"
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
  bin("ulua")
    .current_dir(dir.path())
    .arg(&main)
    .assert()
    .success()
    .stdout(predicate::str::contains("got 99"));
}

// ---------------------------------------------------------------------------
// Hostile paths — defined outcome, never a panic
// ---------------------------------------------------------------------------

#[test]
fn nonexistent_file_errors_without_panic() {
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("ghost.luau");
  bin("ulua").arg(&missing).assert().failure().stderr(
    predicate::str::contains("Error opening").and(predicate::str::contains("panicked").not()),
  );
}

#[test]
fn directory_as_file_is_defined_outcome() {
  // get_source_files() traverses a directory for .lua/.luau files; an empty
  // directory simply yields no work. Faithful, and must not crash.
  let dir = tempfile::tempdir().unwrap();
  let as_file = dir.path().join("adir.luau");
  create_dir(&as_file).unwrap();
  bin("ulua")
    .arg(&as_file)
    .assert()
    .success()
    .stderr(predicate::str::contains("panicked").not());
}

#[test]
fn unicode_path_runs_cleanly() {
  let dir = tempfile::tempdir().unwrap();
  let path = dir.path().join("café_файл_🦀.luau");
  write(&path, "print('unicode-path-ok')\n").unwrap();
  bin("ulua")
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("unicode-path-ok"));
}

#[test]
fn compile_directory_as_file_no_panic() {
  let dir = tempfile::tempdir().unwrap();
  let as_file = dir.path().join("d.luau");
  create_dir(&as_file).unwrap();
  // Defined outcome (no .luau files inside) — must not panic.
  bin("ulua-compile")
    .arg(&as_file)
    .assert()
    .stderr(predicate::str::contains("panicked").not());
}
