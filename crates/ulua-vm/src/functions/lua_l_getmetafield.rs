use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getmetatable::lua_getmetatable, lua_pushstring::lua_pushstring, lua_rawget::lua_rawget,
    lua_remove::lua_remove, lua_type::lua_type,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可分配/GC 的受保护帧；`obj` 为合法栈索引（`lua_getmetatable` 读该值元表），
/// `event` 须为 NUL 结尾 C 串（`lua_pushstring` intern 后 `lua_rawget` 查元表）；命中时把元方法留在栈顶并移除元表。
/// cpp/VM/src/laux.cpp:279 luaL_getmetafield。
pub unsafe fn lua_l_getmetafield(l: *mut LuaState, obj: i32, event: *const c_char) -> i32 {
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
