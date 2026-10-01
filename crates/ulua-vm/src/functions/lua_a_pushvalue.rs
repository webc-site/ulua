use crate::{
  functions::ensure_stack::ensure_stack, macros::api_incr_top::api_incr_top,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// 按值压入 TValue（`luaA_pushvalue`）：`l` 以引用传入（存活由类型保证，`ensure_stack`
/// 可能重分配栈并移动 `top`，故扩容后再取 `top` 槽）；`o` 以引用传入，按值拷贝到
/// top 槽。cpp `lapi.cpp:143`。
pub fn lua_a_pushvalue(l: &mut LuaState, o: &TValue) {
  // SAFETY: `l` 存活（引用形保证）；`ensure_stack(l,1)` 保证 top 槽在分配栈界内
  // 可写，`api_incr_top!` 的其余前提由该扩容成立。
  unsafe {
    ensure_stack(l.as_mut_ptr(), 1);
    *l.top = *o;
    api_incr_top!(l);
  }
}
