use core::{
  ffi::c_char,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::functions::cstr_bytes;
/// # Safety
///
/// `buf` 必须指向至少 `bufsize` 字节、正确对齐且可写的缓冲区，且 `offset < bufsize`
/// （否则越界分支的 `bufsize - offset - 1` 无符号下溢）；`data` 必须指向 NUL 结尾的
/// C 字符串。写入区间为 `[offset, offset + copy)`，`copy` 经裁剪不超过
/// `bufsize - offset - 1`，故至少留一字节供调用方补终止符。
pub(crate) unsafe fn append(
  buf: *mut c_char,
  bufsize: usize,
  offset: usize,
  data: *const c_char,
) -> usize {
  // Safety: 契约保证 data 为 NUL 结尾字符串，from_ptr 扫描必然在缓冲内终止
  let size = unsafe { cstr_bytes(data as *mut c_char) }.len();
  let copy = if offset + size >= bufsize {
    bufsize - offset - 1
  } else {
    size
  };

  // Safety: 契约保证 offset<bufsize 且 copy<=bufsize-offset-1，dst 区间在 buf 可写界内
  let dst = unsafe { from_raw_parts_mut(buf.add(offset) as *mut u8, copy) };
  // Safety: src 区间取自上方 CStr 已扫描过的 data 前 copy 字节，必可读
  let src = unsafe { from_raw_parts(data as *const u8, copy) };
  dst.copy_from_slice(src);

  offset + copy
}
