use ulua_vm::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_checkstack::lua_l_checkstack,
    lua_pcallyieldable::lua_pcallyieldable, lua_pushinteger::lua_pushinteger,
    lua_pushvalue::lua_pushvalue,
  },
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn pcall_then_x_call(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    lua_l_checkany(l, 2);

    lua_l_checkstack(l, 3, "pcallThenCall");
    lua_pushinteger(l, 0); // state
    lua_pushinteger(l, 0); // multiplier

    lua_pushvalue(l, 1); // call first function
    lua_pcallyieldable(l, 0, 1, 0)
  }
}
