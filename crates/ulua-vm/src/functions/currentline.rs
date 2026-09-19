use core::ptr::addr_of;

use crate::{
  functions::{currentpc::currentpc, lua_g_getline::luaG_getline},
  macros::ci_func::ci_func,
  records::{call_info::CallInfo, closure::LClosure},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn currentline(_l: *mut lua_State, ci: *mut CallInfo) -> i32 {
  unsafe {
    let cl = ci_func!(ci);
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    luaG_getline((*lcl).p, currentpc(_l, ci))
  }
}
