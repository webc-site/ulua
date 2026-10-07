use core::ptr::NonNull;

use crate::records::{
  lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  lua_state::LuaState,
};

/// 初始化 `LuaLStrbuf`（`luaL_buffinit`）：先落内部缓冲窗口，再挂当前 `l`（以 `NonNull` 句柄
/// 存入 `b.l`，串缓冲后续经它回访状态——地址即引用形传入的同一地址）。`l`/`b` 均以
/// 引用传入（存活由类型保证），纯字段写、不分配、不抛错。
pub fn lua_l_buffinit(l: &mut LuaState, b: &mut LuaLStrbuf) {
  // start with an internal buffer
  b.p = b.buffer.as_mut_ptr();
  b.end = b.p.wrapping_add(LUA_BUFFERSIZE);

  b.l = Some(NonNull::from(l));
  b.storage = None;
}
