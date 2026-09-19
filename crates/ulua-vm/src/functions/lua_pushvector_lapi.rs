use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setvvalue::setvvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(
  feature = "capi",
  unsafe(export_name = "ulua_lua_pushvector_lua_state_f32_f32_f32_f32")
)]
pub unsafe fn lua_pushvector_lua_state_f32_f32_f32_f32(
  l: *mut lua_State,
  x: f32,
  y: f32,
  z: f32,
  w: f32,
) {
  unsafe {
    // cpp `ensure_stack(L, 1)`（本端口是 LUA_VECTOR_DOUBLE == 0 分支，无条件编译的 barrier）
    ensure_stack(l, 1);
    setvvalue!((*l).top, x, y, z, w);
    api_incr_top!(l);
  }
}
