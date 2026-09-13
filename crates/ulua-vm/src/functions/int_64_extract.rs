use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_l_optinteger_64::lua_l_optinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error, mask_64::mask64},
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_extract")]
pub(crate) unsafe fn int64_extract(l: *mut LuaState) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1);
    let f = lua_l_checkinteger_64(l, 2);
    let w = lua_l_optinteger_64(l, 3, 1);

    luaL_argcheck!(l, (0..=63).contains(&f), 2, "field cannot be negative");
    luaL_argcheck!(l, 0 < w, 3, "width must be positive");
    // `f` is bounded to [0,63] above; compare `w > 64 - f` so a near-i64::MAX
    // width can't overflow the `f + w` addition.
    if w > 64 - f {
      luaL_error!(l, "trying to access non-existent bits");
    }

    lua_pushinteger_64(l, (((n as u64) >> f as u32) & mask64(w as i32)) as i64);

    1
  }
}
