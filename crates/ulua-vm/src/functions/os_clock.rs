use ulua_common::clock_shim::monotonic_seconds;

use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 读数/压栈全经 `push_number` 安全门面，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于
/// 可抛错的受保护帧，`push_number` 占用 top 之上 1 个空槽。cpp VM/src/loslib.cpp os_clock。
pub(crate) fn os_clock(l: &mut LuaState) -> i32 {
  l.push_number(monotonic_seconds());
  1
}

lua_lib_fn!(pub(crate) fn os_clock @ref, os_clock_arm);
