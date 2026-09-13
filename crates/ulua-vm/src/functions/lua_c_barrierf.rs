//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:1284:lua_c_barrierf`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:1284-1294, hand-ported)

use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::reallymarkobject::reallymarkobject,
  macros::{
    gc_spause::GCSPAUSE, isblack::isblack, isdead::isdead, iswhite::iswhite,
    keepinvariant::keepinvariant, makewhite::makewhite,
  },
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_barrierf(l: *mut lua_State, o: *mut GCObject, v: *mut GCObject) {
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!(isblack!(o) && iswhite!(v) && !isdead!(g, v) && !isdead!(g, o));
    LUAU_ASSERT!((*g).gcstate as i32 != GCSPAUSE);
    // must keep invariant?
    if keepinvariant(g) {
      reallymarkobject(g, v); // restore invariant
    } else {
      // don't mind
      makewhite!(g, o); // mark as white just to avoid other barriers
    }
  }
}

pub use lua_c_barrierf as luaC_barrierf;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaC_barrierf")]
pub unsafe extern "C-unwind" fn lua_c_barrierf_export(
  l: *mut lua_State,
  o: *mut c_void,
  v: *mut c_void,
) {
  unsafe {
    lua_c_barrierf(l, o as *mut GCObject, v as *mut GCObject);
  }
}
