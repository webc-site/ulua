//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:1296:lua_c_barriertable`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:1296-1314, hand-ported)

use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::reallymarkobject::reallymarkobject,
  macros::{
    black_2_gray::black2gray, gc_spause::GCSPAUSE, gc_spropagateagain::GCSPROPAGATEAGAIN,
    isblack::isblack, isdead::isdead, iswhite::iswhite,
  },
  records::{gc_object::GCObject, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_barriertable(l: *mut lua_State, t: *mut LuaTable, v: *mut GCObject) {
  unsafe {
    let g = (*l).global;
    let o = t as *mut GCObject;

    // in the second propagation stage, table assignment barrier works as a forward barrier
    if (*g).gcstate as i32 == GCSPROPAGATEAGAIN {
      LUAU_ASSERT!(isblack!(o) && iswhite!(v) && !isdead!(g, v) && !isdead!(g, o));
      reallymarkobject(g, v);
      return;
    }

    LUAU_ASSERT!(isblack!(o) && !isdead!(g, o));
    LUAU_ASSERT!((*g).gcstate as i32 != GCSPAUSE);
    black2gray!(o); // make table gray (again)
    (*t).gclist = (*g).grayagain;
    (*g).grayagain = o;
  }
}

pub use lua_c_barriertable as luaC_barriertable;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaC_barriertable")]
pub unsafe extern "C-unwind" fn lua_c_barriertable_export(
  l: *mut lua_State,
  t: *mut c_void,
  v: *mut c_void,
) {
  unsafe {
    lua_c_barriertable(l, t as *mut LuaTable, v as *mut GCObject);
  }
}
