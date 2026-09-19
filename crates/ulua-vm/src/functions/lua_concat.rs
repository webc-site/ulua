use core::ffi::c_int;

use crate::{
  functions::{
    ensure_stack::ensure_stack, lua_c_barrierback::lua_c_barrierback, lua_s_newlstr::luaS_newlstr,
    lua_v_concat::lua_v_concat,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    cast_int::cast_int, isblack::isblack, lua_c_check_gc::luaC_checkGC, obj_2_gco::obj2gco,
    setsvalue::setsvalue,
  },
  records::lua_state::lua_State,
};

/// cpp `lgc.h:115` 的 `luaC_threadbarrier(L)` 宏；本端口的唯一实现，
/// `lua_getinfo` / `lua_getlocal` 与全部 lapi 推栈函数共用。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_c_threadbarrier_lapi(l: *mut lua_State) {
  unsafe {
    let obj = obj2gco!(l);
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
      // cpp `ensure_stack(L, 1)` 只在 n == 0 分支；n >= 2 由 luaV_concat 自行管栈
      ensure_stack(l, 1);
      setsvalue!(l, (*l).top, luaS_newlstr(l, c"".as_ptr(), 0));
      api_incr_top!(l);
    }
  }
}
