//! Faithful port of `void profilerStop()` (CLI/src/Profiler.cpp:109).

use core::sync::atomic::Ordering;

use crate::functions::profiler_trigger::G_PROFILER;

pub fn profiler_stop() {
  unsafe {
    let profiler = core::ptr::addr_of_mut!(G_PROFILER).as_mut().unwrap();
    profiler.exit.store(true, Ordering::Relaxed);
    if let Some(handle) = profiler.thread.take() {
      let _ = handle.join();
    }
  }
}
