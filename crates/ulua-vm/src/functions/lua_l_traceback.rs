use core::{ffi::c_char, mem::zeroed};

use itoa::Buffer;

use crate::{
  functions::{
    cstr_bytes, lua_getinfo::lua_getinfo, lua_l_addchar::lua_l_addchar,
    lua_l_addlstring::lua_l_addlstring, lua_l_addstring::lua_l_addstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_pushresult::lua_l_pushresult,
  },
  records::{lua_debug::LuaDebug, lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// Build a traceback string from `l1`, optionally prepending `msg`, and push
/// the result onto `l`. Faithful 1:1 port of `luaL_traceback` from
/// `luau/VM/src/laux.cpp:381-425`.
/// # Safety
/// `l`/`l1` 均须指向存活 `LuaState`：`l1` 的调用栈自 `level` 起各帧可读（逐帧 lua_getinfo 游走），`l` 承接
/// 最终 pushresult 压栈；`msg` 仅按 C 语义截读至首个 NUL。对应 cpp laux.cpp:377。
pub unsafe fn lua_l_traceback(l: *mut LuaState, l1: *mut LuaState, msg: Option<&str>, level: i32) {
  // Safety: 契约保证 `L1` 调用栈自 level 起可读、`buf` 可写，帧遍历按 lua_getinfo 语义在界内推进
  unsafe {
    debug_assert!(level >= 0);

    // 行号十进制直写：itoa 输出与 `core::fmt` 逐字节一致，
    // 取代 cpp `laux.cpp:401-408` 的手写 `% 10` 反序循环（0 与负数同样正确）
    /// # Safety
    ///
    /// `buf` 必须指向存活且已由 `lua_l_buffinit` 初始化的 `LuaLStrbuf`；
    /// `lua_l_addlstring` 会经其可写游标追加 itoa 输出。
    unsafe fn addsignednum(buf: &mut LuaLStrbuf, n: i32) {
      let mut digits = Buffer::new();
      let s = digits.format(n);
      unsafe { lua_l_addlstring(buf, s.as_bytes()) };
    }

    let mut buf = LuaLStrbuf::new();
    lua_l_buffinit(l, &mut buf);

    if let Some(msg_str) = msg {
      // cpp: `luaL_addstring(B, msg)` 即 `luaL_addlstring(B, msg, strlen(msg))`
      // （laux.cpp:391）：按 C 语义截断于首个 NUL。直接对 &[u8] 前缀零拷贝
      // 追加，去掉 CString 运行时分配。
      let bytes = msg_str.as_bytes();
      let end = memchr::memchr(0, bytes).unwrap_or(bytes.len());
      lua_l_addlstring(&mut buf, &bytes[..end]);
      lua_l_addstring(&mut buf, c"\n".as_ptr());
    }

    let mut ar: LuaDebug = zeroed();
    let mut i: i32 = level;

    while lua_getinfo(l1, i, c"sln".as_ptr(), &mut ar) != 0 {
      if cstr_bytes(ar.what) == b"C" {
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
