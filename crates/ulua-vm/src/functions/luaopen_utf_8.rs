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

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 建模块表/注册/压串仍经 `lua_l_register_bytes`/`lua_pushlstring_bytes` 两处不安全被调而落窄块
/// （被调方自身保留 `# Safety`：裸 C 函数指针与 intern 转手未清零），其余门面皆安全，故本体降为
/// 安全 `fn`）：`l` 须为可分配、可抛错的受保护帧，`FUNCS` 为本文件静态的合法 C 臂表；末尾
/// `set_field_bytes(-2, ...)` 要求 charpattern 串已压栈、模块表在 -2。
pub fn luaopen_utf_8(l: &mut LuaState) -> i32 {
  // SAFETY: `FUNCS` 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，
  // 名字为不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约
  unsafe { lua_l_register_bytes(l, Some(b"utf8"), &FUNCS) };

  // UTF8PATT = "[\0-\x7F\xC2-\xF4][\x80-\xBF]*" — contains an embedded NUL, so a
  // byte slice (not a C string literal). 14 bytes, pushed via lua_pushlstring.
  const UTF8_PATT: [u8; 14] = [
    0x5B, 0x00, 0x2D, 0x7F, 0xC2, 0x2D, 0xF4, 0x5D, 0x5B, 0x80, 0x2D, 0xBF, 0x5D, 0x2A,
  ];
  // SAFETY: `lua_pushlstring_bytes` 切片核心契约自具（l 由 &mut 承载存活/独占，界内拷入
  // 堆上 TString、不留借出窗）；UTF8_PATT 为本文件自有的界内常量切片
  unsafe { lua_pushlstring_bytes(l, &UTF8_PATT) };
  l.set_field_bytes(-2, b"charpattern");

  1
}

lua_lib_fn!(pub fn luaopen_utf_8 @ref, luaopen_utf_8_arm);
