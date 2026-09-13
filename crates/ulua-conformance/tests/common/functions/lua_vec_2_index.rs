use core::{ffi::CStr, mem::size_of};

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_pushnumber::lua_pushnumber},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::{
  functions::{lua_vec_2_get::lua_vec_2_get, lua_vec_2_push::lua_vec_2_push},
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_index(l: *mut lua_State) -> i32 {
  unsafe {
    let v = lua_vec_2_get(l, 1);
    let name_ptr = luaL_checkstring!(l, 2);
    let name = CStr::from_ptr(name_ptr).to_str().unwrap_or("");

    if name == "X" {
      lua_pushnumber(l, (*v).x as f64);
      return 1;
    }

    if name == "Y" {
      lua_pushnumber(l, (*v).y as f64);
      return 1;
    }

    if name == "Magnitude" {
      let mag = ((*v).x * (*v).x + (*v).y * (*v).y).sqrt();
      lua_pushnumber(l, mag as f64);
      return 1;
    }

    if name == "Unit" {
      let mag_sq = (*v).x * (*v).x + (*v).y * (*v).y;
      let inv_sqrt = 1.0 / mag_sq.sqrt();

      let data = lua_vec_2_push(l);
      (*data).x = (*v).x * inv_sqrt;
      (*data).y = (*v).y * inv_sqrt;
      return 1;
    }

    if name == "sizeof" {
      lua_pushnumber(l, size_of::<Vec2>() as f64);
      return 1;
    }

    lua_l_error_l(
      l,
      c"{} is not a valid member of vector".as_ptr(),
      core::format_args!("{} is not a valid member of vector", name),
    );
    0
  }
}
