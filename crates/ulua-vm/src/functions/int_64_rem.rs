use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::lua_l_error::luaL_error,
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_rem"))]
pub(crate) unsafe extern "C-unwind" fn int64_rem(l: *mut LuaState) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1);
    let b = lua_l_checkinteger_64(l, 2);

    if b == 0 {
      luaL_error!(l, "division by zero");
    }

    if a == i64::MIN && b == -1 {
      lua_pushinteger_64(l, 0);
      return 1;
    }

    lua_pushinteger_64(l, a % b);

    1
  }
}
