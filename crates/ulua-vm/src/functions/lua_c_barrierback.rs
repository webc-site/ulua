use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::gc_object::GCObject, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_barrierback(l: *mut lua_State, o: *mut GCObject, gclist: *mut *mut GCObject) {
  unsafe {
    let g = (*l).global;

    // isblack(o) is ((*o).gch.marked & 4) != 0
    // isdead(g, o) is ((*o).gch.marked & 11) == (((*g).currentwhite ^ 3) & 3)
    let is_black = ((*o).gch.marked & 4) != 0;
    let is_dead = ((*o).gch.marked & 11) == (((*g).currentwhite ^ 3) & 3);

    LUAU_ASSERT!(is_black && !is_dead);
    LUAU_ASSERT!((*g).gcstate as i32 != 0); // GCSPAUSE is 0

    // black2gray(o) clears the BLACKBIT (bit 2, mask 4)
    (*o).gch.marked &= !4;

    *gclist = (*g).grayagain;
    (*g).grayagain = o;
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaC_barrierback")]
pub unsafe extern "C-unwind" fn lua_c_barrierback_export(
  l: *mut lua_State,
  o: *mut c_void,
  gclist: *mut *mut c_void,
) {
  unsafe {
    lua_c_barrierback(l, o as *mut GCObject, gclist as *mut *mut GCObject);
  }
}
