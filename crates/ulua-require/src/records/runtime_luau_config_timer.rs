use core::cell::Cell;

use coarsetime::{Duration, Instant};

/// cpp `Navigation.h` 的 `RuntimeLuauConfigTimer`。
///
/// 字段是 `Cell` 而非普通值：`start` 的调用方只持有 `*const Self`（它由
/// `lua_getthreaddata` 从 VM 线程数据里取出），且对象本身经 `&self` 路径借出，
/// 用 `&mut` 写它会破坏 noalias 假设。`Instant`/`Option<Duration>` 均为 `Copy`，
/// `Cell` 即够。
pub(crate) struct RuntimeLuauConfigTimer {
  pub(crate) start_time: Cell<Instant>,
  pub(crate) timeout_duration: Cell<Option<Duration>>,
}

impl RuntimeLuauConfigTimer {
  pub(crate) fn start(&self, timeout_ms: i32) {
    self.start_time.set(Instant::now());
    self
      .timeout_duration
      .set((timeout_ms >= 0).then(|| Duration::from_millis(timeout_ms as u64)));
  }

  pub(crate) fn is_finished(&self) -> bool {
    self
      .timeout_duration
      .get()
      .is_some_and(|timeout| self.start_time.get().elapsed() >= timeout)
  }
}
