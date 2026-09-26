use core::ffi::c_char;

use crate::{
  functions::{
    byteoffset::byteoffset_arm, codepoint::codepoint_arm, iter_codes::iter_codes_arm,
    lua_l_register::lua_l_register, lua_pushlstring::lua_pushlstring, lua_setfield::lua_setfield,
    utfchar::utfchar_arm, utflen::utflen_arm,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static FUNCS: [LuaLReg; 5] = [
  LuaLReg::new(b"offset", byteoffset_arm),
  LuaLReg::new(b"codepoint", codepoint_arm),
  LuaLReg::new(b"char", utfchar_arm),
  LuaLReg::new(b"len", utflen_arm),
  LuaLReg::new(b"codes", iter_codes_arm),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luaopen_utf_8(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"utf8".as_ptr(), &FUNCS);

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

lua_lib_fn!(pub fn luaopen_utf_8, luaopen_utf_8_arm);
