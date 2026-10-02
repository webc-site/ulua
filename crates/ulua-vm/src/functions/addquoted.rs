//! Source: `VM/src/lstrlib.cpp:884`
//!
//! Helper for `string.format("%q", s)` — append `s` to the buffer as a quoted,
//! escapable string literal: wrap in `"`, backslash-escape `"`/`\`/newline,
//! emit `\r` and `\000` for CR and NUL, pass everything else through.
//!
//! r12-w5s T9 切片化：真实逻辑全部落在切片核心 [`addquoted_ref`]（逐字节转义拼接在
//! `&[u8]` 视图上进行）。旧 `(l, b, arg)` 取参垫片经消费面实测为零消费——全仓唯一
//! 调用方 `str_format` 直接以 `check_bytes` 切片实参喂入，垫片随窗口构造一并退役，
//! 本文件不再出现任何 `from_raw_parts` 裸窗。

use crate::{
  functions::{
    lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_prepbuffsize::lua_l_prepbuffsize,
  },
  records::lua_l_strbuf::LuaLStrbuf,
};

/// 引号化转义拼接（切片核心，真实逻辑）：cpp `addquoted`（lstrlib.cpp:884）的
/// `luaL_prepbuffsize(b, l + 2)` 预扩 + `"` 包裹 + 逐字节 switch 转义循环同形——
/// `"`/`\`/`\n` 前置反斜杠、`\r` 写 `"\\r"`、`\0` 写 `"\\000"`，其余原样透传，
/// 拼接顺序与输出字节与 oracle 逐点一致。
///
/// # Safety
/// `b` 须为已在存活 `lua_State` 上经 `lua_l_buffinit` 初始化、尚未 `pushresult` 的
/// 拼接缓冲（`p`/`end` 可写游标契约同 [`lua_l_addchar`]）；`bytes` 的读取界由切片
/// 自带，且不与 `b` 的内部缓冲重叠（cpp `memcpy` 同源前置条件）。
pub(crate) unsafe fn addquoted_ref(b: &mut LuaLStrbuf, bytes: &[u8]) {
  // SAFETY: 契约保证 `b` 为可写缓冲游标态，预扩 `len + 2` 后逐字节转义拼接
  // （addchar/addlstring）均落在扩容后的可写区，写入总量 ≤ bytes.len() + 2
  unsafe {
    lua_l_prepbuffsize(b, bytes.len() + 2);

    lua_l_addchar(b, b'"');
    for &c in bytes {
      match c {
        b'"' | b'\\' | b'\n' => {
          lua_l_addchar(b, b'\\');
          lua_l_addchar(b, c);
        }
        b'\r' => {
          lua_l_addlstring(b, b"\\r");
        }
        b'\0' => {
          lua_l_addlstring(b, b"\\000");
        }
        _ => {
          lua_l_addchar(b, c);
        }
      }
    }
    lua_l_addchar(b, b'"');
  }
}
