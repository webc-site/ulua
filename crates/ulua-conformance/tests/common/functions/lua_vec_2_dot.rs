use ulua_vm::{functions::lua_pushnumber::lua_pushnumber, records::lua_state::lua_State};

use crate::common::{
  functions::lua_vec_2_get::lua_vec_2_get, records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_dot(l: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
  unsafe {
    let b_ptr = lua_vec_2_get(l, 2);

    let self_val = &*self_ptr;
    let b_val = &*b_ptr;

    let result = (self_val.x as f64 * b_val.x as f64) + (self_val.y as f64 * b_val.y as f64);

    lua_pushnumber(l, result);
  }
  1
}
