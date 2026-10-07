//! Source: `VM/src/lgc.cpp` (lgc.cpp:1327-1347, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{
    gc_spause::GCSPAUSE, gray_2_black::gray2black, isgray::isgray, keepinvariant::keepinvariant,
    lua_c_barrier::lua_c_barrier, makewhite::makewhite, upisopen::upisopen,
  },
  records::{gc_object::GCObject, lua_state::LuaState, up_val::UpVal},
};

/// # Safety
/// `uv` 须为存活且已关闭（`upisopen!` 恒假）的 UpVal，其 `v`/GC 头自洽；`l` 的存活与独占由
/// `&mut LuaState` 承载（r19-w4 收形），但 `uv` 为调用方传入的裸指针、体内经 `lua_c_barrier!`
/// 解引用其 `(*uv).v`，依 §2 判例保持 `unsafe fn`。cpp `lgc.cpp:1327 luaC_upvalclosed`。
pub(crate) unsafe fn lua_c_upvalclosed(l: &mut LuaState, uv: *mut UpVal) {
  unsafe {
    let g = l.global;
    let o = uv as *mut GCObject;

    LUAU_ASSERT!(!upisopen!(uv)); // upvalue was closed but needs GC state fixup

    if isgray!(o) {
      if keepinvariant(&*g) {
        gray2black!(o); // closed upvalues need barrier
        lua_c_barrier!(l, uv, (*uv).v);
      } else {
        // sweep phase: sweep it (turning it into white)
        makewhite!(g, o);
        LUAU_ASSERT!((*g).gcstate as i32 != GCSPAUSE);
      }
    }
  }
}
