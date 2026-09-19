use crate::{enums::size_x_64::SizeX64, records::register_x_64::RegisterX64};

pub const fn byte_reg(reg: RegisterX64) -> RegisterX64 {
  RegisterX64 {
    bits: SizeX64::Byte as u8 | (reg.bits & RegisterX64::INDEX_MASK),
  }
}
