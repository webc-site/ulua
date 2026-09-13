use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  functions::{currentpc::currentpc, getluaproto::get_lua_proto, lua_f_getlocal::luaF_getlocal},
  macros::{
    api_check::api_check, getstr::getstr, lua_callinfo_native::LUA_CALLINFO_NATIVE,
    setobj_2_s::setobj_2_s,
  },
  records::{call_info::CallInfo, loc_var::LocVar, lua_state::lua_State, proto::Proto},
};

#[unsafe(export_name = "ulua_lua_setlocal")]
pub(crate) unsafe fn lua_setlocal(l: *mut lua_State, level: c_int, n: c_int) -> *const c_char {
  unsafe {
    api_check!(l, (*l).top.offset_from((*l).base) >= 1);

    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return null();
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));

    // changing registers in native functions externally may invalidate safety contracts wrt register type tags
    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      return null();
    }

    let fp: *mut Proto = get_lua_proto(ci);
    let var: *const LocVar = if !fp.is_null() {
      luaF_getlocal(fp, n, currentpc(l, ci))
    } else {
      null()
    };

    if !var.is_null() {
      setobj_2_s!(
        l,
        (*ci).base.offset((*var).reg as isize),
        (*l).top.offset(-1)
      );
    }

    (*l).top = (*l).top.offset(-1); // pop value

    if !var.is_null() {
      getstr((*var).varname)
    } else {
      null()
    }
  }
}
