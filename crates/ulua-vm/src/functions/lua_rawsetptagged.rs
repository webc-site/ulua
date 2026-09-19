use core::ffi::{c_int, c_void};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_g_readonlyerror::lua_g_readonlyerror, lua_h_setp::lua_h_setp,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, hvalue::hvalue,
    lua_c_barriert::luaC_barriert, setobj_2_t::setobj2t, ttistable::ttistable,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawsetptagged(l: *mut lua_State, idx: c_int, p: *mut c_void, tag: c_int) {
  unsafe {
    api_checknelems!(l, 1);
    let o: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(o));
    if (*hvalue!(o)).readonly != 0 {
      lua_g_readonlyerror(l);
    }
    let val = (*l).top.offset(-1);
    setobj2t!(l, lua_h_setp(l, hvalue!(o), p, tag), val);
    luaC_barriert!(l, hvalue!(o), val);
    (*l).top = (*l).top.offset(-1);
  }
}
