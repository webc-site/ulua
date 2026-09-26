use ulua_common::fflag::FixMathNoisePrecision;

use crate::{
  functions::{lua_pushnumber::lua_pushnumber, lua_tonumberx::lua_tonumberx, perlin::perlin},
  macros::{
    lua_isnoneornil::lua_isnoneornil, lua_l_argexpected::luaL_argexpected, lua_lib_fn::lua_lib_fn,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1..=3 号位经 `lua_tonumberx` 取数字并回报可否转换，
/// `luaL_argexpected`/`lua_isnoneornil` 校验（1 必为数字，2/3 可缺省为 nil），`lua_pushnumber` 写回可分配/GC。
/// cpp/VM/src/lmathlib.cpp:369 math_noise。
pub unsafe fn math_noise(l: *mut LuaState) -> i32 {
  unsafe {
    let x = lua_tonumberx(l, 1);
    let y = lua_tonumberx(l, 2);
    let z = lua_tonumberx(l, 3);

    luaL_argexpected!(l, x.is_some(), 1, "number");
    luaL_argexpected!(l, y.is_some() || lua_isnoneornil!(l, 2), 2, "number");
    luaL_argexpected!(l, z.is_some() || lua_isnoneornil!(l, 3), 3, "number");

    // 非数值（仅 1 号位可能，2/3 缺省时）按 cpp 失败路径取 0.0
    let x = x.unwrap_or(0.0);
    let y = y.unwrap_or(0.0);
    let z = z.unwrap_or(0.0);

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

lua_lib_fn!(pub fn math_noise, math_noise_arm);
