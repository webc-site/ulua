use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber, lua_pushinteger::lua_pushinteger,
    lua_pushnumber::lua_pushnumber,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 次正规数规格化所用的倍率 2^54（= 1 << (52 + 2)，52 为 double 尾数位宽）。
const SUBNORMAL_SCALE: f64 = 18_014_398_509_481_984.0;
/// 与 [`SUBNORMAL_SCALE`] 对应的指数回退量。
const SUBNORMAL_EXPONENT_ADJUST: i32 = 54;

/// # Safety
/// `l` 须为存活 LuaState 并处于本 C 函数受保护帧：栈 1 号位经 `lua_l_checknumber` 强制为数字（非数字抛错），
/// 随后 `lua_pushnumber`/`lua_pushinteger` 各写一槽（返回两值）。cpp/VM/src/lmathlib.cpp:190 math_frexp。
pub unsafe fn math_frexp(l: *mut LuaState) -> i32 {
  unsafe {
    let (m, e) = frexp(lua_l_checknumber(l, 1));
    lua_pushnumber(l, m);
    lua_pushinteger(l, e);
    2
  }
}

/// libc `frexp(x, &e)` 的等价实现：返回 `(m, e)`，满足 `x == m * 2^e`、
/// `|m| ∈ [0.5, 1)`（0/±inf/NaN 原样返回 m 且 e = 0）。
///
/// cpp 经 `int* e` 出参写回指数，Rust 版以元组一并返回。
fn frexp(x: f64) -> (f64, i32) {
  if !x.is_finite() || x == 0.0 {
    return (x, 0);
  }

  let bits = x.to_bits();
  let mut exponent = ((bits >> 52) & 0x7ff) as i32;
  let mut mantissa_bits = bits & 0xfffffffffffff;

  if exponent == 0 {
    // Subnormal
    let bits_norm = (x * SUBNORMAL_SCALE).to_bits();
    exponent = (((bits_norm >> 52) & 0x7ff) as i32) - SUBNORMAL_EXPONENT_ADJUST;
    mantissa_bits = bits_norm & 0xfffffffffffff;
  }

  // 结果尾数：保留原符号与尾数位，指数域置 0x3fe（即 2^-1 量级，使 |m| ∈ [0.5, 1)）
  let res_bits = bits & (1 << 63) | (0x3fe << 52) | mantissa_bits;
  (f64::from_bits(res_bits), (exponent - 1022))
}

lua_lib_fn!(pub fn math_frexp, math_frexp_arm);
