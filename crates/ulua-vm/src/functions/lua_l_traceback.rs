use core::{
  ffi::{CStr, c_char, c_int},
  mem::zeroed,
  ptr::null_mut,
};
use std::ffi::CString;

use crate::{
  functions::{
    lua_getinfo::lua_getinfo, lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_addstring::lua_l_addstring, lua_l_buffinit::lua_l_buffinit,
    lua_l_pushresult::lua_l_pushresult,
  },
  records::{
    lua_debug::LuaDebug,
    lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  },
  type_aliases::lua_state::lua_State,
};

/// Build a traceback string from `l1`, optionally prepending `msg`, and push
/// the result onto `l`. Faithful 1:1 port of `luaL_traceback` from
/// `luau/VM/src/laux.cpp:381-425`.
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_traceback(
  l: *mut lua_State,
  l1: *mut lua_State,
  msg: Option<&str>,
  level: c_int,
) {
  unsafe {
    debug_assert!(level >= 0);

    // 行号十进制直写：itoa 输出与 `core::fmt` 逐字节一致，
    // 取代 cpp `laux.cpp:401-408` 的手写 `% 10` 反序循环（0 与负数同样正确）
    unsafe fn addsignednum(buf: *mut LuaLStrbuf, n: i32) {
      unsafe {
        let mut digits = itoa::Buffer::new();
        let s = digits.format(n);
        lua_l_addlstring(buf, s.as_ptr().cast::<c_char>(), s.len());
      }
    }

    let mut buf = LuaLStrbuf::new();
    lua_l_buffinit(l, &mut buf);

    if let Some(msg_str) = msg {
      let c_msg = CString::new(msg_str).unwrap_or_default();
      lua_l_addstring(&mut buf, c_msg.as_ptr());
      lua_l_addstring(&mut buf, c"\n".as_ptr());
    }

    let mut ar: LuaDebug = zeroed();
    let mut i: i32 = level;

    while lua_getinfo(l1, i, c"sln".as_ptr(), &mut ar) != 0 {
      if CStr::from_ptr(ar.what).to_bytes() == b"C" {
        i += 1;
        continue;
      }

      if !ar.source.is_null() {
        lua_l_addstring(&mut buf, ar.short_src);
      }

      if ar.currentline > 0 {
        lua_l_addchar(&mut buf, b':' as c_char);
        addsignednum(&mut buf, ar.currentline);
      }

      if !ar.name.is_null() {
        lua_l_addstring(&mut buf, c" function ".as_ptr());
        lua_l_addstring(&mut buf, ar.name);
      }

      lua_l_addchar(&mut buf, b'\n' as c_char);

      i += 1;
    }

    lua_l_pushresult(&mut buf);
  }
}
