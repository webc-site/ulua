use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_s_newlstr::lua_s_newlstr,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc,
    setsvalue::setsvalue,
  },
  records::lua_state::LuaState,
};

/// 安全字节切片版本：向栈顶推入 `s` 对应的 Lua 字符串。
///
/// # Safety
///
/// `l` 必须指向合法且存活的 `LuaState`。
pub unsafe fn lua_pushlstring_bytes(l: *mut LuaState, s: &[u8]) {
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    setsvalue!(l, (*l).top, lua_s_newlstr(&mut *l, s));
    api_incr_top!(l);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushlstring(l: *mut LuaState, s: *const c_char, len: usize) {
  unsafe {
    api_check!(l, !s.is_null());
    let slice = if len == 0 {
      &[]
    } else {
      from_raw_parts(s.cast::<u8>(), len)
    };
    lua_pushlstring_bytes(l, slice);
  }
}
