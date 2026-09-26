use core::ffi::{c_char, c_void};

use crate::functions::{c_file_write_bytes, c_slice, safejson::safejson};

/// # Safety
/// `f` 须为可写合法 `FILE*`；`data` 允许为空当且仅当 `len==0`，否则须指向至少 `len` 字节可读区间
/// （`c_slice(data,len)` 逐字节读入栈缓冲 `buf[256]`，每满 256 刷写一次，故读区严格以 `len` 为界不越界）；不分配、不抛错。
/// cpp VM/src/lgcdebug.cpp:347
pub(crate) unsafe fn dumpstringdata(f: *mut c_void, data: *const c_char, len: usize) {
  unsafe {
    let slice = c_slice(data.cast::<u8>(), len);
    // 栈块批量写出，避免逐字节 C 调用
    let mut buf = [0u8; 256];
    let mut n = 0;
    for &ch in slice {
      buf[n] = if safejson(ch as c_char) { ch } else { b'?' };
      n += 1;
      if n == buf.len() {
        c_file_write_bytes(f, &buf);
        n = 0;
      }
    }
    if n != 0 {
      c_file_write_bytes(f, &buf[..n]);
    }
  }
}
