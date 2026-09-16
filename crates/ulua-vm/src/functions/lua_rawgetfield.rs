use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_h_getstr::lua_h_getstr,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, hvalue::hvalue, lua_s_new::luaS_new,
    setobj_2_s::setobj2s, setsvalue::setsvalue, tsvalue::tsvalue, ttistable::ttistable,
    ttype::ttype,
  },
  records::lua_state::lua_State,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawgetfield(l: *mut lua_State, idx: c_int, k: *const c_char) -> c_int {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));

    let mut key = TValue::default();
    setsvalue!(l, &mut key, luaS_new(l, k));
    setobj2s!(
      l,
      (*l).top,
      lua_h_getstr(hvalue!(t), tsvalue!(&key) as *mut _)
    );
    api_incr_top!(l);

    ttype!((*l).top.sub(1))
  }
}
