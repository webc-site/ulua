use core::ffi::c_int;

use crate::{
  functions::{currentpc::currentpc, lua_g_getline::luaG_getline},
  macros::ci_func::ci_func,
  records::closure::LClosure,
  type_aliases::{call_info::CallInfo, lua_state::lua_State},
};

pub(crate) unsafe fn currentline(_l: *mut lua_State, ci: *mut CallInfo) -> c_int {
  unsafe {
    let cl = ci_func!(ci);
    let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
    luaG_getline((*lcl).p, currentpc(_l, ci))
  }
}
