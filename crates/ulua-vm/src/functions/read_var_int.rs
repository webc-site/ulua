use core::ffi::c_char;

use crate::functions::read_var_int_64::read_var_int_64;

/// 对齐 cpp `readVarInt = static_cast<unsigned int>(readVarInt64(...))`：
/// 截断 64 位读取结果。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn read_var_int(data: *const c_char, size: usize, offset: &mut usize) -> u32 {
  unsafe { read_var_int_64(data, size, offset) as u32 }
}
