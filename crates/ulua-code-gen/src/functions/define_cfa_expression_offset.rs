use crate::functions::{writeu_8::writeu_8, writeuleb_128::writeuleb_128};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn define_cfa_expression_offset(mut pos: *mut u8, stack_offset: u32) -> *mut u8 {
  unsafe {
    const DW_CFA_DEF_CFA_OFFSET: u8 = 0x0e;
    pos = writeu_8(pos, DW_CFA_DEF_CFA_OFFSET);
    pos = writeuleb_128(pos, stack_offset as u64);
    pos
  }
}
