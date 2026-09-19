use core::ptr::null_mut;

use crate::{
  macros::lua_callinfo_handle::LUA_CALLINFO_HANDLE,
  records::{call_info::CallInfo, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn resume_findhandler(l: *mut lua_State) -> *mut CallInfo {
  unsafe {
    let mut ci = (*l).ci;

    while ci > (*l).base_ci {
      if ((*ci).flags & LUA_CALLINFO_HANDLE as u32) != 0 {
        return ci;
      }

      ci = ci.offset(-1);
    }

    null_mut()
  }
}
