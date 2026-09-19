use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn vector_floor(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(
        l,
        (*v.offset(0)).floor(),
        (*v.offset(1)).floor(),
        (*v.offset(2)).floor(),
        (*v.offset(3)).floor(),
      );
    } else {
      lua_pushvector_lua_state_f32_f32_f32(
        l,
        (*v.offset(0)).floor(),
        (*v.offset(1)).floor(),
        (*v.offset(2)).floor(),
      );
    }

    1
  }
}
