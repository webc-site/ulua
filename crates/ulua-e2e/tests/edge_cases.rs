//! Hostile / adversarial edge cases.
//!
//! Every case must end in a DEFINED outcome — `Ok`, a clean `Err`, a non-zero
//! exit, or a bounded timeout-kill — and NEVER a Rust panic / abort / segfault.
//! Behaviors were cross-checked against the upstream C++ `luau` reference where
//! semantics could be in doubt (deep recursion, float edge cases, unicode
//! identifiers) and match byte-for-byte.

use std::{fs::write, iter::once};
#[macro_use]
mod common;

use std::time::Duration;

use common::{bin, write_script};
use predicates::prelude::*;
use ulua::{Error, compile, eval};

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
  // 迭代器链单次生成 10 万行，避免逐行 push_str 的多次扩容
  let src: String = (0..10_000)
    .map(|i| format!("local v{i} = {i}\n"))
    .chain(once("return v9999\n".to_owned()))
    .collect();
  let (_dir, path) = write_script("big.luau", &src);
  cli_case!(bin("ulua").arg(&path).timeout(Duration::from_secs(120)) => success, stderr(no_panic()));
}

// ---------------------------------------------------------------------------
// Deep nesting — must error gracefully or succeed, never stack-overflow the
// process into an abort.
// ---------------------------------------------------------------------------

#[test]
fn moderate_nested_parens_succeed() {
  let src = format!("return {}1{}\n", "(".repeat(200), ")".repeat(200));
  let (_dir, path) = write_script("nest.luau", &src);
  cli_case!(bin("ulua").arg(&path) => success, stderr(no_panic()));
}

#[test]
fn moderate_nested_tables_succeed() {
  let src = format!("return {}1{}\n", "{".repeat(200), "}".repeat(200));
  let (_dir, path) = write_script("tab.luau", &src);
  cli_case!(bin("ulua").arg(&path) => success, stderr(no_panic()));
}

#[test]
fn very_deep_nesting_errors_gracefully() {
  // 过解析递归上限后是干净的非零退出（非进程崩溃、非泄漏 Rust panic），
  // 文案全等锁定 cpp Parser.cpp:5394 的
  // "Exceeded allowed recursion depth; simplify your %s to make the code compile"
  // （%s=expression）。原来的 contains("recursion depth") 挡不住换皮文案。
  let src = format!("return {}1{}\n", "(".repeat(5000), ")".repeat(5000));
  let (_dir, path) = write_script("deep.luau", &src);
  cli_case!(bin("ulua").arg(&path) => failure,
  stderr(
    predicate::str::contains(
      "Exceeded allowed recursion depth; simplify your expression to make the code compile"
    )
    .and(no_panic())
  ));
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
  cli_case!(bin("ulua").arg(&path) => failure,
    stderr(predicate::str::contains("stack overflow").and(no_panic())));
}

#[test]
fn deep_non_tail_recursion_is_defined() {
  let (_dir, path) = write_script(
    "nontail.luau",
    "local function f(n) if n == 0 then return 0 end return 1 + f(n-1) end\nreturn f(1000000)\n",
  );
  cli_case!(bin("ulua").arg(&path) => failure,
    stderr(predicate::str::contains("stack overflow").and(no_panic())));
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
  cli_case!(bin("ulua").arg(&path) => success,
    stdout(
      predicate::str::contains("inf")
        .and(predicate::str::contains("-inf"))
        .and(predicate::str::contains("nan"))
        .and(predicate::str::contains("9007199254740992")),
    ),
    stderr(no_panic()));
}

#[test]
fn huge_integer_arithmetic_is_defined() {
  // i64::MAX 转 f64 舍入到 2^63（实测 ulua 与 cpp 同为 true）；原来的
  // `x + 0.0 == x + 0.0` 是同表达式自比恒真，错把整数换算成任何双精度值都能过
  eval("local x = 9223372036854775807; assert(x + 0.0 == 2^63)")
    .expect("i64::MAX rounds to 2^63 when widened to double");
  eval("assert(math.huge > 1e308)").expect("math.huge comparison ok");
}

// ---------------------------------------------------------------------------
// Unicode in strings and identifiers
// ---------------------------------------------------------------------------

#[test]
fn unicode_string_literals_work() {
  // 锁字节语义：`#` 数 UTF-8 字节而非码点（'héllo 世界 🦀' = 1+2+1+1+1+1+3+3+1+4），
  // 错按码点数（11）或 UTF-16 单元数（13）实现都会在这里暴露
  eval("local s = 'héllo 世界 🦀'; assert(#s == 18)").expect("unicode string literal ok");
}

