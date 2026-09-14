use core::ffi::c_int;

use crate::type_aliases::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_singlestep")]
pub unsafe fn lua_singlestep(l: *mut lua_State, enabled: c_int) {
  unsafe {
    (*l).singlestep = enabled != 0;
  }
}
