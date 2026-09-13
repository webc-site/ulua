use core::ffi::{c_char, c_int, c_void};

use ulua_common::FFlag;

use crate::{
  functions::{lua_h_new::lua_h_new, lua_h_setstr::lua_h_setstr},
  macros::{
    api_check::api_check, fixedbit::FIXEDBIT, l_setbit::l_setbit, lua_s_new::luaS_new,
    lua_utag_limit::LUA_UTAG_LIMIT, setpvalue::setpvalue,
  },
  records::{global_state::global_State, lua_state::lua_State, t_string::tstring},
  type_aliases::{lua_userdata_direct_field_get::LuaUserdataDirectFieldGet, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_registeruserdatadirectfieldget(
  l: *mut lua_State,
  tag: c_int,
  field: *const c_char,
  fn_: LuaUserdataDirectFieldGet,
) {
  unsafe {
    if !FFlag::LuauDirectFieldGet.get() {
      return;
    }

    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    api_check!(l, !field.is_null());
    api_check!(l, fn_.is_some());

    let g: *mut global_State = (*l).global;

    if (*g).udatadirectfields[tag as usize].is_null() {
      (*g).udatadirectfields[tag as usize] = lua_h_new(l, 0, 1);
    }

    let ts: *mut tstring = luaS_new(l, field);
    l_setbit!((*ts).hdr.marked, FIXEDBIT);

    let slot: *mut TValue = lua_h_setstr(l, (*g).udatadirectfields[tag as usize], ts);
    // cpp release 对空 fn 存 nullptr；unwrap_or 消除 panic 路径且行为一致
    setpvalue!(slot, fn_.unwrap_or(null_mut()) as *mut c_void, 0);
  }
}
