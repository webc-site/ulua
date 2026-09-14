use core::ffi::c_int;

use crate::records::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_stackdepth")]
pub unsafe fn lua_stackdepth(l: *mut lua_State) -> c_int {
  unsafe { (*l).ci.offset_from((*l).base_ci) as c_int }
}
