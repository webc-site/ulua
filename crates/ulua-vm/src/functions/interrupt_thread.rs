use core::ffi::{c_int, c_void};

use crate::{
  functions::{lua_break::lua_break, luau_callhook::luau_callhook},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn interrupt_thread(l: *mut lua_State, co: *mut lua_State) -> c_int {
  unsafe {
    let global = (*l).global;
    let debuginterrupt = (*global).cb.debuginterrupt;
    if debuginterrupt.is_some() {
      luau_callhook(l, debuginterrupt, co as *mut c_void);
    }

    lua_break(l)
  }
}
