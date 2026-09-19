use core::{ffi::c_char, slice::from_raw_parts};

use ulua_common::functions::read_var_int_64::read_var_int_64 as common_read_var_int_64;

use crate::functions::read::read;

/// LEB128 的终止字节（最高位为 0）是否存在于 `[offset, size)` 内。
///
/// 只做「流是否完整」的判定，不解码：解码唯一实现仍在 ulua-common
/// （cpp 也只有一份 `readVarInt64`）。终止字节存在时，common 的逐字节
/// `read::<u8>` 必然落在缓冲内，越界 panic 通道因此关闭。
fn varint_terminated(data: *const c_char, size: usize, mut offset: usize) -> bool {
  while let Some(byte) = unsafe { read::<u8>(data, size, &mut offset) } {
    if byte & 0x80 == 0 {
      return true;
    }
  }

  false
}

/// cpp `readVarInt64`：不可信流的终止字节不在 blob 内时返回 `None`，
/// 由 `loadsafe` 转成「损坏字节码」错误。
///
/// # Safety
/// 指针参数必须有效且指向存活对象（长度 size 的字节缓冲）；`size > 0` 时
/// `data` 不得为 null。
pub(crate) unsafe fn read_var_int_64(
  data: *const c_char,
  size: usize,
  offset: &mut usize,
) -> Option<u64> {
  if !varint_terminated(data, size, *offset) {
    return None;
  }

  // 走到这里说明至少有一个字节可读，故 size > 0、data 非空
  let bytes = unsafe { from_raw_parts(data as *const u8, size) };

  Some(common_read_var_int_64(bytes, offset))
}
