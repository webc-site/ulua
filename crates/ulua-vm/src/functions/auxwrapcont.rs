use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    auxresumecont::auxresumecont, auxwrapfinish::auxwrapfinish, interrupt_thread::interrupt_thread,
    lua_tothread::lua_tothread,
  },
  macros::lua_upvalueindex::lua_upvalueindex,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn auxwrapcont(l: *mut lua_State, _status: c_int) -> c_int {
  unsafe {
    let co = lua_tothread(l, lua_upvalueindex(1));

    if (*co).status == LuaStatus::Break as u8 {
      return interrupt_thread(l, co);
    }

    let r = auxresumecont(l, co);
    auxwrapfinish(l, r)
  }
}
