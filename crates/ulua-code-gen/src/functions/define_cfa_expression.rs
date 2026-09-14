use crate::functions::{
  writeu_8::writeu_8 as writeu8, writeuleb_128::writeuleb_128 as writeuleb128,
};

const DW_CFA_DEF_CFA: u8 = 0x0c;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn define_cfa_expression(mut pos: *mut u8, dw_reg: i32, stack_offset: u32) -> *mut u8 {
  unsafe {
    pos = writeu8(pos, DW_CFA_DEF_CFA);
    pos = writeuleb128(pos, dw_reg as u64);
    pos = writeuleb128(pos, stack_offset as u64);
    pos
  }
}
