use core::ffi::c_int;

use crate::{functions::lua_settop::lua_settop, records::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn lua_pop(l: *mut lua_State, n: c_int) {
  unsafe {
    lua_settop(l, -n - 1);
  }
}
