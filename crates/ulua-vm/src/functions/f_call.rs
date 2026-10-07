use core::ffi::c_void;

use crate::{
  functions::lua_d_call::lua_d_call,
  records::{call_s::CallS, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState` 且正处于 `lua_d_pcall`/`rawrunprotected` 保护帧内（本函数是其 Pfunc 回调）：
/// `ud` 须为由调用方栈上 `CallS` 转来的有效指针（`ud as *mut CallS` 后读其 `func`(须为栈内合法 StkId)
/// 与 `nresults`）；`lua_d_call` 会切帧调用并可抛错/触发 GC。
/// cpp VM/src/lapi.cpp:1170
pub(crate) unsafe extern "C-unwind" fn f_call(l: *mut LuaState, ud: *mut c_void) {
  unsafe {
    let c = ud as *mut CallS;
    lua_d_call(l, (*c).func, (*c).nresults);
  }
}
