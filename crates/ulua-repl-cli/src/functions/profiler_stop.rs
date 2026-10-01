//! Faithful port of `void profilerStop()` (CLI/src/Profiler.cpp:109).

use core::sync::atomic::Ordering;

use crate::functions::profiler_trigger::{G_PROFILER_MAIN, G_PROFILER_SHARED};

pub(crate) fn profiler_stop() {
  // exit 为原子发布面：置位后采样线程最迟下一轮判退出（与 cpp 同语义）。
  G_PROFILER_SHARED.exit.store(true, Ordering::Relaxed);

  // 先取出句柄再 join：句柄是主线程 thread_local 字段，borrow 在 join 前释放。
  // join 建立与采样线程终止的 happens-before，返回后 profiler_dump 方可在同线程
  // 读取其独占字段。
  let handle = G_PROFILER_MAIN.with(|cell| cell.borrow_mut().thread.take());
  if let Some(handle) = handle {
    let _ = handle.join();
  }
}
