use ulua_vm::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checkstack::lua_l_checkstack,
    lua_pushinteger::lua_pushinteger, lua_settop::lua_settop, lua_yield::lua_yield,
  },
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields(l: *mut lua_State) -> i32 {
  unsafe {
    lua_settop(l, 1);

    let base = lua_l_checkinteger(l, 1);

    lua_l_checkstack(l, 2, "cmultiyield");

    let pos: i32 = 1;

    lua_pushinteger(l, pos);

    lua_pushinteger(l, base + pos);

    lua_yield(l, 1)
  }
}
