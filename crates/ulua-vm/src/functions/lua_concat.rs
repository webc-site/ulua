use core::ffi::c_int;

use crate::{
  functions::{
    lua_c_barrierback::lua_c_barrierback, lua_s_newlstr::luaS_newlstr, lua_v_concat::lua_v_concat,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    cast_int::cast_int, isblack::isblack, lua_c_check_gc::luaC_checkGC, setsvalue::setsvalue,
  },
  records::{gc_object::GCObject, lua_state::lua_State},
};

pub(crate) unsafe fn lua_c_threadbarrier_lapi(l: *mut lua_State) {
  unsafe {
    let obj = l as *mut GCObject;
    if isblack!(obj) {
      lua_c_barrierback(l, obj, &mut (*l).gclist);
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_concat(l: *mut lua_State, n: c_int) {
  unsafe {
    api_check!(l, n >= 0);
    api_checknelems!(l, n);

    if n >= 2 {
      luaC_checkGC!(l);
      lua_c_threadbarrier_lapi(l);
      lua_v_concat(l, n, cast_int!((*l).top.offset_from((*l).base)) - 1);
      (*l).top = (*l).top.sub((n - 1) as usize);
    } else if n == 0 {
      lua_c_threadbarrier_lapi(l);
      setsvalue!(l, (*l).top, luaS_newlstr(l, c"".as_ptr(), 0));
      api_incr_top!(l);
    }
  }
}
