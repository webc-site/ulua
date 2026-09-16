use core::{ffi::c_char, ptr::copy_nonoverlapping};

/// lua_Integer → 十进制字符串热路径。
///
/// itoa 的十进制输出与 `core::fmt` 完全一致，直接回填 C 缓冲区并保持 NUL 结尾。
///
/// # Safety
///
/// `buf` 须可写至少 `LUAI_MAXINT2STR` 字节。
pub(crate) unsafe fn luai_int2str(buf: *mut c_char, l: i64) -> *mut c_char {
  unsafe {
    let mut b = itoa::Buffer::new();
    let s = b.format(l);
    copy_nonoverlapping(s.as_ptr(), buf.cast::<u8>(), s.len());
    *buf.add(s.len()) = 0;
    buf.add(s.len())
  }
}
