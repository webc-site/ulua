use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_sign")]
pub(crate) unsafe extern "C-unwind" fn math_sign(l: *mut lua_State) -> i32 {
  unsafe {
    let v = lua_l_checknumber(l, 1);
    let res = if v > 0.0 {
      1.0
    } else if v < 0.0 {
      -1.0
    } else {
      0.0
    };

    lua_pushnumber(l, res);
    1
  }
}
