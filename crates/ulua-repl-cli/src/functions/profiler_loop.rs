//! Faithful port of `static void profilerLoop()` (CLI/src/Profiler.cpp:75).

use core::{ffi::c_int, sync::atomic::Ordering};
use std::thread;

use ulua_vm::{functions::lua_clock::lua_clock, records::lua_state::lua_State};

use crate::functions::profiler_trigger::{G_PROFILER, profiler_trigger};

/// The VM safepoint interrupt callback. `extern "C-unwind"` to match
/// `LuaCallbacks::interrupt`; mirrors C++ assigning
/// `gProfiler.callbacks->interrupt = profilerTrigger`.
unsafe extern "C-unwind" fn profiler_interrupt(l: *mut lua_State, gc: c_int) {
  unsafe {
    profiler_trigger(l, gc);
  }
}

pub fn profiler_loop() {
  unsafe {
    let p = core::ptr::addr_of_mut!(G_PROFILER);
    // `frequency` and `callbacks` are set by `profiler_start` before this
    // thread is spawned, so they are stable for the lifetime of the loop.
    let frequency = (*p).frequency as f64;
    let callbacks = (*p).callbacks;

    let mut last = lua_clock();

    while !(*p).exit.load(Ordering::Relaxed) {
      let now = lua_clock();

      if now - last >= 1.0 / frequency {
        let ticks = ((now - last) * 1e6) as i64;

        (*p).ticks.fetch_add(ticks as u64, Ordering::Relaxed);
        (*p).samples.fetch_add(1, Ordering::Relaxed);
        if !callbacks.is_null() {
          (*callbacks).interrupt = Some(profiler_interrupt);
        }

        last += ticks as f64 * 1e-6;
      } else {
        thread::yield_now();
      }
    }
  }
}
