use crate::{macros::avx_w::avx_w, records::register_x_64::RegisterX64};

#[inline(always)]
pub const fn avx_3_3(w: bool, v: RegisterX64, l: u8, p: u8) -> u8 {
  avx_w(w) | ((!(v.index() & 0xf) & 0xf) << 3) | (l << 2) | p
}
