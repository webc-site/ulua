use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setbvalue::setbvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`（`ensure_stack(l,1)` 可能重分配栈并移动 `(*l).top`）；把 `b!=0` 写入
/// `(*l).top` 所指槽——该槽须在栈数组内且已预留。cpp `lapi.cpp:802`。
pub unsafe fn lua_pushboolean(l: *mut LuaState, b: i32) {
  unsafe {
    ensure_stack(l, 1);
    // The setbvalue macro requires TValue and lua_Type to be in scope at the call site.
    setbvalue!((*l).top, b != 0); // ensure that true is 1
    api_incr_top!(l);
  }
}
