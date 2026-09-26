//! `mask64` 收紧版契约（对照 cpp lintlib.cpp `#define mask64(w)
//! (0xFFFFFFFFFFFFFFFFULL >> (64 - (w)))`）：`integer.extract` /
//! `integer.replace` 共用，越界宽度不移位而是直接给 0 / 全 1。

use ulua_vm::macros::mask_64::mask64;

/// 调用点合法域（cpp `luaL_argcheck(L, 0 < w, ...)` 且 `f + w <= 64`）内，
/// 收紧版必须与上游宏 `0xFFFF... >> (64 - w)` 逐位一致，否则即为语义漂移。
#[test]
fn matches_cpp_macro_over_call_domain() {
  for w in 1..=64i32 {
    // w == 64 时移位量为 0，w >= 1 时移位量落在 0..=63，均为合法移位
    let cpp = u64::MAX >> (64 - w as u32);
    assert_eq!(mask64(w), cpp, "w={w} 与上游 mask64 宏不一致");
  }
}

/// 边界：w == 0 是上游宏的 UB 输入（`>> 64`），收紧版给 0 位掩码。
#[test]
fn zero_and_negative_width_yields_empty_mask() {
  assert_eq!(mask64(0), 0);
  assert_eq!(mask64(-1), 0);
  assert_eq!(mask64(i32::MIN), 0);
}

/// 边界：w == 64 取全 1（与上游 `>> 0` 同值），更宽的输入饱和为全 1 而不是
/// 移位溢出 panic。
#[test]
fn full_and_over_width_yields_all_ones() {
  assert_eq!(mask64(64), u64::MAX);
  assert_eq!(mask64(65), u64::MAX);
  assert_eq!(mask64(i32::MAX), u64::MAX);
}
