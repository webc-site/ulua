use core::ffi::c_int;

use crate::{
  functions::{
    index_2_addr::index2addr, lua_g_readonlyerror::lua_g_readonlyerror, lua_h_clear::lua_h_clear,
  },
  macros::{api_check::api_check, hvalue::hvalue, ttistable::ttistable},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_cleartable(l: *mut lua_State, idx: c_int) {
  unsafe {
    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));
    let tt = hvalue!(t);
    if (*tt).readonly != 0 {
      lua_g_readonlyerror(l);
    }
    lua_h_clear(tt);
  }
}
