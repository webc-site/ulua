//! `luaO_str2d`（cpp lobject.h 以 `LUAI_FUNC` 导出）的接受/拒绝边界契约：
//! 十进制/十六进制/inf-nan 拼写与尾随垃圾的收口。

use ulua_vm::functions::lua_o_str_2_d::lua_o_str_2_d;

/// 对照 cpp luaO_str2d（lobject.cpp:86）的接受/拒绝边界
fn str2d(s: &[u8]) -> Option<f64> {
  lua_o_str_2_d(s)
}

#[test]
fn accepts_decimal_with_trailing_space() {
  assert_eq!(str2d(b"42"), Some(42.0));
  assert_eq!(str2d(b" 3.5 "), Some(3.5));
  assert_eq!(str2d(b"-0.0"), Some(0.0)); // 值比较下 -0.0 == 0.0
  assert!(str2d(b"-0.0").unwrap().is_sign_negative());
  assert_eq!(str2d(b"1e3"), Some(1000.0));
}

#[test]
fn accepts_hex_when_strtod_stops_at_x() {
  // strtod 只吃 "0"，hex 路径由 strtoull 接管（与无 hex-float 的 C 行为一致）
  assert_eq!(str2d(b"0x1f"), Some(31.0));
  assert_eq!(str2d(b"0X10"), Some(16.0));
  assert_eq!(str2d(b"0"), Some(0.0));
  assert_eq!(str2d(b"0x"), None); // strtoull 停在 x，trailing 非法
}

#[test]
fn rejects_non_numeric_and_trailing_junk() {
  assert_eq!(str2d(b""), None);
  assert_eq!(str2d(b"abc"), None);
  assert_eq!(str2d(b"12abc"), None); // 尾随非空白 → 拒绝
  assert_eq!(str2d(b".e5"), None); // 无数字的伪浮点
  assert_eq!(str2d(b"  "), None);
}

#[test]
fn accepts_c_inf_nan_spellings() {
  assert_eq!(str2d(b"inf"), Some(f64::INFINITY));
  assert_eq!(str2d(b"-infinity"), Some(f64::NEG_INFINITY));
  assert!(str2d(b"nan").unwrap().is_nan());
}
