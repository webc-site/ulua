use core::ffi::{c_char, c_int};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getmetatable::lua_getmetatable, lua_pushstring::lua_pushstring, lua_rawget::lua_rawget,
    lua_remove::lua_remove, lua_type::lua_type,
  },
  macros::lua_pop::lua_pop,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_getmetafield")]
pub unsafe fn lua_l_getmetafield(l: *mut lua_State, obj: c_int, event: *const c_char) -> c_int {
  unsafe {
    if lua_getmetatable(l, obj) == 0 {
      return 0; // no metatable
    }

    lua_pushstring(l, event);
    lua_rawget(l, -2);

    let is_nil = lua_type(l, -1) == (LuaType::Nil as i32);

    if is_nil {
      lua_pop(l, 2); // remove metatable and metafield
      0
    } else {
      lua_remove(l, -2); // remove only metatable
      1
    }
  }
}
