use core::ffi::c_void;

use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState`；`data` 允许为 NULL，仅原样存入 `(*l).userdata`——本端不追踪其生命周期，
/// 调用方须保证该指针在被读取前有效（或为 NULL）。cpp `lapi.cpp:1393`。
pub unsafe fn lua_setthreaddata(l: *mut LuaState, data: *mut c_void) {
  unsafe {
    (*l).userdata = data;
  }
}
