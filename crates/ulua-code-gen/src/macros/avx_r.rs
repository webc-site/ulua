use crate::records::register_x_64::RegisterX64;

pub const fn avx_r(reg: RegisterX64) -> u8 {
  (!reg.index() & 0x8) << 4
}
