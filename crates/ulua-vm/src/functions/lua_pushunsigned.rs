use core::ffi::c_uint;

use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setnvalue::setnvalue},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe fn lua_pushunsigned(l: *mut LuaState, u: c_uint) {
  // Safety: 契约保证 `l` 为存活调用帧且栈顶预留 1 可写槽，写入 u64 数值 TValue 不越帧界
  unsafe {
    ensure_stack(l, 1);
    setnvalue!((*l).top, u as f64);
    api_incr_top!(l);
  }
}
