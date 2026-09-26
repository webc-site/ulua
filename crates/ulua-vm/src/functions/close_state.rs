//! Source: `VM/src/lstate.cpp:92-113` (hand-ported)

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    freestack::freestack, lua_c_freeall::lua_c_freeall, lua_f_close::lua_f_close,
    lua_m_free::lua_m_free,
  },
  records::{lg::LG, lua_state::LuaState, t_string::tstring},
};

/// # Safety
/// `l` 须为其所属 `global` 状态的主线程且仍存活；`(*l).global`、`(*l).stack`、`strt.hash`、
/// `freepages/freegcopages` 均须仍有效——本函数依次关闭 upvalue、`lua_c_freeall`、释放字符串表与栈、
/// 调用 `ecb.close`/`frealloc`，调用后该状态即被销毁、不可再使用；须在所有协程结束、无 GC 运行时调用。
/// cpp `lstate.cpp:111`。
pub(crate) unsafe fn close_state(l: *mut LuaState) {
  unsafe {
    let g = (*l).global;
    lua_f_close(l, (*l).stack); // close all upvalues for this thread
    lua_c_freeall(l); // collect all objects
    LUAU_ASSERT!((*g).strt.nuse == 0);
    // 桶数组显式转手（Stringtable::detach_buckets）：摘出后由 frealloc 释放
    let (hash, buckets) = (*g).strt.detach_buckets();
    lua_m_free(l, hash as *mut u8, buckets * size_of::<*mut tstring>(), 0);
    freestack(l, l);
    for page in (*g).freepages.iter() {
      LUAU_ASSERT!(page.is_null());
    }
    for page in (*g).freegcopages.iter() {
      LUAU_ASSERT!(page.is_null());
    }
    LUAU_ASSERT!((*g).allgcopages.is_null());
    LUAU_ASSERT!((*g).totalbytes == size_of::<LG>());
    LUAU_ASSERT!((*g).memcatbytes[0] == size_of::<LG>());
    for &bytes in (*g).memcatbytes.iter().skip(1) {
      LUAU_ASSERT!(bytes == 0);
    }

    if let Some(close) = (*(*l).global).ecb.close {
      close(l);
    }

    if let Some(frealloc) = (*g).frealloc {
      frealloc((*g).ud, l as *mut u8, size_of::<LG>(), 0);
    }
  }
}
