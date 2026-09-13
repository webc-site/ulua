use core::{
  ffi::{CStr, c_char, c_int},
  ptr::null,
};

use crate::{
  enums::lua_type::{LUA_T_COUNT, LUA_TNONE},
  macros::api_check::api_check,
  records::lua_state::lua_State,
};

const TYPENAMES: [&str; 14] = [
  "nil",      // 0: LUA_TNIL
  "boolean",  // 1: LUA_TBOOLEAN
  "userdata", // 2: LUA_TLIGHTUSERDATA
  "number",   // 3: LUA_TNUMBER
  "integer",  // 4: LUA_TINTEGER
  "vector",   // 5: LUA_TVECTOR
  "string",   // 6: LUA_TSTRING
  "table",    // 7: LUA_TTABLE
  "function", // 8: LUA_TFUNCTION
  "userdata", // 9: LUA_TUSERDATA
  "thread",   // 10: LUA_TTHREAD
  "buffer",   // 11: LUA_TBUFFER
  "class",    // 12: LUA_TCLASS
  "object",   // 13: LUA_TOBJECT
];

const TYPENAMES_C: [&CStr; 14] = [
  c"nil",
  c"boolean",
  c"userdata",
  c"number",
  c"integer",
  c"vector",
  c"string",
  c"table",
  c"function",
  c"userdata",
  c"thread",
  c"buffer",
  c"class",
  c"object",
];

/// 返回类型 ID 对应的静态类型名称 `&'static str`（零堆分配）。
///
/// 若 `t` 为有效类型（包含 `LUA_TNONE`），返回 `Some(&'static str)`，否则返回 `None`。
#[inline]
pub fn lua_typename_str(t: i32) -> Option<&'static str> {
  if t == LUA_TNONE {
    Some("no value")
  } else if t >= 0 && (t as usize) < TYPENAMES.len() {
    Some(TYPENAMES[t as usize])
  } else {
    None
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_typename(_l: *mut lua_State, t: c_int) -> *const c_char {
  api_check!(l, t >= LUA_TNONE && t < LUA_T_COUNT as c_int);

  if t == LUA_TNONE {
    c"no value".as_ptr()
  } else if t >= 0 && (t as usize) < TYPENAMES_C.len() {
    TYPENAMES_C[t as usize].as_ptr()
  } else {
    null()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lua_typename_str() {
    assert_eq!(lua_typename_str(-1), Some("no value"));
    assert_eq!(lua_typename_str(0), Some("nil"));
    assert_eq!(lua_typename_str(1), Some("boolean"));
    assert_eq!(lua_typename_str(2), Some("userdata"));
    assert_eq!(lua_typename_str(3), Some("number"));
    assert_eq!(lua_typename_str(6), Some("string"));
    assert_eq!(lua_typename_str(7), Some("table"));
    assert_eq!(lua_typename_str(8), Some("function"));
    assert_eq!(lua_typename_str(11), Some("buffer"));
    assert_eq!(lua_typename_str(13), Some("object"));
    assert_eq!(lua_typename_str(14), None);
    assert_eq!(lua_typename_str(-2), None);
  }
}
