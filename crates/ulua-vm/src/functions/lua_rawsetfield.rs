use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_g_readonlyerror::lua_g_readonlyerror, lua_h_setstr::lua_h_setstr,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, hvalue::hvalue,
    lua_c_barriert::luaC_barriert, lua_s_new::luaS_new, setobj_2_t::setobj2t, ttistable::ttistable,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawsetfield(l: *mut lua_State, idx: c_int, k: *const c_char) {
  unsafe {
    api_checknelems!(l, 1);
    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));
    if (*hvalue!(t)).readonly != 0 {
      lua_g_readonlyerror(l);
    }
    setobj2t!(
      l,
      lua_h_setstr(l, hvalue!(t), luaS_new(l, k)),
      (*l).top.offset(-1)
    );
    luaC_barriert!(l, hvalue!(t), (*l).top.offset(-1));
    (*l).top = (*l).top.offset(-1);
  }
}
