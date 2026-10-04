use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// IEEE-754 double 的符号位掩码。
const SIGN_BIT_MASK: u64 = 1 << 63;

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 取数/压栈全经安全门面，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错的受保护帧——
/// 栈 1 号位为数字（`check_number` 非数字即抛错发散），两次 `push_number` 占用 top 之上 2 个空槽。
/// cpp/VM/src/lmathlib.cpp:125 math_modf。
pub fn math_modf(l: &mut LuaState) -> i32 {
  // C `modf(x, &ip)`：整数部分入 ip，小数部分为返回值；cpp 先推 ip 再推 fp。
  let (fp, ip) = modf(l.check_number(1));
  l.push_number(ip);
  l.push_number(fp);
  2
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

lua_lib_fn!(pub fn math_modf @ref, math_modf_arm);
