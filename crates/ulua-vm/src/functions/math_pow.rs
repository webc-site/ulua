use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_pow")]
pub(crate) unsafe extern "C-unwind" fn math_pow(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, lua_l_checknumber(l, 1).powf(lua_l_checknumber(l, 2)));
    1
  }
}
