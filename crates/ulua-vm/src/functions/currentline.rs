use core::ptr::addr_of;

use crate::{
  functions::{currentpc::currentpc, lua_g_getline::luaG_getline},
  macros::ci_func::ci_func,
  records::closure::LClosure,
  type_aliases::{call_info::CallInfo, lua_state::lua_State},
};

pub(crate) unsafe fn currentline(_l: *mut lua_State, ci: *mut CallInfo) -> i32 {
  unsafe {
    let cl = ci_func!(ci);
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    luaG_getline((*lcl).p, currentpc(_l, ci))
  }
}
