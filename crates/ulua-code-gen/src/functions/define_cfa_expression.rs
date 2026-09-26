use crate::{
  functions::{write_unaligned::writeu_8 as writeu8, writeuleb_128::writeuleb_128 as writeuleb128},
  macros::dwarf_reg::DW_CFA_DEF_CFA,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn define_cfa_expression(mut pos: *mut u8, dw_reg: i32, stack_offset: u32) -> *mut u8 {
  // Safety: 契约保证 pos 为存活可写缓冲且预留足够 CFA 编码空间；writeu8/writeuleb128
  // 逐字节写 opcode 与 LEB128 操作数（u8 对齐平凡），指针只前进不越出调用方预留区。
  unsafe {
    pos = writeu8(pos, DW_CFA_DEF_CFA);
    pos = writeuleb128(pos, dw_reg as u64);
    pos = writeuleb128(pos, stack_offset as u64);
    pos
  }
}
