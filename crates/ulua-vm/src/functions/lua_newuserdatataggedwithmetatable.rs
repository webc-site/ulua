use core::ffi::{c_int, c_void};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_u_newudata::lua_u_newudata},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, checkliveness::checkliveness,
    isblack::isblack, lua_c_check_gc::luaC_checkGC, lua_utag_limit::LUA_UTAG_LIMIT,
  },
  records::{gc_object::GCObject, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_newuserdatataggedwithmetatable(
  l: *mut lua_State,
  sz: usize,
  tag: c_int,
) -> *mut c_void {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);

    let u = lua_u_newudata(l, sz, tag);

    ulua_common::LUAU_ASSERT!(!isblack!(u as *mut GCObject));

    let h = (*(*l).global).udatamt[tag as usize];
    api_check!(l, !h.is_null());

    (*u).metatable = h;

    (*(*l).top).value.gc = u as *mut GCObject;
    (*(*l).top).tt = LuaType::UserData as c_int;
    checkliveness!((*l).global, (*l).top);
    api_incr_top!(l);

    (*u).data.as_mut_ptr().cast()
  }
}
