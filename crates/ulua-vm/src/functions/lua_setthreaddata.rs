use core::ffi::c_void;

use crate::records::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_setthreaddata")]
pub unsafe fn lua_setthreaddata(l: *mut lua_State, data: *mut c_void) {
  unsafe {
    (*l).userdata = data;
  }
}
