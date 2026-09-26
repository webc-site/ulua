use crate::{enums::size_x_64::SizeX64, records::register_x_64::RegisterX64};

pub const fn rex_force(reg: RegisterX64) -> u8 {
  // 注：RegisterX64::size() 非 const，但可以从 bits 计算。
  // 依据 RegisterX64 的内部结构（bits）提取 size。
  let size_bits = reg.bits & RegisterX64::SIZE_MASK;

  // SizeX64::Byte 是 enum 变体。impl 非 const 时 const if 里不能用 ==，
  // 且 size() 非 const，故拿原始 bits 与 SizeX64::Byte 的判别值比较。
  if size_bits == SizeX64::Byte as u8 && reg.index() >= 4 {
    0x40
  } else {
    0x00
  }
}
