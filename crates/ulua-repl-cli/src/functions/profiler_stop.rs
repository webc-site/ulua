//! Faithful port of `void profilerStop()` (CLI/src/Profiler.cpp:109).

use core::sync::atomic::Ordering;

use crate::functions::profiler_trigger::G_PROFILER;

pub(crate) fn profiler_stop() {
  // Safety: `exit` 为原子存写；`thread` 仅主线程读写（与 profiler_start 同线程、
  // 串行）。join 建立与采样线程终止的 happens-before，返回后采样线程不再访问任何
  // 字段，非原子字段可被后续 profiler_dump 安全读取。
  unsafe {
    let profiler = G_PROFILER.get();
    (*profiler).exit.store(true, Ordering::Relaxed);
    if let Some(handle) = (*profiler).thread.take() {
      let _ = handle.join();
    }
  }
}
