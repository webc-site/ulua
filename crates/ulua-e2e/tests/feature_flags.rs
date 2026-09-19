//! Feature-flag tests.
//!
//! (a) Luau FastFlags via the CLI `--fflags=` option and via the library flag
//!     API in `ulua-common` (`FFlag::<Name>.get()/.set()`), confirming a flag
//!     toggles observably and that `--fflags=` is accepted without error.
//! (b) The Cargo feature matrix is checked by `scripts/check-features.sh` (run
//!     separately, NOT as a recursive-cargo #[test]).

use std::sync::{Mutex, MutexGuard};
mod common;

use common::write_script;
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// (a) FastFlags — library API
// ---------------------------------------------------------------------------

/// `FValue` 是进程级全局：`cargo test` 在同一个进程里以多线程跑本文件的用例，
/// 下面的用例会互相覆写 `LuauCompileConcatTargetTop`。用一把锁把改全局旗标的用例
/// 串行化（nextest 每用例独立进程时该锁无竞争）。
static FLAG_TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock_flags() -> MutexGuard<'static, ()> {
  FLAG_TEST_LOCK
    .lock()
    .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn library_fflag_toggle_is_observable() {
  use ulua_common::fflag;
  let _serial = lock_flags();
  // The flag API round-trips: setting a value is immediately observable via
  // get(). (FValue<bool> is a process-global, matching Luau's FFlag storage.)
  let original = fflag::LuauCompileConcatTargetTop.get();

  unsafe { fflag::LuauCompileConcatTargetTop.set(false) };
  assert!(
    !fflag::LuauCompileConcatTargetTop.get(),
    "flag should read back false"
  );

  unsafe { fflag::LuauCompileConcatTargetTop.set(true) };
  assert!(
    fflag::LuauCompileConcatTargetTop.get(),
    "flag should read back true"
  );

  // Restore so we don't perturb other tests sharing this process.
  unsafe { fflag::LuauCompileConcatTargetTop.set(original) };
}

#[test]
fn set_luau_bool_flags_round_trips() {
  use ulua_common::fflag;
  let _serial = lock_flags();
  let original = fflag::LuauCompileConcatTargetTop.get();
  // The CLI's setLuauFlagsDefault() analog must run without panicking and be
  // observable on at least one representative flag.
  ulua_common::set_luau_bool_flags(true);
  assert!(
    fflag::LuauCompileConcatTargetTop.get(),
    "set_luau_bool_flags(true) should enable Luau flags"
  );
  unsafe { fflag::LuauCompileConcatTargetTop.set(original) };
}

// ---------------------------------------------------------------------------
// (a) FastFlags — CLI acceptance
// ---------------------------------------------------------------------------

#[test]
fn cli_global_fflags_true_is_accepted() {
  let (_dir, path) = write_script("p.luau", "return 1 + 2\n");
  // The documented global form `--fflags=true` enables all flags and must be
  // accepted cleanly (exit 0), still producing the disassembly.
  common::bin("ulua-compile")
    .arg("--fflags=true")
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("RETURN"));
}

#[test]
fn cli_global_fflags_false_is_accepted() {
  let (_dir, path) = write_script("p.luau", "print('ff-false-ok')\n");
  common::bin("ulua")
    .arg("--fflags=false")
    .arg(&path)
    .assert()
    .success()
    .stdout(predicate::str::contains("ff-false-ok"));
}

#[test]
fn cli_named_fflag_does_not_fail() {
  let (_dir, path) = write_script("p.luau", "return 1\n");
  // A named-flag toggle is accepted without a non-zero exit even when the
  // setter warns about an unrecognized name (faithful: setLuauFlags does not
  // abort on an unknown flag).
  common::bin("ulua-compile")
    .arg("--fflags=LuauCompileConcatTargetTop=true")
    .arg(&path)
    .assert()
    .success();
}

#[test]
fn analyze_accepts_fflags_option() {
  let (_dir, path) = write_script("good.luau", "--!strict\nlocal x: number = 1\nreturn x\n");
  common::bin("ulua-analyze")
    .arg("--fflags=true")
    .arg(&path)
    .assert()
    .success();
}
