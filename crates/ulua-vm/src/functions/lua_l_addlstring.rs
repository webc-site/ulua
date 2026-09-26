use core::ptr::copy_nonoverlapping;

use crate::{functions::extendstrbuf::extendstrbuf, records::lua_l_strbuf::LuaLStrbuf};

/// 追加字节串 `s` 到缓冲 `b`；`s` 的界由切片自带，容量不足先扩容。
///
/// # Safety
///
/// `b` 必须指向已在存活 `lua_State` 上经 `lua_l_buffinit`/`lua_l_buffinitsize` 初始化、
/// 尚未 `pushresult` 的 `LuaLStrbuf`（`p`/`end` 为其缓冲内可写游标）；不足容量经
/// `extendstrbuf` 以 boxloc `-1` 扩容（GC 分配、可抛错并重挂 `b.p`）。`s` 的读取界由
/// 切片自带，须不与 `b` 的缓冲重叠（同 cpp memcpy 前置条件）。
/// cpp laux.cpp:499 `luaL_addlstring`。
pub(crate) unsafe fn lua_l_addlstring(b: &mut LuaLStrbuf, s: &[u8]) {
  // Safety: 契约保证 b 的 p/end 为其缓冲界内游标；扩容后 p 重挂到新缓冲，
  // copy 读写均按 s.len() 收口，不越出界
  unsafe {
    let free = b.end.offset_from(b.p) as usize;
    let len = s.len();
    if free < len {
      extendstrbuf(b as *mut _, len - free, -1);
    }
    copy_nonoverlapping(s.as_ptr(), b.p as *mut u8, len);
    b.p = b.p.add(len);
  }
}
