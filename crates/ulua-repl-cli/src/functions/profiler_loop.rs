//! Faithful port of `static void profilerLoop()` (CLI/src/Profiler.cpp:75).

use core::{ffi::c_int, sync::atomic::Ordering};
use std::thread;

use ulua_common::clock_shim::monotonic_seconds;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::profiler_trigger::{G_PROFILER, profiler_trigger};

/// The VM safepoint interrupt callback. `extern "C-unwind"` to match
/// `LuaCallbacks::interrupt`; mirrors C++ assigning
/// `gProfiler.callbacks->interrupt = profilerTrigger`.
// Safety: 由 VM 在 safepoint 上调用（与 cpp 直接把 profilerTrigger 装入
// interrupt 槽一致），`l` 为触发中断的存活状态。
unsafe extern "C-unwind" fn profiler_interrupt(l: *mut LuaState, gc: c_int) {
  // Safety: 见 profiler_trigger 的 # Safety：本函数在 VM 线程上被调用。
  unsafe {
    profiler_trigger(l, gc);
  }
}

pub(crate) fn profiler_loop() {
  // Safety: `frequency` 由 `profiler_start` 在本线程 spawn 前写入（spawn 建立
  // happens-before），循环期间只读。
  let p = G_PROFILER.get();
  let frequency = unsafe { (*p).frequency } as f64;

  let mut last = monotonic_seconds();

  // 退出标志是原子字段（Relaxed，与 cpp 语义一致），每轮判一次。
  // Safety: `exit` 仅经原子访问；`ticks`/`samples` 的 fetch_add 同理；
  // `callbacks` 为 start 后只读的 Option（判空即 cpp nullptr 检查），其指向的
  // interrupt 槽写入沿用 VM safepoint 契约——以上均不触碰 Profiler 的其余非原子字段。
  while !(unsafe { (*p).exit.load(Ordering::Relaxed) }) {
    let now = monotonic_seconds();

    if now - last >= 1.0 / frequency {
      let ticks = ((now - last) * 1e6) as i64;

      unsafe {
        (*p).ticks.fetch_add(ticks as u64, Ordering::Relaxed);
        (*p).samples.fetch_add(1, Ordering::Relaxed);
        if let Some(mut callbacks) = (*p).callbacks {
          callbacks.as_mut().interrupt = Some(profiler_interrupt);
        }
      }

      last += ticks as f64 * 1e-6;
    } else {
      thread::yield_now();
    }
  }
}
