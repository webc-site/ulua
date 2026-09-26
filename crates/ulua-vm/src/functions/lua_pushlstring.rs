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

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushlstring(l: *mut LuaState, s: *const c_char, len: usize) {
  unsafe {
    api_check!(l, !s.is_null());
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    // Safety: `s` 非空由上方 `api_check` 保证，len 字节可读由调用方指针契约给出；
    // capi 指针形态在此收口为切片入参。len==0 时零长切片不取指针
    // （from_raw_parts 要求非空，保持旧实现对 `(null, 0)` 的宽容）
    setsvalue!(
      l,
      (*l).top,
      lua_s_newlstr(
        l,
        if len == 0 {
          &[]
        } else {
          from_raw_parts(s.cast::<u8>(), len)
        }
      )
    );
    api_incr_top!(l);
  }
}
