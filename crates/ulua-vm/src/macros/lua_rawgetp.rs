use core::ffi::c_void;

use crate::{functions::lua_rawgetptagged::lua_rawgetptagged, records::lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawgetp(l: *mut LuaState, idx: i32, p: *mut c_void) -> i32 {
  unsafe { lua_rawgetptagged(l, idx, p, 0) }
}
