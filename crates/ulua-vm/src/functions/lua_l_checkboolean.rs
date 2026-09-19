use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_toboolean::lua_toboolean, lua_type::lua_type, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_l_checkboolean"))]
pub unsafe fn lua_l_checkboolean(l: *mut lua_State, narg: c_int) -> c_int {
  unsafe {
    // This checks specifically for boolean values, ignoring
    // all other truthy/falsy values. If the desired result
    // is true if value is present then lua_toboolean should
    // directly be used instead.

    let is_bool = lua_type(l, narg) == (LuaType::Boolean as c_int);

    if !is_bool {
      tag_error(l, narg, LuaType::Boolean as c_int);
    }

    lua_toboolean(l, narg)
  }
}
