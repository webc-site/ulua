use std::io;

/// 将字节串写到 stdout 并 flush；写失败静默忽略（与 C 版 `Writestring` 一致）。
pub(crate) fn writestring(buf: &[u8]) {
  use std::io::Write;

  let mut stdout = io::stdout().lock();
  let _ = stdout.write_all(buf);
  let _ = stdout.flush();
}
