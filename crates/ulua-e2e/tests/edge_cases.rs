//! Hostile / adversarial edge cases.
//!
//! Every case must end in a DEFINED outcome — `Ok`, a clean `Err`, a non-zero
//! exit, or a bounded timeout-kill — and NEVER a Rust panic / abort / segfault.
//! Behaviors were cross-checked against the upstream C++ `luau` reference where
//! semantics could be in doubt (deep recursion, float edge cases, unicode
//! identifiers) and match byte-for-byte.

use std::fs::write;
mod common;

use std::time::Duration;

use common::{bin, write_script};
use predicates::prelude::*;
use ulua::{compile, eval};

/// Stderr must never contain a leaked Rust panic banner: the VM/parser use
/// `panic_any` for `longjmp`-style control flow, but those caught unwinds are
/// silenced — an escaped "panicked at"/"Box<dyn Any>" means a real defect.
fn no_panic() -> impl Predicate<str> {
  predicate::str::contains("panicked")
    .not()
    .and(predicate::str::contains("Box<dyn Any>").not())
}

// ---------------------------------------------------------------------------
// Empty / whitespace / trivial source
// ---------------------------------------------------------------------------

#[test]
fn empty_source_compiles_and_runs() {
  compile("").expect("empty source compiles");
  eval("").expect("empty source runs");
}

#[test]
fn whitespace_only_source_is_fine() {
  compile("   \n\t  \n").expect("whitespace compiles");
  eval(" \n \t \n").expect("whitespace runs");
}

#[test]
fn comment_only_source_is_fine() {
  eval("-- just a comment\n--[[ block ]]\n").expect("comments run");
}

// ---------------------------------------------------------------------------
// Large generated source
// ---------------------------------------------------------------------------

#[test]
fn hundred_thousand_line_script_runs() {
  let mut src = String::with_capacity(2_000_000);
  for i in 0..100_000 {
    src.push_str(&format!("local v{i} = {i}\n"));
  }
  src.push_str("return v99999\n");
  let (_dir, path) = write_script("big.luau", &src);
  bin("ulua")
    .arg(&path)
    .timeout(Duration::from_secs(120))
    .assert()
    .success()
    .stderr(no_panic());
}

// ---------------------------------------------------------------------------
// Deep nesting — must error gracefully or succeed, never stack-overflow the
// process into an abort.
// ---------------------------------------------------------------------------

#[test]
fn moderate_nested_parens_succeed() {
  let src = format!("return {}1{}\n", "(".repeat(200), ")".repeat(200));
  let (_dir, path) = write_script("nest.luau", &src);
  bin("ulua").arg(&path).assert().success().stderr(no_panic());
}

#[test]
fn moderate_nested_tables_succeed() {
  let src = format!("return {}1{}\n", "{".repeat(200), "}".repeat(200));
  let (_dir, path) = write_script("tab.luau", &src);
  bin("ulua").arg(&path).assert().success().stderr(no_panic());
}

#[test]
fn very_deep_nesting_errors_gracefully() {
  // Past the parser recursion limit Luau raises a clean "Exceeded allowed
  // recursion depth" error and exits non-zero — NOT a process crash and NOT a
  // leaked Rust panic. (Cross-checked against upstream `luau`.)
  let src = format!("return {}1{}\n", "(".repeat(5000), ")".repeat(5000));
  let (_dir, path) = write_script("deep.luau", &src);
  bin("ulua")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("recursion depth").and(no_panic()));
}

// ---------------------------------------------------------------------------
// Deep recursion in Luau (tail vs non-tail)
// ---------------------------------------------------------------------------

#[test]
fn deep_tail_recursion_is_defined() {
  // Luau does NOT do PUC-Lua-style tail-call elimination (verified against
  // upstream): deep recursion yields a "stack overflow" Lua error + exit 1,
  // which is a defined outcome (not a crash).
  let (_dir, path) = write_script(
    "tail.luau",
    "local function loop(n) if n == 0 then return 'done' end return loop(n-1) end\nprint(loop(1000000))\n",
  );
  bin("ulua")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("stack overflow").and(no_panic()));
}

#[test]
fn deep_non_tail_recursion_is_defined() {
  let (_dir, path) = write_script(
    "nontail.luau",
    "local function f(n) if n == 0 then return 0 end return 1 + f(n-1) end\nreturn f(1000000)\n",
  );
  bin("ulua")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("stack overflow").and(no_panic()));
}

#[test]
fn shallow_recursion_succeeds() {
  eval(
    "local function f(n) if n == 0 then return 0 end return 1 + f(n-1) end assert(f(100) == 100)",
  )
  .expect("shallow recursion is fine");
}

// ---------------------------------------------------------------------------
// Float / integer edge cases (cross-checked against upstream luau)
// ---------------------------------------------------------------------------

