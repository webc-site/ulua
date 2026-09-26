use core::ffi::c_void;

use crate::{
  functions::{lua_break::lua_break, luau_callhook::luau_callhook},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global.cb.debuginterrupt` 为其所属调试中断回调；`co` 须为存活 coroutine 线程，
/// 将作为 `c_void` 透传给 `luau_callhook`。回调须在 `l` 的受保护帧内执行、可抛错，随后 `lua_break` 挂起当前线程。
/// cpp/VM/src/lcorolib.cpp:27 interruptThread。
pub(crate) unsafe fn interrupt_thread(l: *mut LuaState, co: *mut LuaState) -> i32 {
  unsafe {
    let global = (*l).global;
    let debuginterrupt = (*global).cb.debuginterrupt;
    if debuginterrupt.is_some() {
      luau_callhook(l, debuginterrupt, Some(co as *mut c_void));
    }

    lua_break(l)
  }
}
