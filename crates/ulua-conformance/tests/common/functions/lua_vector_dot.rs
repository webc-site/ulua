use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_l_checkvector::lua_l_checkvector, lua_pushnumber::lua_pushnumber},
  records::lua_state::lua_State,
};

/// C-unwind ABI：作为 LuaCfunction 注册进 VM，避免 transmute。
pub(crate) extern "C-unwind" fn lua_vector_dot(l: *mut lua_State) -> c_int {
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_checkvector(l, 2);

    let result = (*a.add(0)) * (*b.add(0)) + (*a.add(1)) * (*b.add(1)) + (*a.add(2)) * (*b.add(2));
    lua_pushnumber(l, result as f64);
  }
  1
}
