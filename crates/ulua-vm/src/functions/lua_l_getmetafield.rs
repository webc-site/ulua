use core::ffi::c_char;

use crate::{
  functions::{
    lua_pushlstring::lua_pushlstring_bytes, lua_pushstring::lua_pushstring, lua_rawget::lua_rawget,
  },
  records::lua_state::LuaState,
};

/// 键入栈后的元表 rawget 主体（`lua_getmetatable` 已将元表压到 -2，调用方已压入键）：
/// 命中时把元方法留在栈顶并移除元表，未命中弹回键与元表。
///
/// # Safety
/// `l` 须为存活 LuaState 且栈顶为键、-2 为元表（[`lua_l_getmetafield_bytes`] /
/// [`lua_l_getmetafield`] 的前置即此）。
unsafe fn metafield_rawget(l: *mut LuaState) -> i32 {
  unsafe {
    lua_rawget(l, -2);

    if (*l).is_nil(-1) {
      (*l).pop(2); // remove metatable and metafield
      0
    } else {
      (*l).remove(-2); // remove only metatable
      1
    }
  }
}

/// [`lua_l_getmetafield`] 的字节切片核心（§10：Rust 内部调用方一律走此形，
/// 不构造 NUL 结尾缓冲）。
///
/// # Safety
/// `l` 须为存活 LuaState 并处于可分配/GC 的受保护帧；`obj` 为合法栈索引（`lua_getmetatable` 读该值元表），
/// `event` 为元表键字节切片（`lua_pushlstring_bytes` intern 后 `lua_rawget` 查元表）；命中时把元方法留在栈顶并移除元表。
/// cpp/VM/src/laux.cpp:279 luaL_getmetafield。
pub unsafe fn lua_l_getmetafield_bytes(l: *mut LuaState, obj: i32, event: &[u8]) -> i32 {
  unsafe {
    if !(*l).get_metatable(obj) {
      return 0; // no metatable
    }

    lua_pushlstring_bytes(l, event);
    metafield_rawget(l)
  }
}

/// # Safety
/// `l` 须为存活 LuaState 并处于可分配/GC 的受保护帧；`obj` 为合法栈索引（`lua_getmetatable` 读该值元表），
/// `event` 须为 NUL 结尾 C 串或 null（C ABI 契约：null 经 `lua_pushstring` 折算为 nil 键，恒未命中）；
/// 命中时把元方法留在栈顶并移除元表。cpp/VM/src/laux.cpp:279 luaL_getmetafield。
pub unsafe fn lua_l_getmetafield(l: *mut LuaState, obj: i32, event: *const c_char) -> i32 {
  unsafe {
    if !(*l).get_metatable(obj) {
      return 0; // no metatable
    }

    // 保 C 契约的 null→nil 键分支；非空即与 `lua_l_getmetafield_bytes(cstr_bytes(event))` 同路径
    lua_pushstring(l, event);
    metafield_rawget(l)
  }
}
