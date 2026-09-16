use core::ffi::c_int;

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_h_get::lua_h_get,
  },
  macros::{
    api_check::api_check, hvalue::hvalue, setobj_2_s::setobj2s, ttistable::ttistable, ttype::ttype,
  },
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawget(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));

    let slot = (*l).top.sub(1);
    setobj2s!(l, slot, lua_h_get(hvalue!(t), slot));

    ttype!((*l).top.sub(1))
  }
}
