use core::ffi::c_int;

use crate::{
  functions::{lua_pushnil::lua_pushnil, lua_pushthread::lua_pushthread},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn corunning(l: *mut lua_State) -> c_int {
  unsafe {
    if lua_pushthread(l) != 0 {
      lua_pushnil(l); // main thread is not a coroutine
    }
    1
  }
}
