use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkvector::lua_l_checkvector, lua_pushnumber::lua_pushnumber},
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_magnitude(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushnumber(
        l,
        ((*v.offset(0)) * (*v.offset(0))
          + (*v.offset(1)) * (*v.offset(1))
          + (*v.offset(2)) * (*v.offset(2))
          + (*v.offset(3)) * (*v.offset(3)))
        .sqrt() as f64,
      );
    } else {
      lua_pushnumber(
        l,
        ((*v.offset(0)) * (*v.offset(0))
          + (*v.offset(1)) * (*v.offset(1))
          + (*v.offset(2)) * (*v.offset(2)))
        .sqrt() as f64,
      );
    }

    1
  }
}
