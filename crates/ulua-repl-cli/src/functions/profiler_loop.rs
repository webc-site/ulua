//! Faithful port of `static void profilerLoop()` (CLI/src/Profiler.cpp:75).

use core::{ffi::c_int, sync::atomic::Ordering};
use std::thread;

use ulua_common::clock_shim::monotonic_seconds;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::profiler_trigger::{G_PROFILER, profiler_trigger};

/// The VM safepoint interrupt callback. `extern "C-unwind"` to match
/// `LuaCallbacks::interrupt`; mirrors C++ assigning
/// `gProfiler.callbacks->interrupt = profilerTrigger`.
// DELIBERATE DEVIATION（review.md §9.3）：写入 VM `LuaCallbacks::interrupt` 槽并
// 在 safepoint 处按 C ABI 被回调，`extern "C-unwind"` + 裸 `*mut LuaState`/`c_int`
// 系槽位 ABI 契约；实际采样逻辑收在 profiler_trigger（VM 线程边界）。
// Safety: 由 VM 在 safepoint 上调用（与 cpp 直接把 profilerTrigger 装入
// interrupt 槽一致），`l` 为触发中断的存活状态。
unsafe extern "C-unwind" fn profiler_interrupt(l: *mut LuaState, gc: c_int) {
  // Safety: 见 profiler_trigger 的 # Safety：本函数在 VM 线程上被调用。
  unsafe {
    profiler_trigger(l, gc);
  }
}

pub(crate) fn profiler_loop() {
  // Safety: 本 fn 即采样线程本体，落在 SharedProfiler 字段分区契约的采样窗口：
  // `frequency`/`callbacks` 由 profiler_start 于 spawn 前写入（spawn 建立
  // happens-before）、循环期间只读；`exit`/`ticks`/`samples` 仅经 Relaxed 原子
  // 访问；不触碰 VM 线程独占的非原子字段。
  let p = unsafe { G_PROFILER.sampler() };
  let frequency = *p.frequency as f64;

  let mut last = monotonic_seconds();

  while !p.exit.load(Ordering::Relaxed) {
    let now = monotonic_seconds();

    if now - last >= 1.0 / frequency {
      let ticks = ((now - last) * 1e6) as i64;

      p.ticks.fetch_add(ticks as u64, Ordering::Relaxed);
      p.samples.fetch_add(1, Ordering::Relaxed);
      // `Option<NonNull>` 是 Copy：判空即 cpp nullptr 检查；interrupt 槽写入
      // 沿用 VM safepoint 契约（与 cpp `callbacks->interrupt = profilerTrigger` 同形）。
      if let Some(mut callbacks) = *p.callbacks {
        // Safety: VM safepoint 契约——safepoint 窗口外无人读写该槽，本线程独占写入。
        unsafe { callbacks.as_mut().interrupt = Some(profiler_interrupt) };
      }

      last += ticks as f64 * 1e-6;
    } else {
      thread::yield_now();
    }
  }
}
