use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  records::lua_state::LuaState,
};

/// IEEE-754 double 的符号位掩码。
const SIGN_BIT_MASK: u64 = 1 << 63;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn math_modf(l: *mut LuaState) -> i32 {
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
///
/// `luauF_modf`（`lbuiltins.cpp:330` 快速调用面）与本函数共用这一份 C 语义。
pub(crate) fn modf(x: f64) -> (f64, f64) {
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

// §9.3：本文件的 #[cfg(test)] 模块已删除——`modf` 的全部边界语义（±inf→±0 分数、
// NaN 双 NaN、±0 保号、整数值/|x|≥2^52 截断）与 `luau_f_modf` 共用同一份实现
// （见其函数体直调 `math_modf::modf`），逐条断言已由 `tests/fastcall_modf_builtin.rs`
// 经公开快速调用面覆盖：modf_writes_integer_then_fraction（±3.5/0.5 分数保号）、
// modf_of_infinite_keeps_signed_zero_fraction、modf_of_nan_yields_nan_pair、
// modf_of_zero_keeps_sign、modf_of_integral_and_wide_values_is_exact。
