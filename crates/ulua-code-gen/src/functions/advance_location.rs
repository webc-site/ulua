use crate::functions::write_unaligned::writeu_8;

const DW_CFA_ADVANCE_LOC1: u8 = 0x02;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn advance_location(mut pos: *mut u8, offset: u32) -> *mut u8 {
  // Safety: 契约保证 pos 为存活可写缓冲且至少余 2 字节；断言 offset<256 使
  // offset as u8 无截断值丢失，两次 writeu_8 各写 1 字节（u8 对齐平凡）落在预留区内。
  unsafe {
    assert!(offset < 256);
    pos = writeu_8(pos, DW_CFA_ADVANCE_LOC1);
    pos = writeu_8(pos, offset as u8);
    pos
  }
}
