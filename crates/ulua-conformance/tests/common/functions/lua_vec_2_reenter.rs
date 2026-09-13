use ulua_vm::{
  functions::{lua_pcall::lua_pcall, lua_pushnumber::lua_pushnumber},
  macros::{lua_getglobal::lua_getglobal, lua_isfunction::lua_isfunction},
  records::lua_state::lua_State,
};

use crate::common::records::vec_2_conformance_ir_hooks::Vec2;

pub(crate) fn lua_vec_2_reenter(l: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
  unsafe {
    lua_getglobal(l, c"reenterCallback".as_ptr());
    assert!(lua_isfunction!(l, -1));
    lua_pcall(l, 0, 0, 0);

    let self_val = &*self_ptr;
    let result = (self_val.x as f64) + (self_val.y as f64);
    lua_pushnumber(l, result);
  }
  1
}
