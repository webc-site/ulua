use core::ffi::c_int;

use crate::{
  functions::{lua_pushnil::lua_pushnil, lua_pushthread::lua_pushthread},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn corunning(l: *mut lua_State) -> c_int {
  unsafe {
    if lua_pushthread(l) != 0 {
      lua_pushnil(l); // main thread is not a coroutine
    }
    1
  }
}
