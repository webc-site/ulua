use crate::macros::rol::rol;

/// cpp `lstring.cpp` 的 `mix(u, v, w)` 宏：对 `(a, b, h)` 三元组做一次 ARX 混合。
///
/// C++ 版经宏就地改写三个局部变量，Rust 版以元组把新值返回（无出参）。
#[inline(always)]
pub const fn mix(u: u32, v: u32, w: u32, mut a: u32, mut b: u32, mut h: u32) -> (u32, u32, u32) {
  a ^= h;
  a = a.wrapping_sub(rol(h, u));
  b ^= a;
  b = b.wrapping_sub(rol(a, v));
  h ^= b;
  h = h.wrapping_sub(rol(b, w));
  (a, b, h)
}
