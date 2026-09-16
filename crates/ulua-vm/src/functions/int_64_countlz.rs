use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// cpp lintlib.cpp `int64_countlz`：前零计数，n==0 时为 64
///（`u64::leading_zeros` 对 0 的返回值与 `__builtin_clzll` 语义一致）。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_countlz"))]
pub(crate) unsafe extern "C-unwind" fn int64_countlz(l: *mut LuaState) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1) as u64;

    lua_pushinteger_64(l, n.leading_zeros() as i64);

    1
  }
}
