use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_map")]
pub(crate) unsafe extern "C-unwind" fn math_map(l: *mut lua_State) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let inmin = lua_l_checknumber(l, 2);
    let inmax = lua_l_checknumber(l, 3);
    let outmin = lua_l_checknumber(l, 4);
    let outmax = lua_l_checknumber(l, 5);

    let result = outmin + (x - inmin) * (outmax - outmin) / (inmax - inmin);
    lua_pushnumber(l, result);
    1
  }
}
