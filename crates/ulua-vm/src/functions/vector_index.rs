use core::{ffi::c_int, slice::from_raw_parts};

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_checkvector::lua_l_checkvector,
    lua_pushnumber::lua_pushnumber,
  },
  macros::{lua_l_error::luaL_error, lua_vector_size::LUA_VECTOR_SIZE},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_index(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);
    let mut len = 0usize;
    let name = lua_l_checklstring(l, 2, &mut len);

    if len == 1 {
      let ic = (*name as i32 | 0x20) - 'x' as i32;

      const W_OFFSET: i32 = -1; // 'w' - 'x'
      let ic = if ic == W_OFFSET { 3 } else { ic as usize };

      if ic < LUA_VECTOR_SIZE as usize {
        lua_pushnumber(l, (*v.add(ic)) as f64);
        return 1;
      }
    }

    let name_bytes = from_raw_parts(name as *const u8, len);
    let name = String::from_utf8_lossy(name_bytes);
    luaL_error!(l, "attempt to index vector with '{}'", name);
    0
  }
}
