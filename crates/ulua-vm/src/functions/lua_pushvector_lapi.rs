use crate::{
  functions::ensure_stack::ensure_stack,
  macros::api_incr_top::api_incr_top,
  records::{lua_state::LuaState, slot::Slot},
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
  // SAFETY: 契约保证 `l` 为存活调用帧；本块是全函数唯一指针边界（setvector_lapi 模板）——
  // `ensure_stack(l, 1)`（cpp `LUA_VECTOR_DOUBLE == 0` 分支，无条件编译的 barrier）预留可写
  // 槽后，`Slot::from_raw((*l).top)` 把 top 收成槽句柄，`set_vvalue` 依序写 lane0/1/2、
  // 第 4 lane 仅在 `LUA_VECTOR_SIZE == 4` 编译期门内写（且仅门内求值），末置 tag，与收口前
  // `setvvalue!((*l).top, x, y, z, w)` 宏体逐位一致（`w` 以 `FnOnce` 惰性传入：3-lane 构建
  // 下第 4 分量无槽位、连求值都不做，`macros/setvvalue.rs` 文件头的越界防御 rationale 原样
  // 适用）；`api_incr_top!` 前提（top < ci->top）随扩容成立。
  unsafe {
    ensure_stack(l, 1);
    Slot::from_raw((*l).top).as_mut().set_vvalue(x, y, z, || w);
    api_incr_top!(l);
  }
}

/// # Safety
/// `l` 指向处于可正常压栈状态的存活 `LuaState`（cpp lapi.cpp:731 三分量重载，w 补 0）；栈槽由
/// 内部 `ensure_stack` 扩容，调用方无需预留，但返回后不得再复用此前缓存的栈指针（realloc 悬挂）。
pub unsafe fn lua_pushvector_lua_state_f32_f32_f32(l: *mut LuaState, x: f32, y: f32, z: f32) {
  // SAFETY: 契约保证 `l` 存活；本块是全函数唯一指针边界（setvector_lapi 模板）：
  // `ensure_stack` 后 top 槽独占可写，`Slot` 句柄上 `set_vvalue` 写面与收口前
  // `setvvalue!((*l).top, x, y, z, 0.0f32)` 宏体逐位一致，第 4 分量实参与旧宏体同为常量
  // `0.0`、且仍按方法契约只在 4-lane 编译期门内求值（3-lane 构建下 lane3 不写、常量亦
  // 不落盘）；`api_incr_top!` 前提随扩容成立。
  unsafe {
    ensure_stack(l, 1);
    Slot::from_raw((*l).top)
      .as_mut()
      .set_vvalue(x, y, z, || 0.0f32);
    api_incr_top!(l);
  }
}
