use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::sizestring::sizestring,
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState, t_string::tstring},
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global.strt` 有效（`Stringtable::unlink` 会改桶链、
/// 按 `removed` 维护 `nuse`）；`ts` 须指向正在被回收的存活 tstring（`(*ts).len`/`hash`/
/// `hdr.memcat`/`next` 可读，须已不在其它强引用下）；`page` 为该字符串所属 lua_Page，
/// 可为空由分配器语义决定。cpp/VM/src/lstring.cpp:185 luaS_free。
pub unsafe fn lua_s_free(l: *mut LuaState, ts: *mut tstring, page: *mut lua_Page) {
  unsafe {
    let g = (*l).global;
    let removed = (*g).strt.unlink(ts);
    let s = &*ts; // unlink 可能改写桶链，读取放在其后
    if removed {
      (*g).strt.nuse = (*g).strt.nuse.wrapping_sub(1);
    } else {
      LUAU_ASSERT!(s.next.is_null()); // orphaned string buffer
    }

    lua_m_freegco(
      l,
      ts as *mut GCObject,
      sizestring(s.len as usize),
      s.hdr.memcat,
      page,
    );
  }
}
