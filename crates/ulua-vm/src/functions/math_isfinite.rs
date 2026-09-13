use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_isfinite")]
pub(crate) unsafe extern "C-unwind" fn math_isfinite(l: *mut lua_State) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    lua_pushboolean(l, x.is_finite() as i32);
    1
  }
}
