//! Faithful port of `void profilerStart(LuaState* l, int frequency)` (CLI/src/Profiler.cpp:100).

use core::sync::atomic::Ordering;
use std::thread;

use ulua_ast::functions::optional_node::node_opt;
use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::LuaState};

use crate::functions::{profiler_loop::profiler_loop, profiler_trigger::G_PROFILER};

pub(crate) fn profiler_start(l: *mut LuaState, frequency: i32) {
  // Safety: 主线程串行路径：调用时采样线程尚未启动（或已在 profiler_stop 中
  // join），G_PROFILER 无并发访问。与 profiler_trigger 同理逐字段裸指针写入，
  // 采样线程启动后不得再有覆盖整个 `Profiler` 的引用存活。
  unsafe {
    let profiler = G_PROFILER.get();
    (*profiler).frequency = frequency;
    (*profiler).callbacks = node_opt(lua_callbacks(l));
    (*profiler).exit.store(false, Ordering::Relaxed);
    (*profiler).thread = Some(thread::spawn(profiler_loop));
  }
}
