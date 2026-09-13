//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:1327:lua_c_upvalclosed`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:1327-1347, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{
    gc_spause::GCSPAUSE, gray_2_black::gray2black, isgray::isgray, keepinvariant::keepinvariant,
    lua_c_barrier::luaC_barrier, makewhite::makewhite, upisopen::upisopen,
  },
  records::{gc_object::GCObject, up_val::UpVal},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_upvalclosed(l: *mut lua_State, uv: *mut UpVal) {
  unsafe {
    let g = (*l).global;
    let o = uv as *mut GCObject;

    LUAU_ASSERT!(!upisopen!(uv)); // upvalue was closed but needs GC state fixup

    if isgray!(o) {
      if keepinvariant(g) {
        gray2black!(o); // closed upvalues need barrier
        luaC_barrier!(l, uv, (*uv).v);
      } else {
        // sweep phase: sweep it (turning it into white)
        makewhite!(g, o);
        LUAU_ASSERT!((*g).gcstate as i32 != GCSPAUSE);
      }
    }
  }
}

pub use lua_c_upvalclosed as luaC_upvalclosed;
