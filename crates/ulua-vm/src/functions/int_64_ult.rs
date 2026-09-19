use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_ult"))]
pub(crate) unsafe extern "C-unwind" fn int64_ult(l: *mut LuaState) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1) as u64;
    let b = lua_l_checkinteger_64(l, 2) as u64;

    lua_pushboolean(l, (a < b) as c_int);

    1
  }
}
