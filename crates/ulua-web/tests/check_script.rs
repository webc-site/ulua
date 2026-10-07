//! `check_script`（cpp `CLI/src/Web.cpp:142-182` 的 `checkScript`）的集成测试。
//!
//! 断言锁的是具体文本而非 `is_ok()`/前缀：诊断装配（`line + 1` + `": "` +
//! `toString(err)`，多条以 `\n` 连接，见 `Web.cpp:167-174`）、求解器选择
//! （`Web.cpp:154`）、零诊断回 `None`（`Web.cpp:181` 的 nullptr）。

use ulua_web::functions::check_script::check_script;

/// 旧求解器（cpp 的 `useNewSolver == 0`）。
const OLD_SOLVER: bool = false;
/// 新求解器（cpp 的 `useNewSolver != 0`）。
const NEW_SOLVER: bool = true;

/// 调入口并把零诊断折成空串，便于逐字节比对多条诊断的拼接结果。
fn check(source: &str, use_new_solver: bool) -> String {
  check_script(source, use_new_solver).unwrap_or_default()
}

/// 干净代码与空源码：零诊断 → `None`。
#[test]
fn clean_source_returns_none() {
  assert_eq!(check_script("local x = 1", OLD_SOLVER), None);
  assert_eq!(check_script("", OLD_SOLVER), None);
  assert_eq!(check_script("local x = 1", NEW_SOLVER), None);
}

/// 类型错误：`行: 信息` 全文锁定，行号自 1 起（`Web.cpp:171`）。
///
/// 正文出处为 Error.cpp:640-642 `TypePackMismatch` 的单行形态「Expected this to
/// be '<wanted>', but got '<given>'」（同文案见 TypeInfer.builtins.test.cpp:167）；
/// 原 `starts_with("1: ")` 只盯行号，挡不住正文换皮。
#[test]
fn type_error_locks_line_and_message() {
  assert_eq!(
    check("local x: number = 's'", OLD_SOLVER),
    "1: Expected this to be 'number', but got 'string'"
  );
}

/// 行号取自出错行、多条诊断按顺序以 `\n` 连接（`Web.cpp:169-171`）。
#[test]
fn diagnostics_lock_line_numbers_and_join_order() {
  let got = check(
    "local v = 1\nlocal ok: string = v == 1\nlocal bad: number = 'no'",
    OLD_SOLVER,
  );
  assert_eq!(
    got,
    "2: Expected this to be 'string', but got 'boolean'\n\
     3: Expected this to be 'number', but got 'string'"
  );
  assert_eq!(got.lines().count(), 2, "got: {got}");
}

/// 新求解器在同一段 strict 源码上报出更多诊断：锁全文，同时锁「多条以 `\n` 连接」
/// 与「首条前不补换行」（`Web.cpp:169-170` 的 `!empty()` 条件）。
#[test]
fn new_solver_locks_multi_diagnostic_payload() {
  let got = check(
    "--!strict\nlocal function f(a: number, b: string) return a + b end",
    NEW_SOLVER,
  );
  let overload = "Operator '+' could not be applied to operands of types number and string; \
there is no corresponding overload for __add";
  assert_eq!(
    got,
    format!("2: {overload}\n2: {overload}\n2: Consider annotating the return with number")
  );
  assert_eq!(got.lines().count(), 3, "got: {got}");
}

/// 求解器开关确实改变诊断集合（`Web.cpp:154` 的 `useNewSolver ? New : Old`）：
/// 同一份源码两求解器结果必须不同，上面的逐求解器锁定才不是空断言。
#[test]
fn solver_flag_selects_the_solver() {
  let source = "--!strict\nlocal function f(a: number, b: string) return a + b end";
  assert_ne!(check(source, OLD_SOLVER), check(source, NEW_SOLVER));
}

/// 无跨调用状态：上一次诊断不得影响下一次调用（cpp 靠
/// `Web.cpp:145` 的 `finalCheckResult.clear()` 维持，Rust 形态直接返回新串）。
#[test]
fn stateless_between_calls() {
  assert!(check_script("local x: number = 's'", OLD_SOLVER).is_some());
  assert_eq!(check_script("local x = 1", OLD_SOLVER), None);
}
