use ulua_vm::{
  functions::{lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// `l` must be a valid, non-null pointer to an initialized `lua_State`.
pub unsafe fn setup_state(l: *mut lua_State) {
  unsafe {
    lua_l_openlibs(l);
    lua_l_sandbox(l);
  }
}
