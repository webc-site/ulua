use core::ffi::c_void;

use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_lutag_limit::LUA_LUTAG_LIMIT,
    setpvalue::setpvalue,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub unsafe fn lua_pushlightuserdatatagged(l: *mut LuaState, p: *mut c_void, tag: i32) {
  // Safety: 契约保证 `l` 为存活调用帧且栈顶预留 1 槽，写入 lightuserdata 载荷与 tag 不越帧界
  unsafe {
    // cpp `ensure_stack(L, 1)` 在 api_check 之前
    ensure_stack(l, 1);
    api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);
    setpvalue!((*l).top, p, tag);
    api_incr_top!(l);
  }
}
