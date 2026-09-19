use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_sub"))]
pub(crate) unsafe extern "C-unwind" fn int64_sub(l: *mut LuaState) -> c_int {
  unsafe {
    let x = lua_l_checkinteger_64(l, 1);
    let y = lua_l_checkinteger_64(l, 2);

    let result = (x as u64).wrapping_sub(y as u64) as i64;

    lua_pushinteger_64(l, result);

    1
  }
}
