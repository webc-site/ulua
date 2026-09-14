use core::{
  ffi::{c_int, c_void},
  mem::transmute,
};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_toboolean::lua_toboolean, lua_type::lua_type, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_checkboolean")]
pub unsafe fn lua_l_checkboolean(l: *mut lua_State, narg: c_int) -> c_int {
  unsafe {
    // This checks specifically for boolean values, ignoring
    // all other truthy/falsy values. If the desired result
    // is true if value is present then lua_toboolean should
    // directly be used instead.

    // The lua_isboolean! macro depends on lua_type.
    // Since lua_type is currently a 0-arg stub in the dependency card,
    // we must use transmute to call it with the arguments the logic requires.
    let is_bool = {
      let func: unsafe fn(*mut lua_State, c_int) -> c_int = transmute(lua_type as *const c_void);
      func(l, narg) == (LuaType::Boolean as c_int)
    };

    if !is_bool {
      tag_error(l, narg, LuaType::Boolean as c_int);
    }

    // The dependency card for lua_toboolean shows it as a 0-arg stub.
    // In a real Luau build, this is lua_toboolean(l, narg).
    // We call it via transmute to satisfy the required signature.
    let func_toboolean: unsafe fn(*mut lua_State, c_int) -> c_int =
      transmute(lua_toboolean as *const c_void);

    func_toboolean(l, narg)
  }
}
