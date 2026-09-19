use core::ffi::c_char;

use crate::functions::read::read;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn read_var_int_64(data: *const c_char, size: usize, offset: &mut usize) -> u64 {
  unsafe {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;

    loop {
      let byte: u8 = read(data, size, offset);
      // 损坏的流可能连续出现续字节使 shift ≥ 64（u64 的 varint 最多 10 字节）；
      // 溢出位按丢弃处理，避免 debug 下移位溢出 panic。
      if shift < 64 {
        result |= (u64::from(byte & 127)) << shift;
      }
      shift += 7;
      if byte & 128 == 0 {
        break;
      }
    }

    result
  }
}
