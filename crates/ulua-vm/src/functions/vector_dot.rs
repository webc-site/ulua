use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkvector::lua_l_checkvector, lua_pushnumber::lua_pushnumber},
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_dot(l: *mut lua_State) -> c_int {
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_checkvector(l, 2);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushnumber(
        l,
        ((*a.offset(0)) * (*b.offset(0))
          + (*a.offset(1)) * (*b.offset(1))
          + (*a.offset(2)) * (*b.offset(2))
          + (*a.offset(3)) * (*b.offset(3))) as f64,
      );
    } else {
      lua_pushnumber(
        l,
        ((*a.offset(0)) * (*b.offset(0))
          + (*a.offset(1)) * (*b.offset(1))
          + (*a.offset(2)) * (*b.offset(2))) as f64,
      );
    }

    1
  }
}
