use crate::{
  functions::{
    byteoffset::byteoffset_arm, codepoint::codepoint_arm, iter_codes::iter_codes_arm,
    lua_l_register::lua_l_register_bytes, lua_pushlstring::lua_pushlstring_bytes,
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

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；
/// r12-w6d 后建模块表/注册/压串所经 `lua_l_register_bytes`/`lua_pushlstring_bytes` 皆
/// 降为安全 `fn`（二者内部裸操作各由被调自身窄块与契约承担），本门面调用点不再落
/// `unsafe`，故本体维持安全 `fn`）：`l` 须为可分配、可抛错的受保护帧，`FUNCS` 为本文件
/// 静态的合法 C 臂表（名字为不含尾部 `\0` 的静态字节切片，满足切片契约）；末尾
/// `set_field_bytes(-2, ...)` 要求 charpattern 串已压栈、模块表在 -2。
pub fn luaopen_utf_8(l: &mut LuaState) -> i32 {
  lua_l_register_bytes(l, Some(b"utf8"), &FUNCS);

  // UTF8PATT = "[\0-\x7F\xC2-\xF4][\x80-\xBF]*" — contains an embedded NUL, so a
  // byte slice (not a C string literal). 14 bytes, pushed via lua_pushlstring.
  const UTF8_PATT: [u8; 14] = [
    0x5B, 0x00, 0x2D, 0x7F, 0xC2, 0x2D, 0xF4, 0x5D, 0x5B, 0x80, 0x2D, 0xBF, 0x5D, 0x2A,
  ];
  lua_pushlstring_bytes(l, &UTF8_PATT);
  l.set_field_bytes(-2, b"charpattern");

  1
}

lua_lib_fn!(pub fn luaopen_utf_8 @ref, luaopen_utf_8_arm);