#[test]
fn float_edge_cases_match_lua_semantics() {
  let (_dir, path) = write_script(
    "floats.luau",
    "print(1/0)\nprint(-1/0)\nprint(0/0)\nprint(math.huge)\nprint(-0.0)\nprint(2^53)\n",
  );
  bin("ulua")
    .arg(&path)
    .assert()
    .success()
    .stdout(
      predicate::str::contains("inf")
        .and(predicate::str::contains("-inf"))
        .and(predicate::str::contains("nan"))
        .and(predicate::str::contains("9007199254740992")),
    )
    .stderr(no_panic());
}

#[test]
fn huge_integer_arithmetic_is_defined() {
  eval("local x = 9223372036854775807; assert(x + 0.0 == x + 0.0)")
    .expect("huge numbers don't crash");
  eval("assert(math.huge > 1e308)").expect("math.huge comparison ok");
}

// ---------------------------------------------------------------------------
// Unicode in strings and identifiers
// ---------------------------------------------------------------------------

#[test]
fn unicode_string_literals_work() {
  eval("local s = 'héllo 世界 🦀'; assert(#s > 0)").expect("unicode string literal ok");
}

#[test]
fn non_ascii_identifier_errors_like_upstream() {
  // Luau forbids non-ASCII identifiers; the parser reports a clean error
  // (matches upstream `luau`). The library `compile` surfaces it as `Err`.
  let err = compile("local é = 1\nreturn é").expect_err("non-ascii ident should be Err");
  assert!(
    err.contains("Unicode character"),
    "expected unicode-ident error: {err}"
  );
}

// ---------------------------------------------------------------------------
// Invalid UTF-8 bytes in source (CLI path reads raw bytes)
// ---------------------------------------------------------------------------

#[test]
fn invalid_utf8_in_source_is_defined() {
  // The CLI reads source as raw bytes (`read_file` uses from_utf8_unchecked),
  // so invalid UTF-8 inside a string literal is a defined outcome, not a crash.
  let dir = tempfile::tempdir().unwrap();
  let path = dir.path().join("badutf.luau");
  write(&path, b"print(\"\xff\xfe ok\")\n").unwrap();
  bin("ulua").arg(&path).assert().stderr(no_panic());
}

// ---------------------------------------------------------------------------
// Syntax errors
// ---------------------------------------------------------------------------

#[test]
fn syntax_error_at_eof_is_clean() {
  let (_dir, path) = write_script("eof.luau", "local x = (1 + \n");
  bin("ulua")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("<eof>").and(no_panic()));
}

#[test]
fn library_syntax_error_is_err_not_panic() {
  let err = compile("if then end").expect_err("malformed if should be Err");
  assert!(!err.is_empty());
}

// ---------------------------------------------------------------------------
// Runtime errors mid-execution
// ---------------------------------------------------------------------------

#[test]
fn runtime_error_mid_execution_is_clean() {
  let (_dir, path) = write_script(
    "mid.luau",
    "print('before')\nlocal t = nil\nprint(t.field)\nprint('after')\n",
  );
  bin("ulua")
    .arg(&path)
    .assert()
    .failure()
    .stdout(predicate::str::contains("before"))
    .stderr(predicate::str::contains("attempt to index nil").and(no_panic()));
}

#[test]
fn explicit_error_call_does_not_panic() {
  let (_dir, path) = write_script("err.luau", "error('boom-edge')\n");
  bin("ulua")
    .arg(&path)
    .assert()
    .failure()
    .stderr(predicate::str::contains("boom-edge").and(no_panic()));
}

// ---------------------------------------------------------------------------
// Large string allocation
// ---------------------------------------------------------------------------

#[test]
fn string_rep_large_is_defined() {
  let (_dir, path) = write_script(
    "rep.luau",
    "local s = string.rep('ab', 500000)\nprint(#s)\n",
  );
  bin("ulua")
    .arg(&path)
    .timeout(Duration::from_secs(60))
    .assert()
    .success()
    .stdout(predicate::str::contains("1000000"))
    .stderr(no_panic());
}

// ---------------------------------------------------------------------------
// Infinite loop guarded by a short timeout — must not hang CI.
// ---------------------------------------------------------------------------

#[test]
fn infinite_loop_is_killed_by_timeout_not_hung() {
  let (_dir, path) = write_script("inf.luau", "while true do end\n");
  // assert_cmd kills the child after the timeout; the result is a non-success
  // termination. The point is that the test returns in bounded time and the
  // process never hung CI or aborted with a Rust panic.
  let result = bin("ulua")
    .arg(&path)
    .timeout(Duration::from_secs(3))
    .assert();
  // A timeout-killed process does not exit successfully.
  let output = result.get_output();
  assert!(
    !output.status.success(),
    "infinite loop should have been killed, not exited successfully"
  );
  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    !stderr.contains("panicked"),
    "unexpected panic in stderr: {stderr}"
  );
}
