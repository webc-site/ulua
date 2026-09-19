use core::{ffi::c_char, slice::from_raw_parts};
use std::io::stdout;
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn writestring(s: *const c_char, l: usize) {
  unsafe {
    use std::io::Write;

    let buf = from_raw_parts(s as *const u8, l);
    let mut stdout = stdout().lock();
    let _ = stdout.write_all(buf);
    let _ = stdout.flush();
  }
}
