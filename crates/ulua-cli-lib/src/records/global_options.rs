use core::sync::atomic::{AtomicI32, Ordering};

/// cpp 默认优化级别（`Luau::CompileOpts` 缺省，Repl.cpp/Bytecode.cpp 的
/// `globalOptions.optimizationLevel = 1`）
pub const DEFAULT_OPTIMIZATION_LEVEL: i32 = 1;
/// cpp 默认调试级别（`globalOptions.debugLevel = 1`）
pub const DEFAULT_DEBUG_LEVEL: i32 = 1;

/// 级别开关的单一读写形态：Relaxed 原子装载/落位的样板只此一份。
struct Level(AtomicI32);

impl Level {
  const fn new(default: i32) -> Self {
    Self(AtomicI32::new(default))
  }

  #[inline]
  fn get(&self) -> i32 {
    self.0.load(Ordering::Relaxed)
  }

  #[inline]
  fn set(&self, level: i32) {
    self.0.store(level, Ordering::Relaxed);
  }
}

/// 镜像 cpp 全局 `globalOptions`（单线程 CLI，原子全局免 unsafe）。
/// repl 与 bytecode 两个 CLI 共用同一对级别开关（cpp 中即同一组
/// `globalOptions` 静态），故收敛于此。
static OPTIMIZATION_LEVEL: Level = Level::new(DEFAULT_OPTIMIZATION_LEVEL);
static DEBUG_LEVEL: Level = Level::new(DEFAULT_DEBUG_LEVEL);

pub fn get_optimization_level() -> i32 {
  OPTIMIZATION_LEVEL.get()
}

pub fn set_optimization_level(level: i32) {
  OPTIMIZATION_LEVEL.set(level);
}

pub fn get_debug_level() -> i32 {
  DEBUG_LEVEL.get()
}

pub fn set_debug_level(level: i32) {
  DEBUG_LEVEL.set(level);
}

/// 恢复 cpp 缺省级别（每次 CLI 入口重置，防进程内多次调用串扰）。
pub fn reset_to_defaults() {
  OPTIMIZATION_LEVEL.set(DEFAULT_OPTIMIZATION_LEVEL);
  DEBUG_LEVEL.set(DEFAULT_DEBUG_LEVEL);
}
