use core::{ptr::null, slice::from_raw_parts};

use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector, lua_l_optvector::lua_l_optvector,
    lua_pushnumber::lua_pushnumber,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_vector_angle")]
pub(crate) unsafe extern "C-unwind" fn vector_angle(l: *mut lua_State) -> i32 {
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_optvector(l, 2, null());
    let axis = lua_l_optvector(l, 3, null());

    let a_val = from_raw_parts(a, 3);
    let b_val = from_raw_parts(b, 3);

    let cross = [
      a_val[1] * b_val[2] - a_val[2] * b_val[1],
      a_val[2] * b_val[0] - a_val[0] * b_val[2],
      a_val[0] * b_val[1] - a_val[1] * b_val[0],
    ];

    let sin_a = ((cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]) as f64).sqrt();
    let cos_a = (a_val[0] * b_val[0] + a_val[1] * b_val[1] + a_val[2] * b_val[2]) as f64;
    let mut angle = sin_a.atan2(cos_a);

    if !axis.is_null() {
      let axis_val = from_raw_parts(axis, 3);
      if cross[0] * axis_val[0] + cross[1] * axis_val[1] + cross[2] * axis_val[2] < 0.0 {
        angle = -angle;
      }
    }

    lua_pushnumber(l, angle);
    1
  }
}
