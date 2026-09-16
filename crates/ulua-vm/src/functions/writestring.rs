use core::{ffi::c_char, slice::from_raw_parts};
use std::io::stdout;
pub(crate) unsafe fn writestring(s: *const c_char, l: usize) {
  unsafe {
    use std::io::Write;

    let buf = from_raw_parts(s as *const u8, l);
    let mut stdout = stdout().lock();
    let _ = stdout.write_all(buf);
    let _ = stdout.flush();
  }
}
