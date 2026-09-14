use core::ffi::{c_char, c_int, c_void};

use crate::{macros::sizebuffer::sizebuffer, type_aliases::buffer::Buffer};

pub(crate) unsafe fn dumpbuffer(f: *mut c_void, b: *mut Buffer) {
  unsafe {
    let b = &*b;
    let fmt = "{\"type\":\"buffer\",\"cat\":%d,\"size\":%d}\0";

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(
      f,
      fmt.as_ptr() as *const c_char,
      b.memcat as c_int,
      sizebuffer(b.len as usize) as c_int,
    );
  }
}
