//! 进程内退出码测试：`run_main(&args)` 接收注入的参数，无需 spawn 子进程
//! （对齐 `ulua-reduce-cli/tests/reduce_roundtrip.rs` 的用法）。
//!
//! 早退分支（--help / 未知选项）不触碰文件系统；`--mode=nonstrict` 与
//! ICE→2 两个用例（对照 `CLI/src/Analyze.cpp:428` 与 `:538`）需要真实临时
//! 脚本。`run` 每次调用开头都会 `set_luau_flags_default()` 重置全局 FFlag，
//! 因此本文件所有用例共用一把进程级锁串行化，避免并行跑动互相覆盖 flag 与
//! mode 等进程级状态。

use std::{
  env::temp_dir,
  fs,
  process::id,
  sync::{Mutex, MutexGuard, OnceLock, PoisonError},
};

use ulua_analyze_cli::run_main;

/// 串行化所有会改动进程级状态（FFlag/mode）的 run_main 调用
fn run_lock() -> MutexGuard<'static, ()> {
  static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
  LOCK
    .get_or_init(|| Mutex::new(()))
    .lock()
    .unwrap_or_else(PoisonError::into_inner)
}

/// 组装 argv：首元素是程序名，其余由用例给出
fn argv(rest: &[&str]) -> Vec<String> {
  ["luau-analyze"]
    .iter()
    .chain(rest)
    .map(|arg| (*arg).to_string())
    .collect()
}

/// 用例独占的临时 .luau 脚本（绝对路径，Drop 时删除）
struct TempScript {
  path: String,
}

impl TempScript {
  fn new(name: &str, source: &str) -> Self {
    let path = temp_dir().join(format!("ulua-analyze-exit-{}-{name}.luau", id()));
    fs::write(&path, source).expect("write temp script");
    Self {
      path: path.to_string_lossy().into_owned(),
    }
  }
}

impl Drop for TempScript {
  fn drop(&mut self) {
    let _ = fs::remove_file(&self.path);
  }
}

#[test]
fn help_exits_zero() {
  let _guard = run_lock();
  assert_eq!(run_main(&argv(&["--help"])), 0);
}

#[test]
fn unrecognized_option_exits_one() {
  let _guard = run_lock();
  assert_eq!(run_main(&argv(&["--definitely-not-an-option"])), 1);
}

/// cpp Analyze.cpp:428：`--mode=nonstrict` 是合法选项。修复前它落入解析
/// else 分支 → `Unrecognized option` → 退出 1；现在须与 `--mode=strict`
/// 一样被完整接受（干净脚本退出 0）。
#[test]
fn mode_nonstrict_is_accepted() {
  let _guard = run_lock();
  let script = TempScript::new("nonstrict", "return 1\n");

  assert_eq!(
    run_main(&argv(&["--mode=nonstrict", &script.path])),
    0,
    "--mode=nonstrict 不应被当作未知选项"
  );
  // 对照组：strict 同样被接受，防止两条分支互相回归
  assert_eq!(run_main(&argv(&["--mode=strict", &script.path])), 0);
}

/// cpp Analyze.cpp:538：内部编译错误有独立退出码 2。触发路径与上游 golden
/// selftest 相同：`--fflags=DebugLuauMagicTypes` 下把 `_luau_ice` 写进类型
/// 标注，约束生成器在 resolveReferenceType（cpp ConstraintGenerator.cpp:4578）
/// 抛 InternalCompilerError，由 run 的 catch 分支报告后退出 2。
#[test]
fn internal_compiler_error_exits_two() {
  let _guard = run_lock();
  let script = TempScript::new("ice", "local value: _luau_ice\n");

  assert_eq!(
    run_main(&argv(&["--fflags=DebugLuauMagicTypes", &script.path])),
    2,
    "ICE 必须退出 2（cpp Analyze.cpp:538），不得塌缩为类型错误的 1"
  );
}
