use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_sinh")]
pub(crate) unsafe extern "C-unwind" fn math_sinh(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, f64::sinh(lua_l_checknumber(l, 1)));
    1
  }
}
