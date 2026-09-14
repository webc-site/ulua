use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  macros::radians_per_degree::RADIANS_PER_DEGREE,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_rad")]
pub(crate) unsafe extern "C-unwind" fn math_rad(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, lua_l_checknumber(l, 1) * RADIANS_PER_DEGREE);
    1
  }
}
