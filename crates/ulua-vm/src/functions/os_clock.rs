use crate::{
  functions::{lua_clock::lua_clock, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn os_clock(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, lua_clock());
    1
  }
}
