use core::{ffi::c_void, ptr::null};

use crate::{
  functions::{
    lua_checkstack::lua_checkstack, lua_d_call::lua_d_call, lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::{
    cast_to::cast_to, lua_g_runerror::lua_g_runerror, lua_pushlightuserdata::lua_pushlightuserdata,
  },
  records::c_call_s::CCallS,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn f_ccall(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let c = cast_to!(*mut CCallS, ud);

    if lua_checkstack(l, 2) == 0 {
      lua_g_runerror!(l, "stack limit");
    }

    lua_pushcclosurek(l, (*c).func, null(), 0, None);
    lua_pushlightuserdata(l as *mut c_void, (*c).ud);
    lua_d_call(l, (*l).top.sub(2), 0);
  }
}
