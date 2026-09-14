use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  functions::{
    currentpc::currentpc, getluaproto::get_lua_proto, lua_a_pushvalue::luaA_pushvalue,
    lua_f_getlocal::luaF_getlocal,
  },
  macros::{
    getstr::getstr, lua_c_threadbarrier::luaC_threadbarrier,
    lua_callinfo_native::LUA_CALLINFO_NATIVE,
  },
  records::{call_info::CallInfo, loc_var::LocVar, lua_state::lua_State},
  type_aliases::proto::Proto,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_getlocal")]
pub unsafe fn lua_getlocal(l: *mut lua_State, level: c_int, n: c_int) -> *const c_char {
  unsafe {
    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return null();
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));

    // changing tables in native functions externally may invalidate safety contracts wrt table state (metatable/size/readonly)
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
      luaC_threadbarrier!(l);
      luaA_pushvalue(l, (*ci).base.offset((*var).reg as isize));
    }

    if !var.is_null() {
      getstr((*var).varname)
    } else {
      null()
    }
  }
}
