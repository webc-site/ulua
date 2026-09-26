use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setnvalue::setnvalue},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub unsafe fn lua_pushinteger(l: *mut LuaState, n: i32) {
  // Safety: 契约保证 `l` 为存活调用帧且栈顶预留 1 可写槽（api_check），setnvalue!/setivalue 写 top 不越帧界
  unsafe {
    ensure_stack(l, 1);
    setnvalue!((*l).top, n as f64);
    api_incr_top!(l);
  }
}
