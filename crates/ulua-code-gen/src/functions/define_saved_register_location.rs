use crate::functions::{
  write_unaligned::writeu_8 as writeu8, writeuleb_128::writeuleb_128 as writeuleb128,
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
  // 纯整数断言（无裸指针操作，移出 unsafe）：确保 stack_offset 为对齐因子整数倍、除法不丢关键位。
  assert!(
    stack_offset.is_multiple_of(K_DATA_ALIGN_FACTOR),
    "stack offsets have to be measured in K_DATA_ALIGN_FACTOR units"
  );

  // Safety: 契约保证 pos 为存活可写缓冲且预留本条 CFA 指令最长编码；各分支写 1 字节 opcode
  // + 可选 LEB128（u8 对齐平凡），指针前进量不超过预留空间。下文简记「依契约」。
  if dw_reg <= 0x3f {
    pos = unsafe { writeu8(pos, DW_CFA_OFFSET + dw_reg as u8) };
  } else {
    pos = unsafe { writeu8(pos, DW_CFA_OFFSET_EXTENDED) };
    pos = unsafe { writeuleb128(pos, dw_reg as u64) };
  }

  // Safety: 依契约；offset 操作数以 LEB128 编码（整除关键位已由上方断言保证）。
  pos = unsafe { writeuleb128(pos, (stack_offset / K_DATA_ALIGN_FACTOR) as u64) };

  pos
}
