use core::{
  ffi::c_char,
  fmt::{self, Write},
  slice::from_raw_parts_mut,
  str::from_utf8,
};

use crate::functions::cstr_bytes;

/// 可写入 `&mut [c_char]` 的 fmt::Write 适配器，自动 NUL 结尾截断。
///
/// 用于替代 enum/dump 系列函数中的 `snprintf`，全平台一致、无 unsafe FFI。
struct CCharBuf<'a> {
  buf: &'a mut [u8],
  pos: usize,
}

impl<'a> CCharBuf<'a> {
  fn new(buf: &'a mut [c_char]) -> Self {
    // c_char 和 u8 大小相同，安全转换
    // Safety: c_char 与 u8 布局一致，from_raw_parts_mut 仅按同一借用的界重建切片，长度不变
    let buf = unsafe { from_raw_parts_mut(buf.as_mut_ptr().cast::<u8>(), buf.len()) };
    Self { buf, pos: 0 }
  }
}

impl Write for CCharBuf<'_> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let cap = self.buf.len().saturating_sub(1); // 留 1 字节给 NUL
    let avail = cap.saturating_sub(self.pos);
    let n = s.len().min(avail);
    self.buf[self.pos..self.pos + n].copy_from_slice(&s.as_bytes()[..n]);
    self.pos += n;
    self.buf[self.pos] = 0; // NUL 结尾
    Ok(())
  }
}

/// 将 `fmt::Arguments` 写入 `c_char` 缓冲区，NUL 结尾，截断安全。
///
/// 用法：`fmt_cstr_buf(&mut buf, format_args!("thread at {}:{} {}", name, line, src));`
pub(crate) fn fmt_cstr_buf(buf: &mut [c_char], args: fmt::Arguments<'_>) {
  if buf.is_empty() {
    return;
  }
  buf[0] = 0; // 先置空
  let mut w = CCharBuf::new(buf);
  let _ = w.write_fmt(args);
}

/// 从 `*const c_char` 安全读取 C 字符串为 `&str`。
///
/// # Safety
/// 指针必须指向以 NUL 结尾的合法缓冲区。
pub(crate) unsafe fn cstr_display(p: *const c_char) -> &'static str {
  unsafe { from_utf8(cstr_bytes(p)).unwrap_or("?") }
}
