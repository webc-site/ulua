use core::ffi::c_char;

use crate::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, lua_o_utf_8_esc::lua_o_utf_8_esc},
  macros::{lua_l_argcheck::luaL_argcheck, maxunicode::MAXUNICODE, utf_8_buffsz::UTF8BUFFSZ},
  records::lua_state::LuaState,
};

/// cpp `lutf8lib.cpp:146 buffutfchar`：把 `arg` 栈槽码点 UTF-8 编码进 `buff` 尾部，
/// C++ 的 `charstr`/长度双出参折叠为返回的编码字节切片（借用 `buff`）。
///
/// # Safety
///
/// `l` 必须是正在执行的 utf8 库 C 函数帧的存活 `LuaState`，`arg` 为其合法栈索引；
/// 码点不在 `[0, MAXUNICODE]` 时经 `luaL_argcheck` 抛错、不返回。`buff` 为调用方自有的
/// UTF8BUFFSZ 栈缓冲（对应 cpp 局部 `char buff[UTF8BUFFSZ]`），写入只回退占用其尾部。
/// cpp lutf8lib.cpp:146。
pub(crate) unsafe fn buffutfchar(
  l: *mut LuaState,
  arg: i32,
  buff: &mut [c_char; UTF8BUFFSZ],
) -> &[c_char] {
  // Safety: 契约保证 l 为存活调用帧、arg 栈槽可读；编码字节数 n ∈ [1, UTF8BUFFSZ]，
  // 尾部切片由下标检查自证界内
  unsafe {
    let code = lua_l_checkinteger(l, arg);
    luaL_argcheck!(
      l,
      (0..=MAXUNICODE).contains(&code),
      arg,
      "value out of range"
    );

    let n = lua_o_utf_8_esc(buff, (code as i64) as u32);
    &buff[UTF8BUFFSZ - n as usize..]
  }
}
