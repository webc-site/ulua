//! Faithful port of `void profilerStop()` (CLI/src/Profiler.cpp:109).

use core::sync::atomic::Ordering;

use crate::functions::profiler_trigger::G_PROFILER;

pub(crate) fn profiler_stop() {
  // Safety: 主线程串行路径。本窗口采样线程可能仍在其末轮读取 callbacks 等
  // 只读字段，故只取 exit/thread 两枚的停止视图（见 StopFields 的类型化契约）；
  // join 建立与采样线程终止的 happens-before，返回后 profiler_dump 方可经
  // dump 视图读取非原子字段。
  let p = unsafe { G_PROFILER.stop() };
  p.exit.store(true, Ordering::Relaxed);
  if let Some(handle) = p.thread.take() {
    let _ = handle.join();
  }
}
