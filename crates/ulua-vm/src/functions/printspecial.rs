use core::ffi::c_char;

/// cpp `lnumprint.cpp:printspecial`：向 `buf` 写 `-inf`/`inf`/`nan`。
///
/// 连同结尾 NUL 一并写入 4 字节（与原实现一致，调用方缓冲有富余），
/// 返回写出文本的结尾位置（不含 NUL）。
pub(crate) fn printspecial(buf: &mut [c_char], sign: i32, fraction: u64) -> usize {
  let (src, offset, end): (&[u8], usize, usize) = if fraction == 0 {
    (b"-inf\0", (1 - sign) as usize, (3 + sign) as usize)
  } else {
    (b"nan\0", 0, 3)
  };
  for (dst, &c) in buf.iter_mut().zip(&src[offset..]) {
    *dst = c as c_char;
  }
  end
}
