use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setnvalue::setnvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`（`ensure_stack(l,1)` 可能重分配栈并移动 `(*l).top`）；把 `n` 写入
/// `(*l).top` 所指槽——该槽须在栈数组内且已预留。cpp `lapi.cpp:690`。
pub unsafe fn lua_pushnumber(l: *mut LuaState, n: f64) {
  unsafe {
    ensure_stack(l, 1);
    setnvalue!((*l).top, n);
    api_incr_top!(l);
  }
}
