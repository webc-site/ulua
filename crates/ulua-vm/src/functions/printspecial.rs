use core::{
  ffi::{c_char, c_int},
  ptr::copy_nonoverlapping,
};
pub(crate) unsafe fn printspecial(buf: *mut c_char, sign: c_int, fraction: u64) -> *mut c_char {
  unsafe {
    if fraction == 0 {
      let src = b"-inf\0";
      let offset = (1 - sign) as usize;
      copy_nonoverlapping(src.as_ptr().add(offset), buf as *mut u8, 4);
      buf.add((3 + sign) as usize)
    } else {
      let src = b"nan\0";
      copy_nonoverlapping(src.as_ptr(), buf as *mut u8, 4);
      buf.add(3)
    }
  }
}
