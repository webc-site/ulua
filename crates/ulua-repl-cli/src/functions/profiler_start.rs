//! Faithful port of `void profilerStart(LuaState* l, int frequency)` (CLI/src/Profiler.cpp:100).

use core::sync::atomic::Ordering;
use std::thread;

use ulua_ast::functions::optional_node::node_opt;
use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::LuaState};

use crate::{
  functions::{
    profiler_loop::profiler_loop,
    profiler_trigger::{G_PROFILER_MAIN, G_PROFILER_SHARED},
  },
  records::profiler::SamplerCallbacks,
};

// DELIBERATE DEVIATION（review.md §9.3）：主线程串行窗口内接线采样控制量并写 VM
// `lua_callbacks` interrupt 句柄，随后 spawn 采样线程；`frequency` 与回调表句柄
// 按值搬进线程（spawn 建立 happens-before，采样线程栈上独占读，cpp「先写静态量
// 再 spawn」的等价形态），裸 `*mut LuaState`/`lua_callbacks` 调用系 ulua-vm c-API
// 边界固有。
pub(crate) fn profiler_start(l: *mut LuaState, frequency: i32) {
  // Safety: l 为调用方（repl_main/run_repl）持有的有效 VM 状态，lua_callbacks 对
  // 有效状态恒返回非空回调表指针；node_opt 判空转 Option，null 哨兵止步于本边界。
  let callbacks = node_opt(unsafe { lua_callbacks(l) }).map(SamplerCallbacks);

  G_PROFILER_SHARED.exit.store(false, Ordering::Relaxed);
  // Safety 面之外全为 thread_local 借用（主线程串行窗口，采样线程尚未存在）。
  G_PROFILER_MAIN.with(|cell| {
    let mut p = cell.borrow_mut();
    // 记录接线供 profiler_trigger 摘除 interrupt 用（判空即 cpp nullptr 检查）。
    p.callbacks = callbacks.map(|c| c.0);
    p.thread = Some(thread::spawn(move || profiler_loop(frequency, callbacks)));
  });
}
