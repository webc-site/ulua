use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber, lua_l_optnumber::lua_l_optnumber,
    lua_pushnumber::lua_pushnumber,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_os_difftime")]
pub(crate) unsafe extern "C-unwind" fn os_difftime(l: *mut lua_State) -> i32 {
  unsafe {
    let t1 = lua_l_checknumber(l, 1);
    let t2 = lua_l_optnumber(l, 2, 0.0);

    // difftime in C returns the difference in seconds (t1 - t2) as a double.
    // Since we are targeting wasm32-unknown-unknown and portable environments,
    // and the input numbers are already doubles from the Lua stack, we can
    // perform the subtraction directly.
    let result = t1 - t2;

    lua_pushnumber(l, result);
    1
  }
}
