use core::ffi::{c_char, c_void};

use crate::functions::{c_file_write_bytes, c_slice, safejson::safejson};

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
