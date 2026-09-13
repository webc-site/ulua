use ulua_vm::{
  functions::{
    lua_l_callyieldable::lua_l_callyieldable, lua_l_checkstack::lua_l_checkstack,
    lua_pushvalue::lua_pushvalue,
  },
  type_aliases::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checkstack(l, 3, "cpass");
    lua_pushvalue(l, 1);
    lua_pushvalue(l, 2);
    lua_pushvalue(l, 3);
    lua_l_callyieldable(l, 2, 1)
  }
}
