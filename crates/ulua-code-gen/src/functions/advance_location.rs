use crate::functions::writeu_8::writeu_8;

const DW_CFA_ADVANCE_LOC1: u8 = 0x02;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn advance_location(mut pos: *mut u8, offset: u32) -> *mut u8 {
  unsafe {
    assert!(offset < 256);
    pos = writeu_8(pos, DW_CFA_ADVANCE_LOC1);
    pos = writeu_8(pos, offset as u8);
    pos
  }
}
