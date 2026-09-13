use core::ffi::{c_char, c_int};

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::lua_h_getstr,
  macros::{
    api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT, svalue::svalue, ttisstring::ttisstring,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_getuserdataname(l: *mut lua_State, tag: c_int) -> *const c_char {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = (*(*l).global).udatamt[tag as usize];
    if !mt.is_null() {
      let type_ = lua_h_getstr(mt, (*(*l).global).tmname[TMS::TmType as usize]);
      if ttisstring!(type_) {
        return svalue!(type_);
      }
    }

    c"userdata".as_ptr()
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_getuserdataname")]
pub unsafe extern "C-unwind" fn lua_getuserdataname_export(
  l: *mut lua_State,
  tag: c_int,
) -> *const c_char {
  unsafe { lua_getuserdataname(l, tag) }
}
