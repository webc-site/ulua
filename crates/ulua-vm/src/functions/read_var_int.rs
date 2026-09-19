use core::ffi::c_char;

use crate::functions::read_var_int_64::read_var_int_64;

/// 对齐 cpp `readVarInt = static_cast<unsigned int>(readVarInt64(...))`：
/// 截断 64 位读取结果。
///
/// 返回 `None` 表示 varint 的终止字节已跑出 blob（不可信长度），
/// 由 `loadsafe` 转成「损坏字节码」错误。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn read_var_int(
  data: *const c_char,
  size: usize,
  offset: &mut usize,
) -> Option<u32> {
  unsafe { read_var_int_64(data, size, offset).map(|value| value as u32) }
}
