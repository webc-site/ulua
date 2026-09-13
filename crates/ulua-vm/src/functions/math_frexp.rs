use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber, lua_pushinteger::lua_pushinteger,
    lua_pushnumber::lua_pushnumber,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_frexp")]
pub(crate) unsafe extern "C-unwind" fn math_frexp(l: *mut lua_State) -> i32 {
  unsafe {
    let mut e: i32 = 0;
    let x = lua_l_checknumber(l, 1);
    let m = frexp(x, &mut e);
    lua_pushnumber(l, m);
    lua_pushinteger(l, e);
    2
  }
}

fn frexp(x: f64, exp: &mut c_int) -> f64 {
  if !x.is_finite() || x == 0.0 {
    *exp = 0;
    return x;
  }
  let bits = x.to_bits();
  let mut exponent = ((bits >> 52) & 0x7ff) as i32;
  let mut mantissa_bits = bits & 0xfffffffffffff;

  if exponent == 0 {
    // Subnormal
    let x_norm = x * 18014398509481984.0; // 2^54
    let bits_norm = x_norm.to_bits();
    exponent = (((bits_norm >> 52) & 0x7ff) as i32) - 54;
    mantissa_bits = bits_norm & 0xfffffffffffff;
  }

  *exp = (exponent - 1022) as c_int;

  let sign_bit = bits & (1 << 63);
  let res_bits = sign_bit | (0x3fe << 52) | mantissa_bits;
  f64::from_bits(res_bits)
}
