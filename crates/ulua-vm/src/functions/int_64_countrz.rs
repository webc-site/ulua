use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// cpp lintlib.cpp `int64_countrz`：尾零计数，n==0 时为 64
///（`u64::trailing_zeros` 对 0 的返回值与 `__builtin_ctzll` 语义一致）。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_countrz"))]
pub(crate) unsafe extern "C-unwind" fn int64_countrz(l: *mut LuaState) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1) as u64;

    lua_pushinteger_64(l, n.trailing_zeros() as i64);

    1
  }
}
