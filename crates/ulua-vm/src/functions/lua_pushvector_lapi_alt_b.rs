use crate::{
  macros::{api_incr_top::api_incr_top, setvvalue::setvvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_pushvector_lua_state_f32_f32_f32")]
pub unsafe fn lua_pushvector_lua_state_f32_f32_f32(l: *mut lua_State, x: f32, y: f32, z: f32) {
  unsafe {
    setvvalue!((*l).top, x, y, z, 0.0f32);
    api_incr_top!(l);
  }
}
