use crate::functions::{
  writeu_8::writeu_8 as writeu8, writeuleb_128::writeuleb_128 as writeuleb128,
};

const DW_CFA_OFFSET: u8 = 0x80;
const DW_CFA_OFFSET_EXTENDED: u8 = 0x05;
const K_DATA_ALIGN_FACTOR: u32 = 8;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn define_saved_register_location(
  mut pos: *mut u8,
  dw_reg: i32,
  stack_offset: u32,
) -> *mut u8 {
  unsafe {
    assert!(
      stack_offset.is_multiple_of(K_DATA_ALIGN_FACTOR),
      "stack offsets have to be measured in K_DATA_ALIGN_FACTOR units"
    );

    if dw_reg <= 0x3f {
      pos = writeu8(pos, DW_CFA_OFFSET + dw_reg as u8);
    } else {
      pos = writeu8(pos, DW_CFA_OFFSET_EXTENDED);
      pos = writeuleb128(pos, dw_reg as u64);
    }

    pos = writeuleb128(pos, (stack_offset / K_DATA_ALIGN_FACTOR) as u64);
    pos
  }
}
