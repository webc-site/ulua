use core::ffi::c_char;
#[inline]
pub(crate) unsafe fn trimzero(mut end: *mut c_char) -> *mut c_char {
  unsafe {
    while *end.offset(-1) == b'0' as c_char {
      end = end.offset(-1);
    }

    end
  }
}
