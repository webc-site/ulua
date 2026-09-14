use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_atan2")]
pub(crate) unsafe extern "C-unwind" fn math_atan2(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(
      l,
      f64::atan2(lua_l_checknumber(l, 1), lua_l_checknumber(l, 2)),
    );
    1
  }
}
