use itoa::Buffer;

use crate::{
  functions::{
    lua_getinfo::lua_getinfo, lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_pushresult::lua_l_pushresult,
  },
  records::{
    lua_debug::{LuaDebug, LuaWhat},
    lua_l_strbuf::LuaLStrbuf,
    lua_state::LuaState,
  },
};

/// Build a traceback string from `l1`, optionally prepending `msg`, and push
/// the result onto `l`. Faithful 1:1 port of `luaL_traceback` from
/// `luau/VM/src/laux.cpp:381-425`.
/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v43 收形，buffinit 转手经 `&mut *l` 一次性重借用）；
/// `l1` 仍收裸形并转交 `lua_getinfo`，须指向存活 `LuaState`：其调用栈自 `level` 起各帧可读（逐帧
/// lua_getinfo 游走），`l` 承接最终 pushresult 压栈；`msg` 仅按 C 语义截读至首个 NUL。对应 cpp laux.cpp:377。
pub unsafe fn lua_l_traceback(l: &mut LuaState, l1: *mut LuaState, msg: Option<&str>, level: i32) {
  // SAFETY: 契约保证 `L1` 调用栈自 level 起可读、`buf` 可写，帧遍历按 lua_getinfo 语义在界内推进
  unsafe {
    debug_assert!(level >= 0);

    let mut buf = LuaLStrbuf::new();
    lua_l_buffinit(&mut *l, &mut buf);

    if let Some(msg_str) = msg {
      // cpp: `luaL_addstring(B, msg)` 即 `luaL_addlstring(B, msg, strlen(msg))`
      // （laux.cpp:391）：按 C 语义截断于首个 NUL。直接对 &[u8] 前缀零拷贝
      // 追加，去掉运行时堆分配。
      let bytes = msg_str.as_bytes();
      let end = memchr::memchr(0, bytes).unwrap_or(bytes.len());
      lua_l_addlstring(&mut buf, &bytes[..end]);
      lua_l_addchar(&mut buf, b'\n');
    }

    let mut ar: LuaDebug = LuaDebug::default();
    let mut num = Buffer::new();
    let mut i: i32 = level;

    while lua_getinfo(l1, i, b"sln", &mut ar) != 0 {
      if ar.what == LuaWhat::C {
        i += 1;
        continue;
      }

      if ar.source.is_some() {
        lua_l_addlstring(&mut buf, ar.short_src.as_deref().unwrap_or(b""));
      }

      if ar.currentline > 0 {
        lua_l_addchar(&mut buf, b':');
        lua_l_addlstring(&mut buf, num.format(ar.currentline).as_bytes());
      }

      if let Some(name) = &ar.name {
        lua_l_addlstring(&mut buf, b" function ");
        lua_l_addlstring(&mut buf, name);
      }

      lua_l_addchar(&mut buf, b'\n');

      i += 1;
    }

    lua_l_pushresult(&mut buf);
  }
}
