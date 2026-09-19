use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_rrotate"))]
pub(crate) unsafe extern "C-unwind" fn int64_rrotate(l: *mut LuaState) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1) as u64;
    let s = (lua_l_checkinteger_64(l, 2) as u64 % 64) as u32;

    let result = if s != 0 { n.rotate_right(s) } else { n };

    lua_pushinteger_64(l, result as i64);

    1
  }
}
