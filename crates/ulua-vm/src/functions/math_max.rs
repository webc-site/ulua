use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_max")]
pub(crate) unsafe extern "C-unwind" fn math_max(l: *mut lua_State) -> i32 {
  unsafe {
    let n = lua_gettop(l); // number of arguments
    let mut dmax = lua_l_checknumber(l, 1);
    let mut i = 2;
    while i <= n {
      let d = lua_l_checknumber(l, i);
      if d > dmax {
        dmax = d;
      }
      i += 1;
    }
    lua_pushnumber(l, dmax);
    1
  }
}
