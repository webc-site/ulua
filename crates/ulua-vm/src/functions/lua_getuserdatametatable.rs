use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::lua_c_barrierback::lua_c_barrierback,
  macros::{api_check::api_check, blackbit::BLACKBIT, lua_utag_limit::LUA_UTAG_LIMIT},
  records::{gc_object::GCObject, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getuserdatametatable(l: *mut lua_State, tag: c_int) {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

  unsafe {
    let marked = (*l).hdr.marked;
    if (marked as i32 & (1 << BLACKBIT)) != 0 {
      lua_c_barrierback(l, l as *mut GCObject, &mut (*l).gclist);
    }

    let h = (*(*l).global).udatamt[tag as usize];
    if !h.is_null() {
      let i_o = (*l).top;
      (*i_o).value.gc = h as *mut GCObject;
      (*i_o).tt = LuaType::Table as c_int;
    } else {
      (*(*l).top).tt = LuaType::Nil as i32;
    }

    api_check!(l, (*l).top < (*(*l).ci).top);
    (*l).top = (*l).top.add(1);
  }
}
