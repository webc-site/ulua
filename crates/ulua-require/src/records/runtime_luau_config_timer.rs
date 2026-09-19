use core::cell::Cell;
use std::time::{Duration, Instant};

/// cpp `Navigation.h` 的 `RuntimeLuauConfigTimer`。
///
/// 字段是 `Cell` 而非普通值：`start` 的调用方只持有 `*const Self`（它由
/// `lua_getthreaddata` 从 VM 线程数据里取出），且对象本身经 `&self` 路径借出，
/// 用 `&mut` 写它会破坏 noalias 假设。`Instant`/`Option<Duration>` 均为 `Copy`，
/// `Cell` 即够。
#[derive(Debug)]
pub struct RuntimeLuauConfigTimer {
  pub(crate) start_time: Cell<Instant>,
  pub(crate) timeout_duration: Cell<Option<Duration>>,
}
