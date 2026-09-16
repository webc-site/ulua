use core::ffi::{c_int, c_void};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_h_getp::lua_h_getp,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, hvalue::hvalue, setobj_2_s::setobj2s,
    ttistable::ttistable, ttype::ttype,
  },
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawgetptagged(
  l: *mut lua_State,
  idx: c_int,
  p: *mut c_void,
  tag: c_int,
) -> c_int {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));

    setobj2s!(l, (*l).top, lua_h_getp(hvalue!(t), p, tag));
    api_incr_top!(l);

    ttype!((*l).top.sub(1))
  }
}
