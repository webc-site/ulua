use core::{ffi::c_int, ptr::null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tointeger_64::lua_tointeger_64, lua_type::lua_type, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_l_checkinteger_64"))]
pub unsafe fn lua_l_checkinteger_64(l: *mut lua_State, narg: c_int) -> i64 {
  unsafe {
    if lua_type(l, narg) != (LuaType::Integer as c_int) {
      tag_error(l, narg, LuaType::Integer as c_int);
    }

    lua_tointeger_64(l, narg, null_mut())
  }
}
