use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_fmod")]
pub(crate) unsafe extern "C-unwind" fn math_fmod(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, fmod(lua_l_checknumber(l, 1), lua_l_checknumber(l, 2)));
    1
  }
}

#[inline]
fn fmod(a: f64, b: f64) -> f64 {
  a % b
}
