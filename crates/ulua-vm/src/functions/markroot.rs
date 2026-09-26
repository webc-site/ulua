//! Source: `VM/src/lgc.cpp` (lgc.cpp:805-837, hand-ported)

use core::ptr::{addr_of_mut, null_mut};

use crate::{
  functions::{markmt::markmt, marktaggetmt::marktaggetmt, markudatadirect::markudatadirect},
  macros::{gc_spropagate::GCSPROPAGATE, markobject::markobject, markvalue::markvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// 仅在 GC 周期起点（Spropagate 入口）调用：`l` 为主线程且其 global 根集合（gt、registry、mt、
/// udatadirect 等）完整存活；函数会把 gray/grayagain/weak 链头重置为 null，周期中途调用将丢失
/// 在途灰对象致其被误回收。cpp lgc.cpp:917 `markroot`
// mark root set
pub(crate) unsafe fn markroot(l: *mut LuaState) {
  // Safety: 契约保证 `l` 存活且其 global 根集合（registry/mainthread/串表等）完整，逐根标记仅写各自对象灰白标签
  unsafe {
    let g = (*l).global;
    (*g).gray = null_mut();
    (*g).grayagain = null_mut();
    (*g).weak = null_mut();
    markobject!(g, (*g).mainthread);
    // make global table be traversed before main stack
    markobject!(g, (*(*g).mainthread).gt);
    // registry(l) — &l->global->registry
    markvalue!(g, addr_of_mut!((*g).registry));

    // DELIBERATE DEVIATION 注记见 markudatadirect：本仓按 fflag 门控（cpp lgc.cpp:934/1031）
    markudatadirect(g);

    markmt(g);
    // cpp lgc.cpp:930：补标标签 userdata 元表（GC 根之一，缺失会导致元表被 sweep）
    marktaggetmt(g);
    (*g).gcstate = GCSPROPAGATE as u8;
  }
}
