use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setvvalue::setvvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 指向处于可正常压栈状态的存活 `LuaState`（cpp lapi.cpp:719 四分量重载）；栈槽由内部
/// `ensure_stack` 扩容，调用方无需预留，但返回后不得再复用此前缓存的栈指针（realloc 使 `top` 悬挂）。
pub unsafe fn lua_pushvector_lua_state_f32_f32_f32_f32(
  l: *mut LuaState,
  x: f32,
  y: f32,
  z: f32,
  w: f32,
) {
  // Safety: 契约保证 `l` 为存活调用帧且栈顶预留 1 可写槽，四个 f32 分量经 setvvalue! 写入同一 TValue payload 界内
  unsafe {
    // cpp `ensure_stack(L, 1)`（本端口是 LUA_VECTOR_DOUBLE == 0 分支，无条件编译的 barrier）
    ensure_stack(l, 1);
    setvvalue!((*l).top, x, y, z, w);
    api_incr_top!(l);
  }
}

/// # Safety
/// `l` 指向处于可正常压栈状态的存活 `LuaState`（cpp lapi.cpp:731 三分量重载，w 补 0）；栈槽由
/// 内部 `ensure_stack` 扩容，调用方无需预留，但返回后不得再复用此前缓存的栈指针（realloc 悬挂）。
pub unsafe fn lua_pushvector_lua_state_f32_f32_f32(l: *mut LuaState, x: f32, y: f32, z: f32) {
  // Safety: 契约保证 `l` 存活且 ensure_stack 后栈顶 1 槽可写，setvvalue! 写入同槽 payload 界内
  unsafe {
    ensure_stack(l, 1);
    setvvalue!((*l).top, x, y, z, 0.0f32);
    api_incr_top!(l);
  }
}
