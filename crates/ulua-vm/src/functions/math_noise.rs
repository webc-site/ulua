use ulua_common::FFlag::FixMathNoisePrecision;

use crate::{
  functions::{lua_pushnumber::lua_pushnumber, lua_tonumberx::lua_tonumberx, perlin::perlin},
  macros::{lua_isnoneornil::lua_isnoneornil, lua_l_argexpected::luaL_argexpected},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_noise")]
pub(crate) unsafe extern "C-unwind" fn math_noise(l: *mut lua_State) -> i32 {
  unsafe {
    let mut nx = 0;
    let mut ny = 0;
    let mut nz = 0;

    let x = lua_tonumberx(l, 1, &mut nx);
    let y = lua_tonumberx(l, 2, &mut ny);
    let z = lua_tonumberx(l, 3, &mut nz);

    luaL_argexpected!(l, nx != 0, 1, "number");
    luaL_argexpected!(l, ny != 0 || lua_isnoneornil!(l, 2), 2, "number");
    luaL_argexpected!(l, nz != 0 || lua_isnoneornil!(l, 3), 3, "number");

    let x = if FixMathNoisePrecision.get() {
      let x_mod = x % 256.0;
      if x_mod < 0.0 { x_mod + 256.0 } else { x_mod }
    } else {
      x
    };

    let y = if FixMathNoisePrecision.get() {
      let y_mod = y % 256.0;
      if y_mod < 0.0 { y_mod + 256.0 } else { y_mod }
    } else {
      y
    };

    let z = if FixMathNoisePrecision.get() {
      let z_mod = z % 256.0;
      if z_mod < 0.0 { z_mod + 256.0 } else { z_mod }
    } else {
      z
    };

    let r = perlin(x as f32, y as f32, z as f32);

    lua_pushnumber(l, r as f64);
    1
  }
}
