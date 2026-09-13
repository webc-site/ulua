use core::ffi::{CStr, c_int};

use ulua_vm::{
  functions::{
    lua_l_checkvector::lua_l_checkvector, lua_l_error_l::lua_l_error_l,
    lua_pushcclosurek::lua_pushcclosurek, lua_pushnumber::lua_pushnumber,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::{lua_l_checkstring::luaL_checkstring, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::lua_State,
};

use crate::common::functions::lua_vector_dot::lua_vector_dot;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vector_index(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);
    let name_ptr = luaL_checkstring!(l, 2);
    let name = CStr::from_ptr(name_ptr).to_str().unwrap_or("");

    if name == "Magnitude" {
      let mut sum = *v.add(0) * *v.add(0) + *v.add(1) * *v.add(1) + *v.add(2) * *v.add(2);
      if LUA_VECTOR_SIZE == 4 {
        sum += *v.add(3) * *v.add(3);
      }
      lua_pushnumber(l, sum.sqrt() as f64);
      return 1;
    }

    if name == "Unit" {
      let mut sum = *v.add(0) * *v.add(0) + *v.add(1) * *v.add(1) + *v.add(2) * *v.add(2);
      if LUA_VECTOR_SIZE == 4 {
        sum += *v.add(3) * *v.add(3);
      }
      let inv_sqrt = 1.0 / sum.sqrt();

      if LUA_VECTOR_SIZE == 4 {
        lua_pushvector_lua_state_f32_f32_f32_f32(
          l,
          *v.add(0) * inv_sqrt,
          *v.add(1) * inv_sqrt,
          *v.add(2) * inv_sqrt,
          *v.add(3) * inv_sqrt,
        );
      } else {
        lua_pushvector_lua_state_f32_f32_f32(
          l,
          *v.add(0) * inv_sqrt,
          *v.add(1) * inv_sqrt,
          *v.add(2) * inv_sqrt,
        );
      }
      return 1;
    }

    if name == "Dot" {
      lua_pushcclosurek(l, Some(lua_vector_dot), c"Dot".as_ptr(), 0, None);
      return 1;
    }

    lua_l_error_l(
      l,
      c"%s is not a valid member of vector".as_ptr(),
      format_args!("{name} is not a valid member of vector"),
    );
    0
  }
}
