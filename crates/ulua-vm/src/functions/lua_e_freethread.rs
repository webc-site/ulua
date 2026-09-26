//! Source: `VM/src/lstate.cpp:130-138` (hand-ported)

use core::{mem::size_of, ptr::null_mut};

use crate::{
  functions::{freestack::freestack, lua_m_freegco::lua_m_freegco},
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global` 有效（记账/回调宿主，须非被释放线程本身）；`l1` 须为待释放的存活
/// 子线程（`(*l1).hdr.memcat` 可读、其栈已由 `freestack` 归还）；`page` 为 `l1` 所属 lua_Page，可为空由分配器
/// 语义决定。`userthread` 回调以 `null_mut()` 通知析构。cpp/VM/src/lstate.cpp:160 luaE_freethread。
pub unsafe fn lua_e_freethread(l: *mut LuaState, l1: *mut LuaState, page: *mut lua_Page) {
  unsafe {
    let g = (*l).global;
    if let Some(userthread) = (*g).cb.userthread {
      userthread(null_mut(), l1);
    }

    freestack(l, l1);
    lua_m_freegco(
      l,
      l1 as *mut GCObject,
      size_of::<LuaState>(),
      (*l1).hdr.memcat,
      page,
    );
  }
}
