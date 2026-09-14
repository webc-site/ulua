use core::ffi::{c_int, c_void};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_u_newudata::lua_u_newudata},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, checkliveness::checkliveness,
    lua_c_check_gc::luaC_checkGC, lua_utag_limit::LUA_UTAG_LIMIT, utag_proxy::UTAG_PROXY,
  },
  records::{gc_object::GCObject, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_newuserdatatagged(l: *mut lua_State, sz: usize, tag: c_int) -> *mut c_void {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32 || tag == UTAG_PROXY);
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);

    let u = lua_u_newudata(l, sz, tag);
    (*(*l).top).value.gc = u as *mut GCObject;
    (*(*l).top).tt = LuaType::UserData as c_int;
    checkliveness!((*l).global, (*l).top);
    api_incr_top!(l);

    (*u).data.as_mut_ptr().cast()
  }
}
