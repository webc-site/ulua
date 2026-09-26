use core::{ffi::c_void, ptr::null};

use crate::{
  functions::{
    lua_checkstack::lua_checkstack, lua_d_call::lua_d_call, lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::{lua_g_runerror::lua_g_runerror, lua_pushlightuserdata::lua_pushlightuserdata},
  records::{c_call_s::CCallS, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn f_ccall(l: *mut LuaState, ud: *mut c_void) {
  unsafe {
    let c = ud as *mut CCallS;

    if lua_checkstack(l, 2) == 0 {
      lua_g_runerror!(l, "stack limit");
    }

    lua_pushcclosurek(l, (*c).func, null(), 0, None);
    lua_pushlightuserdata(l, (*c).ud);
    lua_d_call(l, (*l).top.sub(2), 0);
  }
}
