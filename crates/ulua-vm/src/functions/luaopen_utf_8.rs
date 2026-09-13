use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  functions::{
    byteoffset::byteoffset, codepoint::codepoint, iter_codes::iter_codes,
    lua_l_register::lua_l_register, lua_pushlstring::lua_pushlstring, lua_setfield::lua_setfield,
    utfchar::utfchar, utflen::utflen,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

struct SyncLuaLReg([LuaLReg; 6]);
unsafe impl Sync for SyncLuaLReg {}

static FUNCS: SyncLuaLReg = SyncLuaLReg([
  LuaLReg {
    name: c"offset".as_ptr(),
    func: Some(byteoffset),
  },
  LuaLReg {
    name: c"codepoint".as_ptr(),
    func: Some(codepoint),
  },
  LuaLReg {
    name: c"char".as_ptr(),
    func: Some(utfchar),
  },
  LuaLReg {
    name: c"len".as_ptr(),
    func: Some(utflen),
  },
  LuaLReg {
    name: c"codes".as_ptr(),
    func: Some(iter_codes),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_utf_8(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_register(l, c"utf8".as_ptr(), FUNCS.0.as_ptr());

    // UTF8PATT = "[\0-\x7F\xC2-\xF4][\x80-\xBF]*" — contains an embedded NUL, so a
    // byte slice (not a C string literal). 14 bytes, pushed via lua_pushlstring.
    const UTF8_PATT: [u8; 14] = [
      0x5B, 0x00, 0x2D, 0x7F, 0xC2, 0x2D, 0xF4, 0x5D, 0x5B, 0x80, 0x2D, 0xBF, 0x5D, 0x2A,
    ];
    lua_pushlstring(l, UTF8_PATT.as_ptr() as *const c_char, UTF8_PATT.len());
    lua_setfield(l, -2, c"charpattern".as_ptr());

    1
  }
}
