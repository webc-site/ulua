use ulua_vm::{
  functions::{lua_l_getmetafield::lua_l_getmetafield, lua_remove::lua_remove},
  records::lua_state::lua_State,
};

/// # Safety
///
/// `l` must be a valid, active pointer to a `lua_State` with at least one element on its stack.
pub unsafe fn try_replace_top_with_index(l: *mut lua_State) -> bool {
  unsafe {
    if lua_l_getmetafield(l, -1, c"__index".as_ptr()) != 0 {
      // Remove the table leaving __index on the top of stack
      lua_remove(l, -2);
      true
    } else {
      false
    }
  }
}
