use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// cpp lintlib.cpp `int64_bswap`：字节序翻转（等价 `__builtin_bswap64`）。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_bswap"))]
pub(crate) unsafe extern "C-unwind" fn int64_bswap(l: *mut LuaState) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1) as u64;

    lua_pushinteger_64(l, a.swap_bytes() as i64);

    1
  }
}
