//! Feature flag 端到端用例。
//!
//! (a) Luau FastFlags：既经 CLI 的 `--fflags=` 选项，也经 `ulua-common` 的库级
//!     旗标 API（`FFlag::<Name>.get()/.set()`），确认旗标可观测地翻转、
//!     `--fflags=` 被无错接受。
//! (b) Cargo 特性矩阵由 `scripts/check-features.sh` 校验（单独运行，
//!     不是递归 cargo 的 #[test]）。

use parking_lot::{Mutex, MutexGuard};
#[macro_use]
mod common;

use common::{bin, write_script};
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// (a) FastFlags —— 库级 API
// ---------------------------------------------------------------------------

/// `FValue` 是进程级全局：`cargo test` 在同一个进程里以多线程跑本文件的用例，
/// 下面的用例会互相覆写 `LuauCompileConcatTargetTop`。用一把锁把改全局旗标的用例
/// 串行化（nextest 每用例独立进程时该锁无竞争）。
///
/// 已知边界：该锁只覆盖本文件。同进程 `cargo test` 下若其他测试文件并发执行
/// `compile`/`eval`（读同一批旗标），此处 set 出的非默认值可能被其读到——
/// 本文件设计上以 nextest（test.sh 的执行器，每用例独立进程）为运行前提。
///
/// 锁型选 `parking_lot::Mutex`：无 std 的中毒语义，持锁用例 panic 后守卫仍可用
/// `lock()` 直接取回，不必 `into_inner()` 兜底。
static FLAG_TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock_flags() -> MutexGuard<'static, ()> {
  FLAG_TEST_LOCK.lock()
}

#[test]
fn library_fflag_toggle_is_observable() {
  use ulua_common::fflag;
  let _serial = lock_flags();
  // 旗标 API 往返成立：set 之后 get 立即可见。
  //（FValue<bool> 是进程级全局，与 Luau 的 FFlag 存储同形。）
  let original = fflag::LuauCompileConcatTargetTop.get();

  fflag::LuauCompileConcatTargetTop.set(false);
  assert!(
    !fflag::LuauCompileConcatTargetTop.get(),
    "flag should read back false"
  );

  fflag::LuauCompileConcatTargetTop.set(true);
  assert!(
    fflag::LuauCompileConcatTargetTop.get(),
    "flag should read back true"
  );

  // 复原，避免扰动同进程共享该旗标的其他用例。
  fflag::LuauCompileConcatTargetTop.set(original);
}

#[test]
fn set_luau_bool_flags_round_trips() {
  use ulua_common::fflag;
  let _serial = lock_flags();
  let original = fflag::LuauCompileConcatTargetTop.get();
  // CLI 的 setLuauFlagsDefault() 对等物必须不 panic，且至少在一个代表性旗标上可观测。
  ulua_common::set_luau_bool_flags(true);
  assert!(
    fflag::LuauCompileConcatTargetTop.get(),
    "set_luau_bool_flags(true) should enable Luau flags"
  );
  fflag::LuauCompileConcatTargetTop.set(original);
}

// ---------------------------------------------------------------------------
// (a) FastFlags —— CLI 接受度
// ---------------------------------------------------------------------------

#[test]
fn cli_global_fflags_true_is_accepted() {
  let (_dir, path) = write_script("p.luau", "return 1 + 2\n");
  // 文档承诺的全局形式 `--fflags=true` 打开全部旗标，必须干净接受（退出码 0），
  // 且照常产出反汇编。
  cli_case!(bin("ulua-compile").arg("--fflags=true").arg(&path) => success,
    stdout(predicate::str::contains("RETURN")));
}

#[test]
fn cli_global_fflags_false_is_accepted() {
  let (_dir, path) = write_script("p.luau", "print('ff-false-ok')\n");
  cli_case!(bin("ulua").arg("--fflags=false").arg(&path) => success,
    stdout(predicate::str::contains("ff-false-ok")));
}

#[test]
fn cli_named_fflag_does_not_fail() {
  let (_dir, path) = write_script("p.luau", "return 1\n");
  // 具名旗标即使 setter 警告名字不认识，也必须以 0 退出码被接受
  //（忠实上游：setLuauFlags 遇未知旗标不中止）。
  cli_case!(bin("ulua-compile").arg("--fflags=LuauCompileConcatTargetTop=true").arg(&path) => success,
    stdout(predicate::str::contains("RETURN")));
}

#[test]
fn analyze_accepts_fflags_option() {
  let (_dir, path) = write_script("good.luau", "--!strict\nlocal x: number = 1\nreturn x\n");
  // 干净 strict 文件 + 旗标全开：干净接受且两个流都不出诊断
  //（cpp reportModuleResult 无错误时不写任何输出）
  cli_case!(bin("ulua-analyze").arg("--fflags=true").arg(&path) => success,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::is_empty()));
}
