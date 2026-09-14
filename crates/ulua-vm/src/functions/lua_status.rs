use core::ffi::c_int;

use crate::type_aliases::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_status")]
pub unsafe fn lua_status(l: *mut lua_State) -> c_int {
  unsafe { (*l).status as c_int }
}
