use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    auxresumecont::auxresumecont, coresumefinish::coresumefinish,
    interrupt_thread::interrupt_thread, lua_tothread::lua_tothread,
  },
  macros::lua_l_argexpected::luaL_argexpected,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn coresumecont(l: *mut lua_State, _status: c_int) -> c_int {
  unsafe {
    let co = lua_tothread(l, 1);
    luaL_argexpected!(l, !co.is_null(), 1, "thread");

    // if coroutine still hasn't yielded after the break, break current thread again
    if (*co).status == LuaStatus::Break as u8 {
      return interrupt_thread(l, co);
    }

    let r = auxresumecont(l, co);
    coresumefinish(l, r)
  }
}
