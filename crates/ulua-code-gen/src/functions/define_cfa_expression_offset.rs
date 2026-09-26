use crate::functions::{write_unaligned::writeu_8, writeuleb_128::writeuleb_128};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn define_cfa_expression_offset(mut pos: *mut u8, stack_offset: u32) -> *mut u8 {
  // Safety: 契约保证 pos 为存活可写缓冲；先写 1 字节 opcode，再由 writeuleb_128 追加
  // stack_offset 的 LEB128 编码，均为 u8 写（对齐平凡）且落在调用方预留空间内。
  unsafe {
    const DW_CFA_DEF_CFA_OFFSET: u8 = 0x0e;
    pos = writeu_8(pos, DW_CFA_DEF_CFA_OFFSET);
    pos = writeuleb_128(pos, stack_offset as u64);
    pos
  }
}
