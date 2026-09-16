use ulua_vm::{
  functions::lua_touserdatatagged::lua_touserdatatagged, macros::lua_l_typeerror::luaL_typeerror,
  records::lua_state::lua_State,
};

use crate::common::records::vec_2_conformance_ir_hooks::Vec2;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_vec_2_get(l: *mut lua_State, idx: i32) -> *mut Vec2 {
  unsafe {
    let a = lua_touserdatatagged(l, idx, 12) as *mut Vec2;

    if !a.is_null() {
      return a;
    }

    luaL_typeerror!(l, idx, "vec2");
  }
}
