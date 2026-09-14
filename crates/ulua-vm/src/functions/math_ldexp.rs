use crate::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checknumber::lua_l_checknumber,
    lua_pushnumber::lua_pushnumber,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_ldexp")]
pub(crate) unsafe extern "C-unwind" fn math_ldexp(l: *mut lua_State) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let exp = lua_l_checkinteger(l, 2);
    // ldexp(x, exp) is x * 2^exp
    lua_pushnumber(l, x * (2.0f64).powi(exp as i32));
    1
  }
}