#[test]
fn non_ascii_identifier_errors_like_upstream() {
  // Luau forbids non-ASCII identifiers; the parser reports a clean error
  // (matches upstream `luau`). The library `compile` surfaces it as `Err`.
  let err = compile("local é = 1\nreturn é").expect_err("non-ascii ident should be Err");
  // 全等锁定 cpp Compiler.cpp:5821 blob 正文（":%d: %s"）+ tests/Parser.test.cpp:917
  // error_on_unicode 同格式正文；码点按 Lexer.cpp:168 "Unicode character U+%x"
  // 小写十六进制渲染（é=U+00E9→"e9"）。
  let Error::SyntaxError { message, .. } = err else {
    panic!("expected SyntaxError, got {err:?}");
  };
  assert_eq!(
    message,
    ":1: Expected identifier when parsing variable name, got Unicode character U+e9"
  );
}

// ---------------------------------------------------------------------------
// Invalid UTF-8 bytes in source (CLI path reads raw bytes)
// ---------------------------------------------------------------------------

#[test]
fn invalid_utf8_in_source_is_defined() {
  // `read_file` 读原始字节后按 UTF-8 解码，非 UTF-8 走 lossy 替换而非 UB，
  // 因此字符串字面量里的非法字节是确定结果而不是崩溃。
  let dir = tempfile::tempdir().unwrap();
  let path = dir.path().join("badutf.luau");
  write(&path, b"print(\"\xff\xfe ok\")\n").unwrap();
  cli_case!(bin("ulua").arg(&path) => success,
    stdout(predicate::str::contains("ok")),
    stderr(no_panic()));
}

// ---------------------------------------------------------------------------
// Syntax errors
// ---------------------------------------------------------------------------

#[test]
fn syntax_error_at_eof_is_clean() {
  // 锁 cpp 级全文而非只盯 "<eof>" 字样：cpp tests/PrettyPrinter.test.cpp:1729
  // 对同类 EOF 截断源 CHECK_EQ 该正文；本输入 `1 +` 后遇 <eof>，
  // simpleExpression→primaryExpression 以 identifier(got <eof>) 报错。
  let (_dir, path) = write_script("eof.luau", "local x = (1 + \n");
  cli_case!(bin("ulua").arg(&path) => failure,
  stderr(
    predicate::str::contains("Expected identifier when parsing expression, got <eof>")
      .and(no_panic())
  ));
}

#[test]
fn library_syntax_error_is_err_not_panic() {
  let err = compile("if then end").expect_err("malformed if should be Err");
  let Error::SyntaxError { message, .. } = err else {
    panic!("expected SyntaxError");
  };
  // 全等锁定 cpp Compiler.cpp:5821 错误 blob 正文（":%d: %s"）+ Parser.cpp:5428
  // （context=expression；'then' 为 Lexer 保留字 "'%s'" 形态）。
  // 原 `!message.is_empty()` 挡不住任意非空文案。
  assert_eq!(
    message,
    ":1: Expected identifier when parsing expression, got 'then'"
  );
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
  cli_case!(bin("ulua").arg(&path) => failure,
    stdout(predicate::str::contains("before")),
    stderr(predicate::str::contains("attempt to index nil").and(no_panic())));
}

#[test]
fn explicit_error_call_does_not_panic() {
  let (_dir, path) = write_script("err.luau", "error('boom-edge')\n");
  cli_case!(bin("ulua").arg(&path) => failure,
    stderr(predicate::str::contains("boom-edge").and(no_panic())));
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
  cli_case!(bin("ulua").arg(&path).timeout(Duration::from_secs(60)) => success,
    stdout(predicate::str::contains("1000000")),
    stderr(no_panic()));
}

// ---------------------------------------------------------------------------
// Infinite loop guarded by a short timeout — must not hang CI.
// ---------------------------------------------------------------------------

#[test]
fn infinite_loop_is_killed_by_timeout_not_hung() {
  let (_dir, path) = write_script("inf.luau", "while true do end\n");
  // assert_cmd 到点即杀子进程，结果是一次非成功终止。要点在于用例必有界返回，
  // 既不把 CI 挂死，也不会以 Rust panic / abort 收场。
  let result = bin("ulua")
    .arg(&path)
    .timeout(Duration::from_millis(300))
    .assert();
  // 被超时杀掉的进程不会以成功退出。
  let output = result.get_output();
  assert!(
    !output.status.success(),
    "infinite loop should have been killed, not exited successfully"
  );
  // 死循环检查复用 no_panic 判据，避免两处「panicked 字样」各写一份。
  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    no_panic().eval(&stderr),
    "unexpected panic in stderr: {stderr}"
  );
}
