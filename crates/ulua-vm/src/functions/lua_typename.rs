use core::{ffi::c_char, ptr::null};

use crate::{
  enums::lua_type::{LUA_T_COUNT, LUA_TNONE},
  macros::api_check::api_check,
  records::lua_state::LuaState,
};

/// 各 `LUA_T*` 类型 ID 对应的字符串名字（`lua_typename` 与全局 `ttname`
/// interning 共用，索引即类型 ID）。
pub(crate) const TYPENAMES_STR: [&str; 14] = [
  "nil", "boolean", "userdata", "number", "integer", "vector", "string", "table", "function",
  "userdata", "thread", "buffer", "class", "object",
];

/// 各 `LUA_T*` 类型 ID 对应的 NUL 结尾字节切片（C 契约调用点 `.as_ptr().cast()`）。
pub(crate) const TYPENAMES_BYTES: [&[u8]; 14] = [
  b"nil\0",
  b"boolean\0",
  b"userdata\0",
  b"number\0",
  b"integer\0",
  b"vector\0",
  b"string\0",
  b"table\0",
  b"function\0",
  b"userdata\0",
  b"thread\0",
  b"buffer\0",
  b"class\0",
  b"object\0",
];

/// `LUA_TNONE`/空槽的显示名（cpp `luaO_nvalue... "no value"`）：`lua_typename` 与
/// `luaL_typename` 共用同一静态串，收口为单点常量。
pub(crate) const NO_VALUE_STR: &str = "no value";
pub(crate) const NO_VALUE_BYTES: &[u8] = b"no value\0";

/// 返回类型 ID 对应的静态类型名称 `&'static str`（零堆分配）。
///
/// 若 `t` 为有效类型（包含 `LUA_TNONE`），返回 `Some(&'static str)`，否则返回 `None`。
#[inline]
pub fn lua_typename_str(t: i32) -> Option<&'static str> {
  if t == LUA_TNONE {
    Some(NO_VALUE_STR)
  } else {
    // 负 `t` 经 `as usize` 回绕为巨值，`get` 直接落 None——同旧 `t >= 0` 判界
    TYPENAMES_STR.get(t as usize).copied()
  }
}

/// 返回类型 ID 对应的 C 字符串名（cpp `lua_typename`，lapi.cpp:211）。
///
/// 本函数体仅读静态表与做 `api_check!`（`$l` 弃用、不触碰状态），不解引用 `l`。
pub fn lua_typename(_l: *mut LuaState, t: i32) -> *const c_char {
  api_check!(l, t >= LUA_TNONE && t < LUA_T_COUNT as i32);

  if t == LUA_TNONE {
    NO_VALUE_BYTES.as_ptr().cast()
  } else {
    // 负 `t` 经 `as usize` 回绕为巨值，`get` 直接落 None——同旧 `t >= 0` 判界
    TYPENAMES_BYTES
      .get(t as usize)
      .map_or(null(), |name| name.as_ptr().cast())
  }
}
