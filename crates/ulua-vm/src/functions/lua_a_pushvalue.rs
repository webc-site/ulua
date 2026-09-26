use crate::{
  functions::ensure_stack::ensure_stack, macros::api_incr_top::api_incr_top,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState`（`ensure_stack` 可能重分配栈并移动 `(*l).top`）；`o` 须为指向存活
/// `TValue` 的非空可读指针（本函数按值拷贝 `*o` 到 `(*l).top`）。cpp `lapi.cpp:143`。
pub unsafe fn lua_a_pushvalue(l: *mut LuaState, o: *const TValue) {
  unsafe {
    ensure_stack(l, 1);
    *(*l).top = *o;
    api_incr_top!(l);
  }
}
