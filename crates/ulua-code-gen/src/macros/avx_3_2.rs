use crate::{
  macros::{avx_b::avx_b, avx_r::avx_r, avx_x::avx_x},
  records::register_x_64::RegisterX64,
};

#[inline(always)]
pub const fn avx_3_2(r: RegisterX64, x: RegisterX64, b: RegisterX64, m: u8) -> u8 {
  avx_r(r) | avx_x(x) | avx_b(b) | m
}
