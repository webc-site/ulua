use core::ptr::null_mut;

use crate::{
  macros::lua_callinfo_handle::LUA_CALLINFO_HANDLE,
  records::{call_info::CallInfo, lua_state::LuaState},
};

/// 自栈顶向下查找首个置 `LUA_CALLINFO_HANDLE` 的 handler 帧（cpp `resume_findhandler`）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为存活协程状态；返回的非空指针是其 ci 数组内存活的 handler 帧，即满足
/// `resume_handle` 细粒度恢复入参契约的帧指针，无 handler 时返回空。
pub(crate) unsafe fn resume_findhandler(l: *mut LuaState) -> *mut CallInfo {
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
