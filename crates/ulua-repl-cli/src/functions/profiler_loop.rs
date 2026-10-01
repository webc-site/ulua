//! Faithful port of `static void profilerLoop()` (CLI/src/Profiler.cpp:75).

use core::{ffi::c_int, sync::atomic::Ordering};
use std::thread;

use ulua_common::clock_shim::monotonic_seconds;
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::profiler_trigger::{G_PROFILER_SHARED, profiler_trigger},
  records::profiler::SamplerCallbacks,
};

/// The VM safepoint interrupt callback. `extern "C-unwind"` to match
/// `LuaCallbacks::interrupt`; mirrors C++ assigning
/// `gProfiler.callbacks->interrupt = profilerTrigger`.
// DELIBERATE DEVIATION（review.md §9.3）：写入 VM `LuaCallbacks::interrupt` 槽并
// 在 safepoint 处按 C ABI 被回调，`extern "C-unwind"` + 裸 `*mut LuaState`/`c_int`
// 系槽位 ABI 契约；实际采样逻辑收在 profiler_trigger（VM 线程边界）。
// Safety: 由 VM 在 safepoint 上调用（与 cpp 直接把 profilerTrigger 装入
// interrupt 槽一致），`l` 为触发中断的存活状态。
unsafe extern "C-unwind" fn profiler_interrupt(l: *mut LuaState, gc: c_int) {
  // 前置条件（l 存活、VM 线程）即 safepoint 回调约定，透传给 profiler_trigger。
  profiler_trigger(l, gc);
}

/// 采样线程本体。控制量经 [`profiler_start`](super::profiler_start) 的
/// `thread::spawn` 按值搬入（spawn 建立 happens-before，本线程栈上独占读，
/// 免共享可变字段——cpp 读文件静态量的等价形态）；跨线程共享只剩
/// [`G_PROFILER_SHARED`] 的原子发布面。
pub(crate) fn profiler_loop(frequency: i32, callbacks: Option<SamplerCallbacks>) {
  let frequency = frequency as f64;

  let mut last = monotonic_seconds();

  while !G_PROFILER_SHARED.exit.load(Ordering::Relaxed) {
    let now = monotonic_seconds();

    if now - last >= 1.0 / frequency {
      let ticks = ((now - last) * 1e6) as i64;

      G_PROFILER_SHARED
        .ticks
        .fetch_add(ticks as u64, Ordering::Relaxed);
      G_PROFILER_SHARED.samples.fetch_add(1, Ordering::Relaxed);
      // 判空即 cpp nullptr 检查；interrupt 槽写入沿用 VM safepoint 契约
      // （safepoint 窗口外无人读写该槽，本线程独占写入）。
      if let Some(mut target) = callbacks {
        // Safety: 指针对象为 profiler_start 接线时的状态固定区回调表，存活期由
        // start/spawn 与 stop/join 的先后次序覆盖采样窗口；本步只写 safepoint
        // 契约下独占的 interrupt 单槽（与 cpp `callbacks->interrupt =
        // profilerTrigger` 同形）。
        unsafe { target.0.as_mut().interrupt = Some(profiler_interrupt) };
      }

      last += ticks as f64 * 1e-6;
    } else {
      thread::yield_now();
    }
  }
}
