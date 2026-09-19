use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

/// IEEE-754 double 的符号位掩码。
const SIGN_BIT_MASK: u64 = 1 << 63;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_math_modf"))]
pub(crate) unsafe extern "C-unwind" fn math_modf(l: *mut lua_State) -> i32 {
  unsafe {
    // C `modf(x, &ip)`：整数部分入 ip，小数部分为返回值；cpp 先推 ip 再推 fp。
    let (fp, ip) = modf(lua_l_checknumber(l, 1));
    lua_pushnumber(l, ip);
    lua_pushnumber(l, fp);
    2
  }
}

/// libc `modf(x, &ip)` 的等价实现：返回 `(fp, ip)`，满足 `x == fp + ip`，
/// `ip` 为截向零的整数部分、`fp` 与 `x` 同号。
///
/// cpp 侧 `math_modf` 直接调用 libc `modf`，`x - x.trunc()` 只在有限且非整的
/// 值上等价，故按 C 语义补齐三处边界：
///
/// 1. `modf(±inf) = (±0, ±inf)`：`inf - inf` 会得 NaN，必须特判；
/// 2. `modf(NaN) = (NaN, NaN)`：整数部分同样写回 NaN；
/// 3. 分数部分为 0 时（±0 与整数值，含 `|x| >= 2^52`）IEEE 减法只得 `+0`，
///    需还原成与 `x` 同号的 ±0。
fn modf(x: f64) -> (f64, f64) {
  if x.is_nan() {
    return (x, x);
  }
  if x.is_infinite() {
    return (signed_zero(x), x);
  }

  let ip = x.trunc();
  let fp = x - ip;
  (if fp == 0.0 { signed_zero(x) } else { fp }, ip)
}

/// 与 `x` 同号的零，对应 libc 对 `±inf`、整数值分数部分给出的 ±0。
#[inline]
fn signed_zero(x: f64) -> f64 {
  f64::from_bits(x.to_bits() & SIGN_BIT_MASK)
}

#[cfg(test)]
mod tests {
  use super::*;

  /// 断言 `v` 为带指定符号的零（`f64 == 0.0` 无法区分 ±0）。
  fn assert_signed_zero(v: f64, negative: bool) {
    assert_eq!(v, 0.0, "期望零，实际 {v}");
    assert_eq!(v.is_sign_negative(), negative, "零的符号位不符");
  }

  #[test]
  fn modf_keeps_fraction_sign_of_input() {
    assert_eq!(modf(3.5), (0.5, 3.0));
    assert_eq!(modf(-3.5), (-0.5, -3.0));
    assert_eq!(modf(0.5), (0.5, 0.0));
    assert_eq!(modf(-0.5), (-0.5, -0.0));
    assert!(!modf(0.5).1.is_sign_negative());
    assert!(modf(-0.5).1.is_sign_negative());
  }

  #[test]
  fn modf_positive_infinity_yields_plus_zero() {
    let (fp, ip) = modf(f64::INFINITY);
    assert_signed_zero(fp, false);
    assert_eq!(ip, f64::INFINITY);
  }

  #[test]
  fn modf_negative_infinity_yields_minus_zero() {
    let (fp, ip) = modf(f64::NEG_INFINITY);
    assert_signed_zero(fp, true);
    assert_eq!(ip, f64::NEG_INFINITY);
  }

  #[test]
  fn modf_nan_yields_nan_pair() {
    let (fp, ip) = modf(f64::NAN);
    assert!(fp.is_nan());
    assert!(ip.is_nan());
  }

  #[test]
  fn modf_signed_zero_yields_signed_zero() {
    let (fp, ip) = modf(-0.0);
    assert_signed_zero(fp, true);
    assert_signed_zero(ip, true);

    let (fp, ip) = modf(0.0);
    assert_signed_zero(fp, false);
    assert_signed_zero(ip, false);
  }

  #[test]
  fn modf_integral_value_keeps_zero_sign() {
    let (fp, ip) = modf(-3.0);
    assert_signed_zero(fp, true);
    assert_eq!(ip, -3.0);

    let (fp, ip) = modf(3.0);
    assert_signed_zero(fp, false);
    assert_eq!(ip, 3.0);

    // |x| >= 2^52 之后 double 全为整数，分数部分只能是 ±0
    let large = -4_503_599_627_370_496.0;
    let (fp, ip) = modf(large);
    assert_signed_zero(fp, true);
    assert_eq!(ip, large);
  }
}
