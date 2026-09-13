use core::ffi::c_int;

use crate::{
  functions::{
    getluaproto::get_lua_proto, lua_a_pushvalue::luaA_pushvalue,
    lua_c_barrierback::lua_c_barrierback,
  },
  macros::lua_callinfo_native::LUA_CALLINFO_NATIVE,
  records::{call_info::CallInfo, lua_state::lua_State, proto::Proto},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_getargument")]
pub unsafe fn lua_getargument(l: *mut lua_State, level: c_int, n: c_int) -> c_int {
  unsafe {
    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return 0;
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));

    // changing tables in native functions externally may invalidate safety contracts wrt table state (metatable/size/readonly)
    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      return 0;
    }

    let fp: *mut Proto = get_lua_proto(ci);
    let mut res: i32 = 0;

    if !fp.is_null() && n > 0 {
      if (n as u32) <= (*fp).numparams as u32 {
        if ((*l).hdr.marked & 4) != 0 {
          lua_c_barrierback(l, l as *mut _, &mut (*l).gclist);
        }
        luaA_pushvalue(l, (*ci).base.offset((n - 1) as isize));
        res = 1;
      } else if (*fp).is_vararg != 0 && (n as isize) < (*ci).base.offset_from((*ci).func) {
        if ((*l).hdr.marked & 4) != 0 {
          lua_c_barrierback(l, l as *mut _, &mut (*l).gclist);
        }
        luaA_pushvalue(l, (*ci).func.offset(n as isize));
        res = 1;
      }
    }

    res
  }
}
