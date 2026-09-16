use core::mem::size_of;

use crate::functions::writeu_8::writeu_8;

const K_DWARF_ALIGN: usize = size_of::<usize>();
const DW_CFA_NOP: u8 = 0;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn align_position(start: *mut u8, mut pos: *mut u8) -> *mut u8 {
  unsafe {
    let size = (pos as usize).wrapping_sub(start as usize);
    let pad = ((size + K_DWARF_ALIGN - 1) & !(K_DWARF_ALIGN - 1)) - size;

    for _ in 0..pad {
      pos = writeu_8(pos, DW_CFA_NOP);
    }

    pos
  }
}
