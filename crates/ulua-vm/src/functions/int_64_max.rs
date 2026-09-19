use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_max"))]
pub(crate) unsafe extern "C-unwind" fn int64_max(l: *mut LuaState) -> c_int {
  unsafe {
    let mut tmax: i64 = lua_l_checkinteger_64(l, 1);
    let n = lua_gettop(l);

    for i in 2..=n {
      let x = lua_l_checkinteger_64(l, i);
      if x > tmax {
        tmax = x;
      }
    }

    lua_pushinteger_64(l, tmax);

    1
  }
}
