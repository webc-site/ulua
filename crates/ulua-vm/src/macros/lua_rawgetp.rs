use core::ffi::{c_int, c_void};

use crate::{functions::lua_rawgetptagged::lua_rawgetptagged, records::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawgetp(l: *mut lua_State, idx: c_int, p: *mut c_void) -> c_int {
  unsafe { lua_rawgetptagged(l, idx, p, 0) }
}
