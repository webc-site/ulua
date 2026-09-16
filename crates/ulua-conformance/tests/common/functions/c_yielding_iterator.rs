use ulua_vm::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_pushinteger::lua_pushinteger, lua_yield::lua_yield,
  },
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn c_yielding_iterator(l: *mut lua_State) -> i32 {
  unsafe {
    let max = lua_l_checkinteger(l, 1);
    let index = lua_l_checkinteger(l, 2);

    if index >= max {
      return 0; // nil: end iteration
    }

    lua_pushinteger(l, index + 1);
    lua_yield(l, 1)
  }
}
