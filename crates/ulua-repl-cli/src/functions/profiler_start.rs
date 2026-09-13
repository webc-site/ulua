//! Faithful port of `void profilerStart(lua_State* l, int frequency)` (CLI/src/Profiler.cpp:100).

use core::sync::atomic::Ordering;
use std::thread;

use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::lua_State};

use crate::functions::{profiler_loop::profiler_loop, profiler_trigger::G_PROFILER};

pub fn profiler_start(l: *mut lua_State, frequency: i32) {
  unsafe {
    let profiler = core::ptr::addr_of_mut!(G_PROFILER).as_mut().unwrap();

    profiler.frequency = frequency;
    profiler.callbacks = lua_callbacks(l);

    profiler.exit.store(false, Ordering::Relaxed);
    profiler.thread = Some(thread::spawn(profiler_loop));
  }
}
