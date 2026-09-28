//! Faithful port of `void profilerStart(LuaState* l, int frequency)` (CLI/src/Profiler.cpp:100).

use core::sync::atomic::Ordering;
use std::thread;

use ulua_ast::functions::optional_node::node_opt;
use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::LuaState};

use crate::functions::{profiler_loop::profiler_loop, profiler_trigger::G_PROFILER};

pub(crate) fn profiler_start(l: *mut LuaState, frequency: i32) {
  // Safety: 主线程串行路径，采样线程尚不存在（首次启动或上一次 profiler_stop
  // 已 join），落在 SharedProfiler 分区契约的启动窗口——此后才 spawn 采样线程。
  let p = unsafe { G_PROFILER.start() };
  *p.frequency = frequency;
  // Safety: l 为调用方（repl_main/setup_state）持有的有效 VM 状态，
  // lua_callbacks 对有效状态恒返回非空回调表指针。
  *p.callbacks = node_opt(unsafe { lua_callbacks(l) });
  p.exit.store(false, Ordering::Relaxed);
  *p.thread = Some(thread::spawn(profiler_loop));
}
